use std::sync::Arc;

use aws_config::Region;
use aws_sdk_s3::{config::Credentials, Client};

use crate::{errors::AppError, infra::settings::SettingsRepo};

pub type CachedClient = Client;

// S3ClientManager will manage our AWS S3 client for us. Each time we request a client, it will
// create a new one.
pub struct S3ClientManager {
    settings_repo: Arc<SettingsRepo>,
}

impl S3ClientManager {
    pub fn new(settings_repo: Arc<SettingsRepo>) -> S3ClientManager {
        S3ClientManager { settings_repo }
    }

    // Return an up-to-date S3 client
    pub async fn get_client(&self) -> Result<CachedClient, AppError> {
        log::info!("Fetching s3 client...");
        let config = self.settings_repo.get_aws_config().await?;

        let credentials = Credentials::new(
            config.access_key_id,
            config.secret_access_key,
            None,
            None,
            "manual",
        );
        let region = Region::new(config.region);
        let s3_config = aws_sdk_s3::config::Builder::new()
            .region(region)
            .credentials_provider(credentials)
            .behavior_version_latest()
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_config);

        Ok(client)
    }
}
