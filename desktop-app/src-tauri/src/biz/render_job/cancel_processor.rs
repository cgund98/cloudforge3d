use core::time;
use std::{sync::Arc, thread};

use scc::HashSet;
use tauri::AppHandle;

use crate::{
    errors::AppError,
    infra::{
        batch::{self, client_manager::BatchClientManager},
        db::{
            decorator::{with_conn, with_transaction},
            pool::PoolType,
            render_job::{self}, render_task::{self, entity::TaskStatus},
        },
    },
    interface::events::emit_job_status_update_event,
};

const MAX_RETRIES: i32 = 5;

// Type alias for a deadqueue Queue
type Queue = deadqueue::unlimited::Queue<CancelProcessorInput>;

#[derive(Clone)]
struct CancelProcessorInput {
    job_id: String,
    num_retries: i32,
}

// A shared queue wrapper that will be used for communicating with other processes
pub struct CancelProcessorQueue {
    queue: Queue,
    queued_ids: HashSet<String>,
}

const FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);

// A simple thread-safe queue with deduplication
impl Default for CancelProcessorQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl CancelProcessorQueue {
    pub fn new() -> CancelProcessorQueue {
        CancelProcessorQueue {
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
            self.queue.push(CancelProcessorInput {
                job_id,
                num_retries: 0,
            });
        }
    }

    fn enqueue_with_retries(&self, job_id: String, num_retries: i32) {
        self.queue.push(CancelProcessorInput {
            job_id,
            num_retries,
        });
    }

    async fn pop(&self) -> CancelProcessorInput {
        let res = self.queue.pop().await;
        self.queued_ids.remove(&res.job_id);
        res
    }
}

// CancelProcessor is an asynchronous handler that will cancel any outstanding AWS Batch jobs for a given Render Job.
pub struct CancelProcessor {
    queue: Arc<CancelProcessorQueue>,
    pool: Arc<PoolType>,
    handle: Arc<AppHandle>,
}

impl CancelProcessor {
    pub fn new(
        queue: Arc<CancelProcessorQueue>,
        pool: Arc<PoolType>,
        handle: Arc<AppHandle>,
    ) -> CancelProcessor {
        CancelProcessor {
            queue,
            handle,
            pool,
        }
    }

    // Start the forever-running task that will attempt to cance Batch Jobs for a Render Job
    pub async fn start_task(&self, mut batch_manager: BatchClientManager) {
        loop {
            let input = self.queue.pop().await;
            let job_id = input.job_id.clone();

            log::info!("Cancelling AWS Batch jobs for render job (job_id={job_id})...");
            self.handle_input(&mut batch_manager, &input).await
            .inspect(|_| log::info!("Cancelled all AWS Batch jobs for render job (job_id={job_id})."))
                .unwrap_or_else(|f| {
                log::error!("Encountered unexpected error when cancelling AWS Batch jobs for render job (job_id={job_id}): {f}");

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

    async fn handle_input(
        &self,
        batch_manager: &mut BatchClientManager,
        input: &CancelProcessorInput,
    ) -> Result<(), AppError> {
        let client = batch_manager.get_client().await?;

        let job_opt = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &input.job_id)
        })?;

        // Unwrap job
        if job_opt.is_none() {
            log::info!("Job not found. Skipping cancellation.");
            return Ok(());
        }
        let mut job = job_opt.unwrap();

        let mut tasks = with_conn(&self.pool, |conn| {
            render_task::repo::list_by_job_id(conn, &job.id)
        })?;

        // Cancel AWS Batch Jobs
        let batch_job_ids = batch::render_job::list_incomplete_jobs(client, &job.id).await?;
        batch::render_job::cancel_jobs(client, batch_job_ids).await?;

        // Update statuses
        log::info!("Updating job status to cancelled.");
        job.status = render_job::entity::JobStatus::Canceled;
        tasks = tasks
            .into_iter()
            .filter(|t| !matches!(t.status, TaskStatus::Succeeded | TaskStatus::Failed))
            .map(|mut t| {
                t.status = render_task::entity::TaskStatus::Canceled;
                t
            })
            .collect::<Vec<_>>();

        // Persist results
        with_transaction(&self.pool, |tx| {
            render_job::repo::save(tx, &job)?;
            for task in tasks {
                render_task::repo::save(tx, &task)?;
            }
            Ok(())
        })?;

        emit_job_status_update_event(&self.handle, &job.id)?;

        Ok(())
    }
}
