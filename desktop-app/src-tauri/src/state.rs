use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::infra::db::pool::PoolType;


/** Custom types */
pub type WrappedState<'a> = State<'a, AppState>;

/** App State */

#[derive(Default)]
pub struct AppState {
    pub pool: Mutex<Option<PoolType>>,
}