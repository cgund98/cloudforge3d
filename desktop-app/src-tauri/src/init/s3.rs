
use std::sync::Arc;

use aws_sdk_s3::Client as S3Client;
use aws_config::BehaviorVersion;

// Initialize the S3 client
pub async fn init_s3_client() -> Arc<aws_sdk_s3::Client> {
    let shared_config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let client = S3Client::new(&shared_config);

    Arc::new(client)
}