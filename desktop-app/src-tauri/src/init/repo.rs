use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::infra::{db::{self, pool::PoolType}, settings};

// Helper struct used to hold all initialized repos
pub struct Repos {
    pub job_repo: Arc<db::render_job::repo::Repo>,
    pub task_repo: Arc<db::render_task::repo::Repo>,
    pub settings_repo: Arc<settings::SettingsRepo>,
}

// Initialize repositories
pub async fn init_repos(pool: Arc<PoolType>, handle: &AppHandle) -> Repos {

    // Settings database path
    let binding = handle.path().app_data_dir().unwrap();
    let data_path = binding.as_path();
    let settings_db_path = data_path.join("settings.json");

    let settings_repo = settings::SettingsRepo::new(settings_db_path).await;

    Repos {
        job_repo: Arc::new(db::render_job::repo::Repo::new(pool.clone())),
        task_repo: Arc::new(db::render_task::repo::Repo::new(pool.clone())),
        settings_repo: Arc::new(settings_repo),
    }
}