use crate::errors::AppError;
use crate::spec::proto::v1;

use crate::state::AppState;

#[tauri::command]
pub fn list_tasks(
    input: v1::ListTasksRequest,
    state: tauri::State<'_, AppState>,
) -> Result<v1::ListTasksResponse, AppError> {
    let ctrl = state.task_ctrl.as_ref().unwrap();
    ctrl.list_tasks(input)
}
