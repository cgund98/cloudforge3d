
use std::sync::Arc;

use crate::infra::db;

use crate::spec::proto::v1;

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
    pub fn create_job(req: v1::CreateJobRequest) {

        

    }
}