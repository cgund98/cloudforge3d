use base64::prelude::*;
use core::time;
use prost::Message;
use std::io::Cursor;
use std::sync::Arc;
use std::{str, thread};
use tauri::AppHandle;

use crate::biz::render_task::controller::Controller;
use crate::errors::AppError;
use crate::infra::s3::client_manager::S3ClientManager;
use crate::spec::proto::v1;

use super::client_manager::SqsClientManager;

const CLIENT_FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);
const MESSAGE_FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(12);

const MAX_MESSAGES: i32 = 10;
const QUEUE_NAME: &str = "cf3d-task-updates.fifo";

pub struct TaskStatusUpdateConsumer {
    queue_url: Option<String>,
    handle: Arc<AppHandle>,
}

impl TaskStatusUpdateConsumer {
    pub fn new(handle: Arc<AppHandle>) -> TaskStatusUpdateConsumer {
        TaskStatusUpdateConsumer {
            queue_url: None,
            handle,
        }
    }

    pub async fn listen_for_task_status_updates(
        &mut self,
        mut sqs_manager: SqsClientManager,
        mut s3_manager: S3ClientManager,
        controller: &Controller,
    ) -> Result<(), AppError> {
        loop {
            let sqs_client_res = sqs_manager.get_client().await;
            let s3_client_res = s3_manager.get_client().await;

            if s3_client_res.is_err() || sqs_client_res.is_err() {
                let e = s3_client_res.err().unwrap_or(sqs_client_res.unwrap_err());
                log::error!("Unable create SQS or S3 client: {e}");

                // Sleep to prevent infinite loops
                thread::sleep(CLIENT_FAILURE_BACKOFF);

                continue;
            }

            let sqs_client = sqs_client_res.unwrap();
            let s3_client = s3_client_res.unwrap();

            let _ = self
                .get_batch_of_messages(sqs_client, s3_client, controller)
                .await
                .inspect_err(|e| {
                    log::error!("Encountered error while handling batch of status updates: {e}");

                    // Sleep to prevent infinite loops
                    thread::sleep(MESSAGE_FAILURE_BACKOFF);
                })
                .inspect(|count| {
                    if *count > 0 {
                        log::info!("Handled {count} status updates.");
                    }
                });
        }
    }

    async fn get_batch_of_messages(
        &mut self,
        sqs_client: &aws_sdk_sqs::Client,
        s3_client: &aws_sdk_s3::Client,
        controller: &Controller,
    ) -> Result<i32, AppError> {
        if self.queue_url.is_none() {
            let url_result = sqs_client
                .get_queue_url()
                .queue_name(QUEUE_NAME)
                .send()
                .await
                .map_err(|e| {
                    if let Some(err) = e.as_service_error() {
                        return AppError::SQSError(format!("could not fetch queue url: {err}"));
                    }
                    AppError::SQSError(format!("could not fetch queue url: {e}"))
                })?;

            self.queue_url = url_result.queue_url;

            if self.queue_url.is_none() {
                return Err(AppError::SQSError(
                    "No URL found for given queue name.".to_string(),
                ));
            }
        }

        let queue_url = self.queue_url.as_ref().unwrap();

        let msgs_response = sqs_client
            .receive_message()
            .max_number_of_messages(MAX_MESSAGES)
            .queue_url(queue_url)
            .send()
            .await
            .map_err(|e| {
                if let Some(err) = e.as_service_error() {
                    return AppError::SQSError(format!("could not fetch messages: {err}"));
                }
                AppError::SQSError(format!("could not fetch messages: {e}"))
            })?;

        let mut handled_count = 0;
        for message in msgs_response.messages.unwrap_or_default() {
            let handle_opt = message.receipt_handle;
            if handle_opt.is_none() {
                return Err(AppError::SQSError(
                    "Received message with no handle.".to_string(),
                ));
            }
            let handle = handle_opt.unwrap();

            let body_raw = message.body.unwrap_or_default();
            let body_decoded = BASE64_STANDARD.decode(body_raw)?;
            let body_parsed = v1::TaskStatusUpdate::decode(&mut Cursor::new(body_decoded))?;

            let task_id = body_parsed.task_id.clone();
            log::info!("Handling status update for task (id={task_id})");

            controller
                .handle_status_update(body_parsed.clone(), s3_client)
                .await?;

            sqs_client
                .delete_message()
                .receipt_handle(handle)
                .queue_url(queue_url)
                .send()
                .await
                .map_err(|e| {
                    if let Some(err) = e.as_service_error() {
                        return AppError::SQSError(format!("could not delete message: {err}"));
                    }
                    AppError::SQSError(format!("could not delete message: {e}"))
                })?;

            handled_count += 1;
        }

        Ok(handled_count)
    }
}
