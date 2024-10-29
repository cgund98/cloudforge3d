use tauri::{AppHandle, Emitter};

use crate::{errors::AppError, spec::proto::v1};

// Define event types
enum EventName {
    JobFileUploadProgress,
    JobStatusUpdate,
}

impl ToString for EventName {
    fn to_string(&self) -> String {
        match self {
            EventName::JobFileUploadProgress => "job-file-upload-progress".to_string(),
            EventName::JobStatusUpdate => "job-status-update".to_string(),
        }
    }
}

// Emit a file upload progress event
pub fn emit_job_file_upload_progress_event(
    handle: &AppHandle,
    event: &v1::JobFileUploadProgressEvent,
) -> Result<(), AppError> {
    // Serialize to JSON
    let event_ser = serde_json::to_string(&event)?;

    // Emit event
    let event_name = EventName::JobFileUploadProgress.to_string();
    handle.emit(&event_name, event_ser)?;

    Ok(())
}

// Emit a job status update event
pub fn emit_job_status_update_event(handle: &AppHandle, job_id: &str) -> Result<(), AppError> {
    let event = v1::JobStatusUpdateEvent {
        job_id: job_id.to_string(),
    };

    // Serialize to JSON
    let event_ser = serde_json::to_string(&event)?;

    // Emit event
    let event_name = EventName::JobStatusUpdate.to_string();
    handle.emit(&event_name, event_ser)?;

    Ok(())
}
