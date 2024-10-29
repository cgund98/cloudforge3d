use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::infra::settings;

// Helper struct used to hold all initialized repos
pub struct Repos {
    pub settings_repo: Arc<settings::SettingsRepo>,
}

// Initialize repositories
pub async fn init_repos(handle: &AppHandle) -> Repos {
    // Settings database path
    let binding = handle.path().app_data_dir().unwrap();
    let data_path = binding.as_path();
    let settings_db_path = data_path.join("settings.json");

    let settings_repo = settings::SettingsRepo::new(settings_db_path).await;

    Repos {
        settings_repo: Arc::new(settings_repo),
    }
}
