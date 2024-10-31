use std::sync::Arc;

use tonic::{Request, Response};

use crate::{biz::render_job::controller::Controller, spec::proto::v1};

pub struct JobsService {
    controller: Arc<Controller>,
}

impl JobsService {
    pub fn new(controller: Arc<Controller>) -> JobsService {
        JobsService { controller }
    }
}

#[tonic::async_trait]
impl v1::jobs_service_server::JobsService for JobsService {
    async fn create_job(
        &self,
        req: Request<v1::CreateJobRequest>,
    ) -> Result<Response<v1::CreateJobResponse>, tonic::Status> {
        let body = req.into_inner();

        let res = self.controller.create_job(body).await;

        if let Ok(response) = res {
            return Ok(Response::new(response));
        } else if let Err(err) = res {
            return Err(tonic::Status::internal(err.to_string()));
        }

        Err(tonic::Status::internal("Internal Server Error."))
    }
}
