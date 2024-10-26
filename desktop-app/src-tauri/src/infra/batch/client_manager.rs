use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use aws_config::Region;
use aws_sdk_batch::config::Credentials;

use crate::{
    errors::AppError,
    infra::settings::{AwsConfig, SettingsRepo},
};

pub struct BatchClientManager {
    settings_repo: Arc<SettingsRepo>,
    cur_client: Option<aws_sdk_batch::Client>,
    cur_hash: u64,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl BatchClientManager {
    pub fn new(settings_repo: Arc<SettingsRepo>) -> BatchClientManager {
        BatchClientManager {
            settings_repo,
            cur_client: None,
            cur_hash: 0,
        }
    }

    // Return an up-to-date Batch client
    pub async fn get_client(&mut self) -> Result<&aws_sdk_batch::Client, AppError> {
        let config = self.settings_repo.get_aws_config().await?;

        // Initialize a new Batch client if credentials have not yet been set
        if self.check_config_change(config.clone()) || self.cur_client.is_none() {
            let credentials = Credentials::new(
                config.access_key_id,
                config.secret_access_key,
                None,
                None,
                "manual",
            );
            let region = Region::new(config.region);
            let s3_config = aws_sdk_batch::config::Builder::new()
                .region(region)
                .credentials_provider(credentials)
                .behavior_version_latest()
                .build();
            let client = aws_sdk_batch::Client::from_conf(s3_config);
            self.cur_client = Some(client)
        }

        Ok(self.cur_client.as_ref().unwrap())
    }

    fn check_config_change(&mut self, config: AwsConfig) -> bool {
        let latest_hash = calculate_hash(&config);

        if latest_hash != self.cur_hash {
            self.cur_hash = latest_hash;
            return false;
        }

        return true;
    }
}
