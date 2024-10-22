use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct RenderTask {
    pub id: String,
    pub job_id: String,
    pub frame_number: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub queued_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub retry_count: i32,
}

#[derive(Debug)]
pub enum TaskStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl std::str::FromStr for TaskStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TaskStatus, Self::Err> {
        match input {
            "pending" => Ok(TaskStatus::Pending),
            "running" => Ok(TaskStatus::Running),
            "succeeded" => Ok(TaskStatus::Succeeded),
            "failed" => Ok(TaskStatus::Failed),
            _ => Err(()),
        }
    }
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Succeeded => "succeeded".to_string(),
            TaskStatus::Failed => "failed".to_string(),
        }
    }
}