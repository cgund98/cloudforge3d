

pub async fn init_s3() -> Result<aws_sdk_s3::Client> {
    let shared_config = aws_config::load_from_env().await;
    let client = S3Client::new(&shared_config);
}