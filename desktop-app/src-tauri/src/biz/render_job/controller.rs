
use crate::infra::db;

use crate::spec::proto::v1;

pub struct Controller {
    job_repo: db::render_job::repo::Repo,
}

impl Controller {

    // Create a new render job
    pub fn create_job(req: v1::CreateJobRequest) {

        

    }
}