use crate::errors::AppError;
use crate::spec::proto::v1::{CreateJobRequest, CreateJobResponse};

use crate::state::AppState;

#[tauri::command]
pub fn create_job(input: CreateJobRequest, state: tauri::State<'_, AppState>) -> Result<CreateJobResponse, AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.create_job(input)
}