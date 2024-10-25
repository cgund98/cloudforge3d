use core::time;
use std::{path::{Path, PathBuf}, sync::Arc, thread};

use tauri::AppHandle;

use crate::{
    errors::AppError,
    infra::{db, s3::{self, client_manager::S3ClientManager}}, interface::events::emit_job_file_upload_progress_event, spec::proto::v1::JobFileUploadProgressEvent
};

// Type alias for a deadqueue Queue
pub type Queue = deadqueue::unlimited::Queue<FileUploadProcessorInput>;

// A shared queue wrapper that will be used for communicating with other processes
pub struct FileUploadQueue {
    queue: Queue,
}

impl FileUploadQueue {
    pub fn new() -> FileUploadQueue {
        FileUploadQueue {
            queue: Queue::new()
        }
    }
    pub fn enqueue(&self, input: FileUploadProcessorInput) {
        self.queue.push(input);
    }

    async fn pop(&self) -> FileUploadProcessorInput {
        self.queue.pop().await
    }
}

// FileUploadProcesser will run in a separate process and upload job files to S3.
pub struct FileUploadProcessor{
    queue: Arc<FileUploadQueue>,
    job_repo: Arc<db::render_job::repo::Repo>,
    handle: Arc<AppHandle>,
}


// Data object used for persisting in queue.
#[derive(Clone)]
pub struct FileUploadProcessorInput {
    pub job_id: String,
    pub file_path: String,
    pub ocio_config_path: Option<String>,
}

// Helper struct used for passing data from file scans
struct FileScanResult {
    target_key: String,
    source_path: PathBuf,
}

const FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);

impl FileUploadProcessor{
    pub fn new( job_repo: Arc<db::render_job::repo::Repo>, queue: Arc<FileUploadQueue>, handle: Arc<AppHandle>) -> FileUploadProcessor {
        FileUploadProcessor {
            queue,
            job_repo,
            handle,
        }
    }

    // Start the process that will continuously read from the queue
    pub async fn start_task(&self, mut client_manager: S3ClientManager) {
        loop {
            let input = self.queue.pop().await;
            let job_id = input.job_id.clone();
            let file_path = input.file_path.clone();

            log::info!("Handling file upload for job (job_id={job_id})...");

            let result = self.handle_input(&mut client_manager, input.clone()).await;

            // We don't want to exit in case of failure. Re-queue the upload and try again.
            result.unwrap_or_else(|f| {
                log::error!("Encountered unexpected error when handling file upload for job (job_id={job_id}): {f}");
                self.queue.enqueue(input);
                self.emit_failed_progress(job_id, file_path);

                // Sleep to prevent infinite loops
                thread::sleep(FAILURE_BACKOFF);
            });
        }
    }

    // Handle a single file upload
    async fn handle_input(&self, client_manager: &mut S3ClientManager, input: FileUploadProcessorInput) -> Result<(), AppError> {
        let job_id = input.job_id.clone();
        let _job_query = self.job_repo.get_by_id(&job_id)?;

        // if job_query.is_none() {
        //     log::info!("Job (job_id={job_id}) does not exist. Skipping upload.");
        //     return Ok(())
        // }

        // Fetch client
        let s3_client = client_manager.get_client().await?;

        // Upload OCIO files
        let ocio_paths = FileUploadProcessor::scan_for_ocio_files(&input)?;
        for entry in ocio_paths {
            let target_key = entry.target_key;
            log::info!("Uploading OCIO file ({target_key})...");
            s3::job_file::upload_job_file(s3_client, job_id.clone(), entry.source_path, target_key, |p| self.emit_progress(p)).await?;
        }

        // Upload blend file
        let blend_path = PathBuf::from(input.file_path);
        let blend_key = "render.blend".to_string();
        let blend_name = blend_path.file_name().unwrap().to_string_lossy().into_owned();

        log::info!("Uploading blend file ({blend_name})");
        s3::job_file::upload_job_file(s3_client, job_id, blend_path, blend_key, |p| self.emit_progress(p)).await?;

        Ok(())
    }

    fn scan_for_ocio_files(input: &FileUploadProcessorInput) -> Result<Vec<FileScanResult>, AppError> {
        let mut paths = Vec::new();
        if input.ocio_config_path.is_none() {
            return Ok(paths);
        }

        let ocio_config_path = PathBuf::from(input.ocio_config_path.clone().unwrap());
        paths.push(FileScanResult {
            target_key: "ocio/config.ocio".to_string(),
            source_path: ocio_config_path.clone(),
        });

        // Scan for .cube files
        let ocio_base_path = ocio_config_path.parent();
        if let Some(base_path) = ocio_base_path {
            for entry in std::fs::read_dir(base_path)? {
                let entry = entry?;
                let path = entry.path();

                // Determine relative path
                let relative_path = path
                    .strip_prefix(base_path.to_path_buf())
                    .map_err(|e| AppError::FileReadError("Could not create relative path.".to_string()))?;
                let relative_path_str = relative_path.to_string_lossy().into_owned();
        
                // Check if the entry is a file and has a ".cube" extension
                if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("cube") {
                    paths.push(FileScanResult {
                        target_key: "ocio/".to_string() + &relative_path_str,
                        source_path: path,
                    });
                }
            }
        }

        Ok(paths)
    }

    /** Event emission wrappers */

    fn emit_progress(&self, progress: s3::job_file::Progress) -> Result<(), AppError> {
        let is_done = progress.uploaded == progress.total;

        let event = JobFileUploadProgressEvent {
            job_id: progress.job_id,
            file_name: progress.file_name,
            current_chunk: progress.uploaded,
            total_chunks: progress.total,
            has_error: false,
            is_done,
            description: Some("Upload in progress".to_string()),
        };

        emit_job_file_upload_progress_event(self.handle.as_ref(), event)
    }

    fn emit_failed_progress(&self, job_id: String, file_name: String) {
        let event = JobFileUploadProgressEvent {
            job_id,
            file_name,
            current_chunk: 0,
            total_chunks: 0,
            has_error: true,
            is_done: true,
            description: Some("Upload failed".to_string()),
        };

        let _ = emit_job_file_upload_progress_event(self.handle.as_ref(), event).inspect_err(|e| {
            log::error!("Error while publishing upload fail event: {e}");
        });
    }
}