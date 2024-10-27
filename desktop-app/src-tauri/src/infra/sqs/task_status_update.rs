use base64::prelude::*;
use core::time;
use prost::Message;
use std::io::Cursor;
use std::sync::Arc;
use std::{str, thread};
use tauri::AppHandle;

use crate::biz::render_task::controller::Controller;
use crate::interface::events::emit_task_status_update_event;
use crate::{errors::AppError, spec::proto::v1};

use super::client_manager::SqsClientManager;

const CLIENT_FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(20);
const MESSAGE_FAILURE_BACKOFF: core::time::Duration = time::Duration::from_secs(3);

const MAX_MESSAGES: i32 = 5;
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
        mut manager: SqsClientManager,
        controller: &Controller,
    ) -> Result<(), AppError> {
        loop {
            let client_res = manager.get_client().await;

            if client_res.is_err() {
                let e = client_res.unwrap_err();
                log::error!("Unable create SQS client: {e}");

                // Sleep to prevent infinite loops
                thread::sleep(CLIENT_FAILURE_BACKOFF);

                continue;
            }

            let client = client_res.unwrap();

            let _ = self
                .get_batch_of_messages(client, controller)
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
        client: &aws_sdk_sqs::Client,
        controller: &Controller,
    ) -> Result<i32, AppError> {
        if self.queue_url.is_none() {
            let url_result = client
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

        let msgs_response = client
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
            let body_raw = message.body.unwrap_or_default();
            let body_decoded = BASE64_STANDARD.decode(body_raw)?;
            let body_parsed = v1::TaskStatusUpdate::decode(&mut Cursor::new(body_decoded))?;

            let task_id = body_parsed.task_id.clone();
            log::info!("Handling status update for task (id={task_id})");

            controller.handle_status_update(body_parsed.clone())?;

            client
                .delete_message()
                .receipt_handle(message.receipt_handle.unwrap_or_default())
                .queue_url(queue_url)
                .send()
                .await
                .map_err(|e| AppError::SQSError(format!("could not delete message: {e}")))?;

            emit_task_status_update_event(&self.handle, &body_parsed)?;

            handled_count += 1;
        }

        Ok(handled_count)
    }
}
