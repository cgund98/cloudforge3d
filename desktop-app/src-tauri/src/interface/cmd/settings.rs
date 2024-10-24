use crate::{errors::AppError, spec::proto::v1::{GetAwsCredentialsResponse, UpdateAwsCredentialsCommand}, state::AppState};


#[tauri::command]
pub async fn update_aws_credentials(input: UpdateAwsCredentialsCommand, state: tauri::State<'_, AppState>) -> Result<(), AppError> {
    let ctrl = state.settings_ctrl.as_ref().unwrap();
    ctrl.update_aws_credentials(input).await
}

#[tauri::command]
pub async fn get_aws_credentials(state: tauri::State<'_, AppState>) -> Result<GetAwsCredentialsResponse, AppError> {
    let ctrl = state.settings_ctrl.as_ref().unwrap();
    let result = ctrl.get_aws_credentials().await?;

    log::info!("Got result: {result:?}");

    Ok(result)
}