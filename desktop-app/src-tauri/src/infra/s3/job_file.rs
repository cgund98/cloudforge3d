use std::path::PathBuf;

use aws_sdk_s3::{operation::create_multipart_upload::CreateMultipartUploadOutput, primitives::{ByteStream, Length}, types::{CompletedMultipartUpload, CompletedPart}};

use crate::errors::AppError;

const BUCKET_NAME: &str = "cf3d-blob-store";
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 5000;

// Progress Struct
pub struct Progress {
    pub job_id: String,
    pub file_name: String,
    pub uploaded: u64,
    pub total: u64,
}

// Construct the path for a job input file
fn generate_input_path(job_id: String, key: String) -> String {
    format!("jobs/{job_id}/inputs/{key}")
}

// Read file metadata and determine number of chunks
async fn parse_chunks_count(path: PathBuf) -> Result<(u64, u64), AppError> {
    // Read file metadata
    let file_metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| AppError::FileReadError(format!("{e}")))?;
    let file_size = file_metadata.len();

    // Calculate chunk count
    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    // Validate chunk count
    if file_size == 0 {
        return Err(AppError::FileReadError("File is empty.".to_string()));
    }
    if chunk_count > MAX_CHUNKS {
        return Err(AppError::FileReadError("File is too large.".to_string()));
    }

    Ok((chunk_count, size_of_last_chunk))
}

// Upload a large file to the job S3 bucket.
pub async fn upload_job_file(client: &aws_sdk_s3::Client, job_id: String, source_path: PathBuf, target_key: String, progress_callback: impl Fn(Progress) -> Result<(), AppError>) -> Result<(), AppError> {

    // Generate object path
    let upload_path = generate_input_path(job_id.clone(), target_key.clone());

    let file_name = source_path.file_name().map(|n| n.to_str()).unwrap_or(Some("")).unwrap_or("unknown");

    // Initialize upload
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(BUCKET_NAME)
        .key(&upload_path)
        .send()
        .await
        .map_err(|e| AppError::S3UploadError(format!("{e}")))?;

    let upload_id = multipart_upload_res.upload_id().ok_or(AppError::S3UploadError(
        "Missing upload_id after CreateMultipartUpload".to_string(),
    ))?;

    // Parse metadata
    let chunks_info = parse_chunks_count(source_path.clone()).await?;
    let chunk_count = chunks_info.0;
    let size_of_last_chunk = chunks_info.1;
    

    // Upload parts
    let mut upload_parts: Vec<aws_sdk_s3::types::CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };

        let stream = ByteStream::read_from()
            .path(source_path.clone())
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();

        // Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client
            .upload_part()
            .key(&upload_path)
            .bucket(BUCKET_NAME)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await
            .map_err(|e| AppError::S3UploadError(format!("{e}")))?;

        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );

        // Notify of upload progress
        let _ = progress_callback(Progress{
            job_id: job_id.clone(),
            file_name: file_name.to_string(),
            uploaded: chunk_index,
            total: chunk_count,
        }).inspect_err(|e| log::warn!("Encountered error while notifying progress update: {e}"));
    }

    // Close upload
    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();

    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(BUCKET_NAME)
        .key(&upload_path)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .map_err(|e| AppError::S3UploadError(format!("{e}")))?;

    Ok(())
}