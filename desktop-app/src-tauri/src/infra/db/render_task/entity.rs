use chrono::{DateTime, Utc};

use crate::spec::proto::v1;

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

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    Unknown,
    Pending,
    Running,
    Succeeded,
    Failed,
    Canceled,
}

impl std::str::FromStr for TaskStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TaskStatus, Self::Err> {
        match input {
            "unknown" => Ok(TaskStatus::Unknown),
            "pending" => Ok(TaskStatus::Pending),
            "running" => Ok(TaskStatus::Running),
            "succeeded" => Ok(TaskStatus::Succeeded),
            "failed" => Ok(TaskStatus::Failed),
            "canceled" => Ok(TaskStatus::Canceled),
            _ => Err(()),
        }
    }
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Unknown => "unknown".to_string(),
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Succeeded => "succeeded".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Canceled => "canceled".to_string(),
        }
    }
}

impl From<v1::TaskStatus> for TaskStatus {
    fn from(value: v1::TaskStatus) -> Self {
        match value {
            v1::TaskStatus::Unspecified => TaskStatus::Unknown,
            v1::TaskStatus::Pending => TaskStatus::Pending,
            v1::TaskStatus::Running => TaskStatus::Running,
            v1::TaskStatus::Succeeded => TaskStatus::Succeeded,
            v1::TaskStatus::Failed => TaskStatus::Failed,
            v1::TaskStatus::Canceled => TaskStatus::Canceled,
        }
    }
}

impl Into<v1::TaskStatus> for TaskStatus {
    fn into(self) -> v1::TaskStatus {
        match self {
            TaskStatus::Unknown => v1::TaskStatus::Unspecified,
            TaskStatus::Pending => v1::TaskStatus::Pending,
            TaskStatus::Running => v1::TaskStatus::Running,
            TaskStatus::Succeeded => v1::TaskStatus::Succeeded,
            TaskStatus::Failed => v1::TaskStatus::Failed,
            TaskStatus::Canceled => v1::TaskStatus::Canceled,
        }
    }
}
