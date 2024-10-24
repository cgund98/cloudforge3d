use tauri::State;

use crate::biz;


/** Custom types */
pub type WrappedState<'a> = State<'a, AppState>;

/** App State */

#[derive(Default)]
pub struct AppState {
    pub job_ctrl:Option<biz::render_job::controller::Controller>,
    pub settings_ctrl:Option<biz::settings::Controller>,
}