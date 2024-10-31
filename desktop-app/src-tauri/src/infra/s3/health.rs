use crate::errors::AppError;

use super::constants::BUCKET_NAME;


// Check that a bucket is deployed
pub async fn check_health(client: &aws_sdk_s3::Client) -> Result<(), AppError> {
    client.list_objects()
        .bucket(BUCKET_NAME)
        .send()
        .await
        .map_err(|e| {
            if let Some(err) = e.as_service_error() {
                return AppError::S3HealthError(format!("{err}"));
            }
            AppError::S3HealthError(format!("{e}"))
        })?;

    Ok(())
}