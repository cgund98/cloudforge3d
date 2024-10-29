// Type aliases
type InitResult = Result<(), Box<dyn std::error::Error + 'static>>;

use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::biz;
use crate::biz::render_job::cancel_processor::{CancelProcessor, CancelProcessorQueue};
use crate::biz::render_job::thumbnail_generator::{ThumbnailGenerator, ThumbnailGeneratorQueue};
use crate::biz::render_job::upload_processor::FileUploadQueue;
use crate::infra::batch::client_manager::BatchClientManager;
use crate::infra::s3::client_manager::S3ClientManager;
use crate::infra::sqs::client_manager::SqsClientManager;
use crate::infra::sqs::task_status_update;
use crate::state::AppState;
use crate::{biz::render_job::upload_processor::FileUploadProcessor, infra::db::init::init_db};

use super::repo::init_repos;

async fn init_async_deps(handle: &AppHandle, processor_handle: AppHandle) -> AppState {
    let async_handle = Arc::new(processor_handle);

    // Initialize tables
    let pool = init_db(handle).unwrap();
    let repos = init_repos(handle).await;

    // AWS SDK client managers
    let s3_client_manager = Arc::new(S3ClientManager::new(repos.settings_repo.clone()));

    // Initialize queues for inter-process communication
    let file_upload_queue = Arc::new(FileUploadQueue::new());
    let thumbnail_queue = Arc::new(ThumbnailGeneratorQueue::new());
    let cancel_queue = Arc::new(CancelProcessorQueue::new());

    // Initialize controllers
    let job_ctrl = biz::render_job::controller::Controller::new(
        pool.clone(),
        file_upload_queue.clone(),
        cancel_queue.clone(),
        s3_client_manager.clone(),
        async_handle.clone(),
    );
    let task_ctrl = Arc::new(biz::render_task::controller::Controller::new(
        pool.clone(),
        async_handle.clone(),
        thumbnail_queue.clone(),
    ));
    let settings_ctrl = biz::settings::Controller::new(repos.settings_repo.clone());

    // Initialize state
    let state = crate::state::AppState {
        job_ctrl: Some(job_ctrl),
        task_ctrl: Some(task_ctrl.clone()),
        settings_ctrl: Some(settings_ctrl),
    };
    // Spawn a separate thread for the file upload processor

    let batch_client_manager = BatchClientManager::new(repos.settings_repo.clone());
    let processor = FileUploadProcessor::new(
        pool.clone(),
        file_upload_queue.clone(),
        async_handle.clone(),
        s3_client_manager.clone(),
    );

    tokio::spawn(async move { processor.start_task(batch_client_manager).await });

    // Spawn a separate thread for the task update consumer
    let sqs_client_manager = SqsClientManager::new(repos.settings_repo.clone());
    let mut updates_consumer = task_status_update::TaskStatusUpdateConsumer::new(
        s3_client_manager.clone(),
    );

    tokio::spawn(async move {
        updates_consumer
            .listen_for_task_status_updates(sqs_client_manager, &task_ctrl)
            .await
    });

    // Spawn a separate thread for thumbnail generation
    let thumbnail_generator =
        ThumbnailGenerator::new(pool.clone(), thumbnail_queue.clone(), async_handle.clone());
    tokio::spawn(async move { thumbnail_generator.start_task().await });

    // Spawn separate thread for job cancellation
    let cancel_processor =
        CancelProcessor::new(cancel_queue.clone(), pool.clone(), async_handle.clone());
    let cancel_batch_manager = BatchClientManager::new(repos.settings_repo.clone());
    tokio::spawn(async move { cancel_processor.start_task(cancel_batch_manager).await });

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
