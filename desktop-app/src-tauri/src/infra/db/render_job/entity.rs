use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct RenderJob {
    pub id: String,
    pub name: String,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub queued_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub file_name: String,
    pub file_size_mb: u64,
    pub frame_count: i32,
    pub frame_rate: i32,
    pub frame_start: i32,
    pub download_path: String,
    pub has_preview: bool,
    pub frame_rendered_count: i32,
}

#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Uploading,
    UploadFailed,
    Pending,
    Running,
    Succeeded,
    Failed,
    Canceling,
    Canceled,
    Deleting,
}

impl std::str::FromStr for JobStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<JobStatus, Self::Err> {
        match input {
            "uploading" => Ok(JobStatus::Uploading),
            "uploading-failed" => Ok(JobStatus::UploadFailed),
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "succeeded" => Ok(JobStatus::Succeeded),
            "failed" => Ok(JobStatus::Failed),
            "canceling" => Ok(JobStatus::Canceling),
            "canceled" => Ok(JobStatus::Canceled),
            "deleting" => Ok(JobStatus::Deleting),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let status = match self {
            JobStatus::Uploading => "uploading",
            JobStatus::UploadFailed => "uploading-failed",
            JobStatus::Pending => "pending",
            JobStatus::Running => "running",
            JobStatus::Succeeded => "succeeded",
            JobStatus::Failed => "failed",
            JobStatus::Canceling => "canceling",
            JobStatus::Canceled => "canceled",
            JobStatus::Deleting => "deleting",
        };

        fmt.write_str(status)?;
        Ok(())
    }
}
