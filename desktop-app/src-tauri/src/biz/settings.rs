use std::sync::Arc;

use crate::{
    errors::AppError,
    infra::{s3::{client_manager::S3ClientManager, health::check_health}, settings::SettingsRepo},
    spec::proto::v1::{GetAwsCredentialsResponse, CheckDeploymentHealthResponse, UpdateAwsCredentialsCommand},
};

pub struct Controller {
    repo: Arc<SettingsRepo>,
    s3_manager: Arc<S3ClientManager>,
}

impl Controller {
    pub fn new(repo: Arc<SettingsRepo>, s3_manager: Arc<S3ClientManager>) -> Controller {
        Controller { repo, s3_manager }
    }

    pub async fn update_aws_credentials(
        &self,
        input: UpdateAwsCredentialsCommand,
    ) -> Result<(), AppError> {
        log::info!("Updating aws credentials...");
        let mut config = self.repo.get_aws_config().await?;

        config.access_key_id = input.access_key_id.unwrap_or(config.access_key_id);
        config.secret_access_key = input.secret_access_key.unwrap_or(config.secret_access_key);
        config.region = input.region.unwrap_or(config.region);

        self.repo.save_aws_config(config).await
    }

    pub async fn get_aws_credentials(&self) -> Result<GetAwsCredentialsResponse, AppError> {
        log::info!("Fetching aws credentials...");
        let config = self.repo.get_aws_config().await?;

        let response = GetAwsCredentialsResponse {
            access_key_id: config.access_key_id,
            secret_access_key: config.secret_access_key,
            region: config.region,
        };

        Ok(response)
    }

    pub async fn check_deployment_health(&self) -> Result<CheckDeploymentHealthResponse, AppError> {
        let s3_client = self.s3_manager.get_client().await?;
        let health_response = check_health(&s3_client).await;

        let status = match health_response {
            Ok(_) => "ready",
            Err(_) => "unreachable",
        };

        let response = CheckDeploymentHealthResponse { status: status.to_string() };

        Ok(response)
    }
}
