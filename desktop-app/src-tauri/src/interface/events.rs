use tauri::{AppHandle, Emitter};

use crate::{errors::AppError, spec::proto::v1::JobFileUploadProgressEvent};



// Define event types
enum EventName {
    JobFileUploadProgress
}

impl ToString for EventName {
    fn to_string(&self) -> String {
        match self {
            EventName::JobFileUploadProgress => "job-file-upload-progress".to_string()
        }
    }
}

// Emit a file upload progress event
pub fn emit_job_file_upload_progress_event(handle: &AppHandle, event: JobFileUploadProgressEvent) -> Result<(), AppError> {

    // Serialize to JSON
    let event_ser = serde_json::to_string(&event)?;

    // Emit event
    let event_name = EventName::JobFileUploadProgress.to_string();
    handle.emit(&event_name, event_ser)?;

    Ok(())
}