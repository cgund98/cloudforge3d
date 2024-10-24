use core::time;
use std::{path::PathBuf, sync::Arc, thread};

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
                log::warn!("Encountered unexpected error when handling file upload for job (job_id={job_id}): {f}");
                self.queue.enqueue(input);
                self.emit_failed_progress(job_id, file_path);

                // Sleep to prevent infinite loops
                thread::sleep(time::Duration::from_secs(3));
            });
        }
    }

    // Handle a single file upload
    async fn handle_input(&self, client_manager: &mut S3ClientManager, input: FileUploadProcessorInput) -> Result<(), AppError> {
        let job_id = input.job_id;
        let job_query = self.job_repo.get_by_id(&job_id)?;

        if job_query.is_none() {
            log::info!("Job (job_id={job_id}) does not exist. Skipping upload.");
            return Ok(())
        }

        // Fetch client
        let s3_client = client_manager.get_client().await?;

        // Upload blend file
        let blend_path = PathBuf::from(input.file_path);
        let blend_key = "render.blend".to_string();

        s3::job_file::upload_job_file(s3_client, job_id, blend_path, blend_key, |p| self.emit_progress(p)).await?;

        Ok(())
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