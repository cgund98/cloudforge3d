
use crate::infra::db;

use crate::spec::proto::cf3d_v1;

struct Controller {
    job_repo: db::render_job::repo::Repo,
}

impl Controller {

    // Create a new render job
    pub fn create_job(req: cf3d_v1::CreateJobRequest) {

    }
}