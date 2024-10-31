use core::time;
use std::{fs, path::PathBuf, sync::Arc, thread};

use tauri::AppHandle;

use crate::{
    errors::AppError,
    infra::{
        batch::{self, client_manager::BatchClientManager},
        db::{
            decorator::{with_conn, with_transaction},
            pool::PoolType,
            render_job::{
                entity::{JobStatus, RenderJob},
                repo,
            },
            render_task::{
                self,
                entity::{RenderTask, TaskStatus},
            },
        },
        s3::{self, client_manager::S3ClientManager, job_file::delete_job_files},
    },
    interface::events::{emit_job_file_upload_progress_event, emit_job_status_update_event},
    spec::proto::v1::JobFileUploadProgressEvent,
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
            queue: Queue::new(),
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
pub struct FileUploadProcessor {
    queue: Arc<FileUploadQueue>,
    pool: Arc<PoolType>,
    handle: Arc<AppHandle>,
    s3_manager: Arc<S3ClientManager>,
}

// Data object used for persisting in queue.
#[derive(Clone)]
pub struct FileUploadProcessorInput {
    pub job_id: String,
    pub file_path: String,
    pub file_name: String,
    pub ocio_config_path: Option<String>,
    pub memory_mib: u32,
    pub vcpus: u8,
}

// Helper struct used for passing data from file scans
struct FileScanResult {
    target_key: String,
    source_path: PathBuf,
}

const FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);

impl FileUploadProcessor {
    pub fn new(
        pool: Arc<PoolType>,
        queue: Arc<FileUploadQueue>,
        handle: Arc<AppHandle>,
        s3_manager: Arc<S3ClientManager>,
    ) -> FileUploadProcessor {
        FileUploadProcessor {
            queue,
            pool,
            handle,
            s3_manager,
        }
    }

    // Start the process that will continuously read from the queue
    pub async fn start_task(&self, mut batch_manager: BatchClientManager) {
        loop {
            let input = self.queue.pop().await;
            let job_id = input.job_id.clone();
            let file_path = input.file_path.clone();

            log::info!("Handling file upload for job (job_id={job_id})...");

            let result = self.handle_input(&mut batch_manager, input.clone()).await;

            // We don't want to exit in case of failure. Re-queue the upload and try again.
            result
                .inspect(|_| log::info!("Handled file upload for job (job_id={job_id})."))
                .unwrap_or_else(|f| {
                log::error!("Encountered unexpected error when handling file upload for job (job_id={job_id}): {f}");
                self.queue.enqueue(input);
                self.emit_failed_progress(job_id, file_path);

                // Sleep to prevent infinite loops
                thread::sleep(FAILURE_BACKOFF);
            });
        }
    }

    // Handle a single file upload
    async fn handle_input(
        &self,
        batch_manager: &mut BatchClientManager,
        input: FileUploadProcessorInput,
    ) -> Result<(), AppError> {
        let job_id = input.job_id.clone();
        let mut job_query = with_conn(&self.pool, |conn| repo::get_by_id(conn, &job_id))?;

        if job_query.is_none() {
            log::info!("Job (job_id={job_id}) does not exist. Skipping upload.");
            return Ok(());
        }

        // Fetch s3 client
        let s3_client = self.s3_manager.get_client().await?;

        // Upload OCIO files
        let ocio_paths = FileUploadProcessor::scan_for_ocio_files(&input)?;
        for entry in ocio_paths {
            let target_key = entry.target_key;
            log::info!("Uploading OCIO file ({target_key})...");
            s3::job_file::upload_job_file(
                &s3_client,
                &job_id,
                entry.source_path,
                &target_key,
                |p| self.emit_progress(p),
            )
            .await?;
        }

        // Upload blend file
        let blend_path = PathBuf::from(input.file_path);
        let blend_key = "render.blend".to_string();
        let blend_name = blend_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        log::info!("Uploading blend file ({blend_name})");
        s3::job_file::upload_job_file(&s3_client, &job_id, blend_path.clone(), &blend_key, |p| {
            self.emit_progress(p)
        })
        .await?;

        // Make sure the job wasn't deleted while uploads occured
        job_query = with_conn(&self.pool, |conn| repo::get_by_id(conn, &job_id))?;
        if job_query.is_none() {
            log::info!("Job (job_id={job_id}) deleted while uploading.");
            delete_job_files(&s3_client, &job_id).await?;
            return Ok(());
        }
        let mut job = job_query.unwrap();

        // Update job status
        job.status = JobStatus::Pending;
        let now = chrono::offset::Utc::now();
        job.queued_at = Some(now);

        // Fetch batch client
        let batch_client = batch_manager.get_client().await?;

        // Create dependent resources
        log::info!("Persisting job and creating tasks...");
        let tasks = with_transaction(&self.pool, |tx| {
            repo::save(&tx, &job)?;
            self.create_tasks(&tx, &job)
        })?;

        emit_job_status_update_event(&self.handle, &job.id)?;

        // Remove tempfile
        fs::remove_file(blend_path)?;

        // Submit batch jobs
        log::info!("Submitting jobs...");
        self.submit_tasks(batch_client, input.vcpus, input.memory_mib, tasks).await?;

        Ok(())
    }

    fn scan_for_ocio_files(
        input: &FileUploadProcessorInput,
    ) -> Result<Vec<FileScanResult>, AppError> {
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
                let relative_path = path.strip_prefix(base_path.to_path_buf()).map_err(|_e| {
                    AppError::FileReadError("Could not create relative path.".to_string())
                })?;
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

    async fn submit_tasks(
        &self,
        batch_client: &aws_sdk_batch::Client,
        vcpus: u8,
        memory_mib: u32,
        tasks: Vec<RenderTask>,
    ) -> Result<(), AppError> {
        for task in tasks {
            batch::render_task::submit_job(batch_client, vcpus, memory_mib, &task).await?;
        }

        Ok(())
    }

    fn create_tasks(
        &self,
        tx: &rusqlite::Transaction<'_>,
        job: &RenderJob,
    ) -> Result<Vec<RenderTask>, AppError> {
        let frame_start = job.frame_start;
        let frame_end = frame_start + job.frame_count;

        let mut tasks = Vec::new();

        for frame_number in frame_start..frame_end {
            let task = RenderTask {
                id: job.id.clone() + "-frame-" + &frame_number.to_string(),
                job_id: job.id.clone(),
                frame_number,
                created_at: Some(chrono::offset::Utc::now()),
                started_at: None,
                queued_at: Some(chrono::offset::Utc::now()),
                completed_at: None,
                status: TaskStatus::Pending,
                retry_count: 0,
            };

            render_task::repo::save(tx, &task)?;

            tasks.push(task);
        }

        Ok(tasks)
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

        emit_job_file_upload_progress_event(self.handle.as_ref(), &event)
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

        let _ =
            emit_job_file_upload_progress_event(self.handle.as_ref(), &event).inspect_err(|e| {
                log::error!("Error while publishing upload fail event: {e}");
            });
    }
}
