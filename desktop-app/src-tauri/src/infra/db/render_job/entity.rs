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
    pub file_size_mb: f32,
    pub frame_count: i32,
    pub frame_rate: i32,
    pub frame_start: i32,
    pub download_path: String,
    pub has_preview: bool,
    pub frame_rendered_count: i32,
}

#[derive(Debug)]
pub enum JobStatus {
    Uploading,
    Pending,
    Running,
    Succeeded,
    Failed,
    Canceled,
}

impl std::str::FromStr for JobStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<JobStatus, Self::Err> {
        match input {
            "uploading" => Ok(JobStatus::Uploading),
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "succeeded" => Ok(JobStatus::Succeeded),
            "failed" => Ok(JobStatus::Failed),
            "canceled" => Ok(JobStatus::Canceled),
            _ => Err(()),
        }
    }
}

impl ToString for JobStatus {
    fn to_string(&self) -> String {
        match self {
            JobStatus::Uploading => "uploading".to_string(),
            JobStatus::Pending => "pending".to_string(),
            JobStatus::Running => "running".to_string(),
            JobStatus::Succeeded => "succeeded".to_string(),
            JobStatus::Failed => "failed".to_string(),
            JobStatus::Canceled => "canceled".to_string(),
        }
    }
}