use std::sync::Arc;

use tauri::State;

use crate::biz;

/** Custom types */
pub type WrappedState<'a> = State<'a, AppState>;

/** App State */

#[derive(Default)]
pub struct AppState {
    pub job_ctrl: Option<Arc<biz::render_job::controller::Controller>>,
    pub task_ctrl: Option<Arc<biz::render_task::controller::Controller>>,
    pub settings_ctrl: Option<biz::settings::Controller>,
}
