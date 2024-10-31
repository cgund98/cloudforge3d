use crate::errors::AppError;
use crate::spec::proto::v1;

use crate::state::AppState;

#[tauri::command]
pub async fn create_job(
    input: v1::CreateJobRequest,
    state: tauri::State<'_, AppState>,
) -> Result<v1::CreateJobResponse, AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.create_job(input).await
}

#[tauri::command]
pub fn list_jobs(
    input: v1::ListJobsRequest,
    state: tauri::State<'_, AppState>,
) -> Result<v1::ListJobsResponse, AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.list_jobs(input)
}

#[tauri::command]
pub fn get_job(
    input: v1::GetJobRequest,
    state: tauri::State<'_, AppState>,
) -> Result<v1::GetJobResponse, AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.get_job(input)
}

#[tauri::command]
pub fn update_job(
    input: v1::UpdateJobRequest,
    state: tauri::State<'_, AppState>,
) -> Result<(), AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.update_job(input)
}

#[tauri::command]
pub fn cancel_job(
    input: v1::CancelJobRequest,
    state: tauri::State<'_, AppState>,
) -> Result<(), AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.cancel_job(input)
}

#[tauri::command]
pub async fn delete_job(
    input: v1::DeleteJobRequest,
    state: tauri::State<'_, AppState>,
) -> Result<(), AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.delete_job(input).await
}

#[tauri::command]
pub async fn download_frames(
    input: v1::DownloadJobOutputsRequest,
    state: tauri::State<'_, AppState>,
) -> Result<v1::DownloadJobOutputsResponse, AppError> {
    let ctrl = state.job_ctrl.as_ref().unwrap();
    ctrl.download_frames(input).await
}
