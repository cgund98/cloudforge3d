use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/** Schema */

type JsonDb = jsondb::JsonDb<V1>;

#[derive(Default, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct V1 {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
}

impl jsondb::SchemaV0 for V1 {
}

#[derive(Hash, Clone)]
pub struct AwsConfig {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
}

/** Repository */

pub struct SettingsRepo {
    db: jsondb::JsonDb<V1>
}

impl SettingsRepo {
    pub async fn new(db_path: PathBuf) -> SettingsRepo {
        let db = JsonDb::load(db_path)
        .await
        .inspect_err(|e| log::error!("Unable to initialize settings DB: {e}"))
        .unwrap();

        SettingsRepo {
            db
        }
    }

    pub async fn save_aws_config(&self, config: AwsConfig) -> Result<(), AppError> {
        let mut writer = self.db.write().await;

        writer.aws_access_key_id = config.access_key_id;
        writer.aws_secret_access_key = config.secret_access_key;
        writer.aws_region = config.region;


        Ok(())
    }

    pub async fn get_aws_config(&self) -> Result<AwsConfig, AppError> {
        let reader = self.db.read().await;

        let config = AwsConfig {
            region: reader.aws_region.clone(),
            access_key_id: reader.aws_access_key_id.clone(),
            secret_access_key: reader.aws_secret_access_key.clone(),
        };

        Ok(config) 
    }
}