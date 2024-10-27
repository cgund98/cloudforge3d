use core::time;
use scc::HashSet;
use std::{sync::Arc, thread};

use tauri::{AppHandle, Manager};

use crate::{
    biz::thumbnail::generate_job_thumbnail_gif,
    errors::AppError,
    infra::db::{
        decorator::{with_conn, with_transaction},
        pool::PoolType,
        render_job,
    },
    interface::events::emit_job_status_update_event,
};

const MAX_RETRIES: i32 = 5;

// Type alias for a deadqueue Queue
type Queue = deadqueue::unlimited::Queue<ThumbnailGeneratorInput>;

// A shared queue wrapper that will be used for communicating with other processes
pub struct ThumbnailGeneratorQueue {
    queue: Queue,
    queued_ids: HashSet<String>,
}

// A simple thread-safe queue with deduplication
impl ThumbnailGeneratorQueue {
    pub fn new() -> ThumbnailGeneratorQueue {
        ThumbnailGeneratorQueue {
            queue: Queue::new(),
            queued_ids: HashSet::new(),
        }
    }
    pub fn enqueue(&self, job_id: String) {
        let already_queued = self.queued_ids.contains(&job_id);

        if !already_queued {
            let _ = self
                .queued_ids
                .insert(job_id.clone())
                .inspect_err(|e| log::warn!("Unable to add key to thumbnail set: {e}"));
            self.queue.push(ThumbnailGeneratorInput {
                job_id,
                num_retries: 0,
            });
        }
    }

    fn enqueue_with_retries(&self, job_id: String, num_retries: i32) {
        self.queue.push(ThumbnailGeneratorInput {
            job_id,
            num_retries,
        });
    }

    async fn pop(&self) -> ThumbnailGeneratorInput {
        let res = self.queue.pop().await;
        self.queued_ids.remove(&res.job_id);
        res
    }
}

// ThumbnailGeneratorProcesser will run in a separate process and upload job files to S3.
pub struct ThumbnailGenerator {
    queue: Arc<ThumbnailGeneratorQueue>,
    pool: Arc<PoolType>,
    handle: Arc<AppHandle>,
}

// Data object used for persisting in queue.
#[derive(Clone)]
struct ThumbnailGeneratorInput {
    job_id: String,
    num_retries: i32,
}

const FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);

impl ThumbnailGenerator {
    pub fn new(
        pool: Arc<PoolType>,
        queue: Arc<ThumbnailGeneratorQueue>,
        handle: Arc<AppHandle>,
    ) -> ThumbnailGenerator {
        ThumbnailGenerator {
            queue,
            pool,
            handle,
        }
    }

    // Start the forever-running task that will attempt to generate thumbnails for jobs
    pub async fn start_task(&self) {
        loop {
            let input = self.queue.pop().await;
            let job_id = input.job_id.clone();

            log::info!("Generating thumbnail for (job_id={job_id})...");
            let _res = self.handle_input(&input).await
            .inspect(|_| log::info!("Generated thumbnail for job (job_id={job_id})."))
                .unwrap_or_else(|f| {
                log::error!("Encountered unexpected error when generating thumbnail for job (job_id={job_id}): {f}");

                // Re-queue if max retries not hit
                let num_retries = input.num_retries;
                if num_retries < MAX_RETRIES {
                    self.queue.enqueue_with_retries(job_id, num_retries + 1);
                }

                // Sleep to prevent infinite loops
                thread::sleep(FAILURE_BACKOFF);
            });
        }
    }

    async fn handle_input(&self, input: &ThumbnailGeneratorInput) -> Result<(), AppError> {
        let job_id = input.job_id.clone();

        // Fetch job
        let job_opt = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &job_id)
        })?;
        if job_opt.is_none() {
            return Err(AppError::NotFound(format!("id = {job_id}")));
        }
        let job = job_opt.unwrap();

        // Generate thumbnail path
        let data_dir = self.handle.path().app_data_dir().unwrap();
        let thumbnails_path = data_dir.join("thumbnails/jobs").join(&job_id);

        let did_generate_thumbnail = generate_job_thumbnail_gif(thumbnails_path, job.frame_rate)
            .await
            .unwrap_or(false);

        if did_generate_thumbnail {
            with_transaction(&self.pool, |tx| {
                render_job::repo::set_has_preview(tx, &job_id, true)
            })?;
            emit_job_status_update_event(&self.handle, &job.id)?;
        }

        Ok(())
    }
}
