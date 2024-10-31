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

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let status = match self {
            TaskStatus::Unknown => "unknown",
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Succeeded => "succeeded",
            TaskStatus::Failed => "failed",
            TaskStatus::Canceled => "canceled",
        };

        fmt.write_str(status)?;
        Ok(())
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

impl From<TaskStatus> for v1::TaskStatus {
    fn from(val: TaskStatus) -> Self {
        match val {
            TaskStatus::Unknown => v1::TaskStatus::Unspecified,
            TaskStatus::Pending => v1::TaskStatus::Pending,
            TaskStatus::Running => v1::TaskStatus::Running,
            TaskStatus::Succeeded => v1::TaskStatus::Succeeded,
            TaskStatus::Failed => v1::TaskStatus::Failed,
            TaskStatus::Canceled => v1::TaskStatus::Canceled,
        }
    }
}
