
// Type aliases
type InitResult = Result<(), Box<dyn std::error::Error + 'static>>;

use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::biz;
use crate::biz::render_job::processor::FileUploadQueue;
use crate::infra::s3::client_manager::S3ClientManager;
use crate::state::AppState;
use crate::{biz::render_job::processor::FileUploadProcessor, infra::db::init::init_db};

use super::repo::init_repos;

async fn init_async_deps(handle: &AppHandle, processor_handle: AppHandle) -> AppState{

    // Initialize tables
    let pool = init_db(handle).unwrap();
    let repos = init_repos(pool, handle).await;

    // Initialize queues for process communication
    let file_upload_queue = Arc::new(FileUploadQueue::new());

    // Initialize controllers
    let job_ctrl = biz::render_job::controller::Controller::new(repos.job_repo.clone(), file_upload_queue.clone());
    let settings_ctrl = biz::settings::Controller::new(repos.settings_repo.clone());

    // Initialize state
    let state = crate::state::AppState{
        job_ctrl: Some(job_ctrl),
        settings_ctrl: Some(settings_ctrl),
    };

    // Spawn a separate thread for the processor
    tokio::spawn(async move {
        // Initialize s3 client manager
        let s3_client_manager = S3ClientManager::new(repos.settings_repo);

        // Initialize file upload processor
        let processor = FileUploadProcessor::new(repos.job_repo, file_upload_queue.clone(), Arc::new(processor_handle));
        processor.start_task(s3_client_manager).await
    });

    state
}

// Dependency initializes methods
pub fn init_deps(app: &mut tauri::App) -> InitResult {
    let handle = app.handle();
    let processor_handle = app.handle().to_owned();

    let future = init_async_deps(handle, processor_handle);

    // Async initialize
    let state = tokio::task::block_in_place(move || {
        let result = tauri::async_runtime::block_on(async { future.await });

        result
    });

    app.manage(state);

    Ok(())
}