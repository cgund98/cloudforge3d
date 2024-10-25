
use std::sync::Arc;

use std::path::Path;

use crate::errors::AppError;
use crate::infra::db;
use crate::spec::proto::v1::{self, CreateJobResponse};
use super::processor::FileUploadProcessorInput;

pub struct Controller {
    job_repo: Arc<db::render_job::repo::Repo>,
    file_upload_queue: Arc<super::processor::FileUploadQueue>,
}

impl Controller {

    pub fn new(job_repo: Arc<db::render_job::repo::Repo>, file_upload_queue: Arc<super::processor::FileUploadQueue>) -> Controller {
        Controller {
            job_repo,
            file_upload_queue,
        }
    }

    // Create a new render job
    pub fn create_job(&self, req: v1::CreateJobRequest) -> Result<CreateJobResponse, AppError> {

        let blend_exists = Path::new(&req.file_path).exists();
        if !blend_exists {
            return Err(AppError::BadRequest("Specified blend file does not exist.".to_string()))
        }

        if req.ocio_config_path.is_some() {
            let ocio_exists = Path::new(&req.ocio_config_path.clone().unwrap()).exists();
            if !ocio_exists {
                return Err(AppError::BadRequest("Specified OCIO config file does not exist.".to_string()))
            }
        }

        let input = FileUploadProcessorInput {
            job_id: "test-id".to_string(),
            file_path: req.file_path,
            ocio_config_path: req.ocio_config_path,
        };

        self.file_upload_queue.enqueue(input);

        Ok(CreateJobResponse { job_id: "test-id".to_string() })
    }
}