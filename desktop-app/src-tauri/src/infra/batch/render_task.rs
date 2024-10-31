use aws_sdk_batch::types::KeyValuePair;

use super::constants::{JOB_DEF_NAME, QUEUE_NAME};
use crate::{errors::AppError, infra::db::render_task::entity::RenderTask};

// Submit a task job
pub async fn submit_job(client: &aws_sdk_batch::Client, vcpu: u8, memory_mib: u32, task: &RenderTask) -> Result<(), AppError> {
    let job_id = task.job_id.clone();
    let task_id = task.id.clone();

    let job_name = format!("cf3d-{task_id}");

    let environment: Vec<KeyValuePair> = vec![
        KeyValuePair::builder()
            .name("CF3D_JOB_ID".to_string())
            .value(job_id)
            .build(),
        KeyValuePair::builder()
            .name("CF3D_TASK_ID".to_string())
            .value(task_id)
            .build(),
        KeyValuePair::builder()
            .name("CF3D_FRAME_NUMBER".to_string())
            .value(task.frame_number.to_string())
            .build(),
    ];

    let resources = vec![
        aws_sdk_batch::types::ResourceRequirement::builder()
            .set_type(Some(aws_sdk_batch::types::ResourceType::Vcpu))
            .set_value(Some(vcpu.to_string()))
            .build(),
        aws_sdk_batch::types::ResourceRequirement::builder()
            .set_type(Some(aws_sdk_batch::types::ResourceType::Memory))
            .set_value(Some(memory_mib.to_string()))
            .build(),
    ];

    let overrides = aws_sdk_batch::types::ContainerOverrides::builder()
        .set_environment(Some(environment))
        .set_resource_requirements(Some(resources))
        .build();

    let _res = client
        .submit_job()
        .job_name(job_name)
        .job_queue(QUEUE_NAME)
        .job_definition(JOB_DEF_NAME)
        .container_overrides(overrides)
        .send()
        .await
        .map_err(|e| {
            if let Some(err) = e.as_service_error() {
                return AppError::BatchError(format!("{err}"));
            }
            AppError::BatchError(format!("{e}"))
        })?;

    Ok(())
}
