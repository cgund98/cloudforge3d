use std::{path::PathBuf, sync::Arc};

use tauri::{AppHandle, Manager};

use crate::{
    biz::render_job::thumbnail_generator::ThumbnailGeneratorQueue,
    errors::AppError,
    infra::{
        db::{
            decorator::{with_conn, with_transaction},
            pool::PoolType,
            render_job, render_task,
        },
        s3::job_file::download_job_file,
    },
    interface::events::emit_job_status_update_event,
    spec::{proto::v1, timestamp::to_pb_timestamp},
};

pub struct Controller {
    pool: Arc<PoolType>,
    handle: Arc<AppHandle>,
    thumbnail_queue: Arc<ThumbnailGeneratorQueue>,
}

impl Controller {
    pub fn new(
        pool: Arc<PoolType>,
        handle: Arc<AppHandle>,
        thumbnail_queue: Arc<ThumbnailGeneratorQueue>,
    ) -> Controller {
        Controller {
            pool,
            handle,
            thumbnail_queue,
        }
    }

    pub fn list_tasks(
        &self,
        input: v1::ListTasksRequest,
    ) -> Result<v1::ListTasksResponse, AppError> {
        let tasks = with_conn(&self.pool, |conn| {
            render_task::repo::list_by_job_id(conn, &input.job_id)
        })?;

        let mut response_tasks: Vec<v1::TaskDetails> = Vec::new();
        for task in tasks {
            let response_task = v1::TaskDetails {
                id: task.id,
                job_id: task.job_id,
                frame_number: task.frame_number,
                created_at: task.created_at.map(to_pb_timestamp),
                queued_at: task.queued_at.map(to_pb_timestamp),
                started_at: task.started_at.map(to_pb_timestamp),
                completed_at: task.completed_at.map(to_pb_timestamp),
                status: task.status.to_string(),
                retry_count: task.retry_count,
            };

            response_tasks.push(response_task);
        }

        let response = v1::ListTasksResponse {
            tasks: response_tasks,
        };

        Ok(response)
    }

    pub async fn handle_status_update(
        &self,
        input: v1::TaskStatusUpdate,
        s3_client: &aws_sdk_s3::Client,
    ) -> Result<(), AppError> {
        let task_id = input.task_id.to_string();
        let task_result = with_conn(&self.pool, |conn| {
            render_task::repo::get_by_id(conn, &task_id)
        })?;

        if task_result.is_none() {
            log::info!("Task does not exist. Will not update status.");
            return Ok(());
        }

        let mut task = task_result.unwrap();

        // Update task fields
        task.status = render_task::entity::TaskStatus::from(input.status());

        let now = chrono::offset::Utc::now();
        match input.status() {
            v1::TaskStatus::Unspecified => (),
            v1::TaskStatus::Pending => (),
            v1::TaskStatus::Canceled => task.completed_at = Some(now),
            v1::TaskStatus::Running => task.started_at = Some(now),
            v1::TaskStatus::Failed => task.completed_at = Some(now),
            v1::TaskStatus::Succeeded => task.completed_at = Some(now),
        }

        // Fetch thumbnail
        let mut received_preview = false;
        if input.status() == v1::TaskStatus::Succeeded {
            let thumbnail_res = self.download_thumbnail(s3_client, &task).await;
            received_preview = thumbnail_res.unwrap_or(false);
        }

        // Fetch job
        let mut job = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &task.job_id)
        })?
        .unwrap();

        // Generate thumbnail
        if received_preview {
            self.thumbnail_queue.enqueue(job.id.clone());
        }

        // See if other tasks are still running
        let remaining_task_count = with_conn(&self.pool, |conn| {
            render_task::repo::count_unfinished_tasks(conn, &job.id, &task.id)
        })?;
        let other_tasks_complete = remaining_task_count == 0;

        // Update job fields
        let job_canceled = job.status == render_job::entity::JobStatus::Canceled
            || job.status == render_job::entity::JobStatus::Canceling;
        let job_failed = job.status == render_job::entity::JobStatus::Failed;

        if job_canceled {
            // If job has been cancelled, do not handle any updates here.
            return Ok(());
        } else if task.status == render_task::entity::TaskStatus::Failed {
            job.status = render_job::entity::JobStatus::Failed;
        } else if task.status == render_task::entity::TaskStatus::Running && !job_failed {
            job.status = render_job::entity::JobStatus::Running;
        } else if task.status == render_task::entity::TaskStatus::Succeeded {
            job.frame_rendered_count = job.frame_count - remaining_task_count as i32;

            if other_tasks_complete {
                job.status = render_job::entity::JobStatus::Succeeded;
                job.completed_at = Some(now);
            }
        }

        with_transaction(&self.pool, |tx| {
            render_task::repo::save(tx, &task)?;
            render_job::repo::save(tx, &job)?;
            Ok(())
        })?;

        emit_job_status_update_event(&self.handle, &job.id)?;

        Ok(())
    }

    // Grab a WEBP thumbnail if it exists for a given task
    async fn download_thumbnail(
        &self,
        s3_client: &aws_sdk_s3::Client,
        task: &render_task::entity::RenderTask,
    ) -> Result<bool, AppError> {
        // Generate source key
        let job_id = task.job_id.clone();
        let source_key = format!("frame{:0width$}.preview.webp", task.frame_number, width = 4);

        // Generate output path
        let thumbnails_path = self.get_thumbnails_path(&job_id);
        let thumbnail_path = thumbnails_path.join(&source_key);

        // Download file
        let download_res = download_job_file(s3_client, &job_id, &source_key, thumbnail_path).await;

        let task_id = task.id.clone();
        if let Ok(_download_size) = download_res {
            log::info!("Downloaded thumbnail for task (id={task_id}).");
            return Ok(true);
        } else if let Err(download_err) = download_res {
            log::error!("Unable to fetch thumbnail for task (id={task_id}): {download_err}");
        }

        Ok(false)
    }

    fn get_thumbnails_path(&self, job_id: &str) -> PathBuf {
        let data_dir = self.handle.path().app_data_dir().unwrap();
        data_dir.join("thumbnails/jobs").join(&job_id)
    }
}
