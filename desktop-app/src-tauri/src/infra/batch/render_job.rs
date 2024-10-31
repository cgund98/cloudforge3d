use aws_sdk_batch::types::{JobStatus, KeyValuesPair};

use super::constants::QUEUE_NAME;

use crate::errors::AppError;

pub async fn list_incomplete_jobs(
    client: &aws_sdk_batch::Client,
    job_id: &str,
) -> Result<Vec<String>, AppError> {
    let job_name_search = "cf3d-".to_string() + job_id + "*";
    log::info!("Searching for jobs matching name '{job_name_search}'");

    let filters = KeyValuesPair::builder()
        .name("JOB_NAME")
        .values(&job_name_search)
        .build();

    let query_res = client
        .list_jobs()
        .filters(filters)
        .job_queue(QUEUE_NAME)
        .into_paginator()
        .send()
        .try_collect()
        .await
        .map_err(|e| {
            if let Some(err) = e.as_service_error() {
                return AppError::BatchError(format!("{err}"));
            }
            AppError::BatchError(format!("{e}"))
        })?
        .into_iter()
        .flat_map(|o| o.job_summary_list.unwrap_or_default())
        .collect::<Vec<_>>();

    let job_count = query_res.len();
    log::info!("Found {job_count} jobs.");

    let job_ids = query_res
        .into_iter()
        .filter(|sum| {
            sum.status
                .as_ref()
                .map(|s| !matches!(s, JobStatus::Failed | JobStatus::Succeeded))
                .unwrap_or(false)
        })
        .flat_map(|sum| sum.job_id)
        .collect::<Vec<_>>();

    let filtered_count = job_ids.len();
    log::info!("Filtered to {filtered_count} unfinished jobs.");

    Ok(job_ids)
}

pub async fn cancel_jobs(
    client: &aws_sdk_batch::Client,
    batch_job_ids: Vec<String>,
) -> Result<(), AppError> {
    for batch_job_id in batch_job_ids {
        client
            .cancel_job()
            .job_id(batch_job_id)
            .reason("Cancelled by user")
            .send()
            .await
            .map_err(|e| {
                if let Some(err) = e.as_service_error() {
                    return AppError::BatchError(format!("{err}"));
                }
                AppError::BatchError(format!("{e}"))
            })?;
    }

    Ok(())
}
