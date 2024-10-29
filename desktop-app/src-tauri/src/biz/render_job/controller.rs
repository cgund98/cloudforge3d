use std::sync::Arc;

use std::path::Path;

use tauri::App;

use super::upload_processor::FileUploadProcessorInput;
use super::validation;
use crate::errors::AppError;
use crate::infra::db::decorator::{with_conn, with_transaction};
use crate::infra::db::pool::PoolType;
use crate::infra::db::render_job;
use crate::infra::db::render_job::entity::{JobStatus, RenderJob};
use crate::infra::file::parse_file_size_bytes;
use crate::spec::proto::v1::{self, ListJobsResponseItem};
use crate::spec::timestamp::to_pb_timestamp;

pub struct Controller {
    pool: Arc<PoolType>,
    file_upload_queue: Arc<super::upload_processor::FileUploadQueue>,
    cancel_queue: Arc<super::cancel_processor::CancelProcessorQueue>,
}

impl Controller {
    pub fn new(
        pool: Arc<PoolType>,
        file_upload_queue: Arc<super::upload_processor::FileUploadQueue>,
        cancel_queue: Arc<super::cancel_processor::CancelProcessorQueue>,
    ) -> Controller {
        Controller {
            pool,
            file_upload_queue,
            cancel_queue,
        }
    }

    // Create a new render job
    pub async fn create_job(
        &self,
        req: v1::CreateJobRequest,
    ) -> Result<v1::CreateJobResponse, AppError> {
        // Validation
        validation::can_create_job(&req)?;

        // Parse file size
        let blend_path = Path::new(&req.file_path);
        let blend_file_size_bytes = parse_file_size_bytes(blend_path.to_path_buf()).await?;
        let blend_file_size_mb = blend_file_size_bytes / 1024 / 1024;

        // Parse additional fields
        let file_name = blend_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        // Persist job
        let job_id = uuid::Uuid::new_v4().to_string();
        let job = RenderJob {
            id: job_id.clone(),
            name: file_name.clone(),
            status: JobStatus::Uploading,
            created_at: chrono::offset::Utc::now(),
            queued_at: None,
            completed_at: None,
            file_name,
            file_size_mb: blend_file_size_mb,
            download_path: req.download_path,
            frame_count: req.frame_count,
            frame_rate: req.frame_rate,
            frame_start: req.frame_start,
            has_preview: false,
            frame_rendered_count: 0,
        };
        with_transaction(&self.pool, |tx| render_job::repo::save(tx, &job))?;

        // Fetch persisted job
        let job = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &job_id)
        })?
        .unwrap();

        // Enqueue file upload
        let input = FileUploadProcessorInput {
            job_id: job.id,
            file_path: req.file_path,
            ocio_config_path: req.ocio_config_path,
        };

        self.file_upload_queue.enqueue(input);

        Ok(v1::CreateJobResponse {
            job_id: "test-id".to_string(),
        })
    }

    pub fn list_jobs(&self, req: v1::ListJobsRequest) -> Result<v1::ListJobsResponse, AppError> {
        let limit = req.limit.unwrap_or(10).min(50);
        let offset = req.offset.unwrap_or(0).max(0);

        let (jobs, count) = with_conn(&self.pool, |conn| {
            render_job::repo::list(conn, limit, offset)
        })?;

        // Re-format for response type
        let mut response_jobs: Vec<v1::ListJobsResponseItem> = Vec::new();
        for job in jobs {
            response_jobs.push(ListJobsResponseItem {
                id: job.id,
                name: job.name,
                status: job.status.to_string(),
                created_at: Some(to_pb_timestamp(job.created_at)),
            })
        }

        Ok(v1::ListJobsResponse {
            jobs: response_jobs,
            total: count,
        })
    }

    pub fn get_job(&self, req: v1::GetJobRequest) -> Result<v1::GetJobResponse, AppError> {
        let job_id = req.job_id;

        let job = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &job_id)
        })?;

        if let Some(found_job) = job {
            let response_job = v1::JobDetails {
                id: job_id,
                name: found_job.name,
                status: found_job.status.to_string(),
                created_at: Some(to_pb_timestamp(found_job.created_at)),
                queued_at: found_job.queued_at.map(to_pb_timestamp),
                completed_at: found_job.completed_at.map(to_pb_timestamp),
                file_name: found_job.file_name,
                file_size_mb: found_job.file_size_mb,
                frame_count: found_job.frame_count,
                frame_rate: found_job.frame_rate,
                frame_start: found_job.frame_start,
                frame_rendered_count: found_job.frame_rendered_count,
                download_path: found_job.download_path,
                has_preview: found_job.has_preview,
            };

            return Ok(v1::GetJobResponse {
                job: Some(response_job),
            });
        }

        Ok(v1::GetJobResponse { job: None })
    }

    pub fn cancel_job(&self, req: v1::CancelJobRequest) -> Result<(), AppError> {
        let job_id = req.job_id;

        let job = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &job_id)
        })?;

        if job.is_none() {
            return Err(AppError::BadRequest("Job not found.".to_string()));
        }

        let mut found_job = job.unwrap();

        // Validate status
        if found_job.status == render_job::entity::JobStatus::Canceled {
            return Err(AppError::BadRequest(
                "Job has already been canceled.".to_string(),
            ));
        } else if found_job.status == render_job::entity::JobStatus::Uploading {
            return Err(AppError::BadRequest(
                "Cannot cancel an uploading job.".to_string(),
            ));
        }

        // Do cancellation
        found_job.status = render_job::entity::JobStatus::Canceling;
        self.cancel_queue.enqueue(job_id.clone());
        with_transaction(&self.pool, |tx| render_job::repo::save(tx, &found_job))?;
        log::info!("Scheduled cancellation for job (job_id={job_id}).");

        Ok(())
    }
}
