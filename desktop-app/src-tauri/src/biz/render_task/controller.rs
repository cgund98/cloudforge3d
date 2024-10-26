use std::sync::Arc;

use crate::{
    errors::AppError,
    infra::db::{
        decorator::{with_conn, with_transaction},
        pool::PoolType,
        render_job, render_task,
    },
    spec::proto::v1,
};

pub struct Controller {
    pool: Arc<PoolType>,
}

impl Controller {
    pub fn new(pool: Arc<PoolType>) -> Controller {
        Controller { pool }
    }

    pub fn handle_status_update(&self, input: v1::TaskStatusUpdate) -> Result<(), AppError> {
        let task_id = input.task_id.to_string();
        let task_result = with_conn(&self.pool, |conn| {
            render_task::repo::get_by_id(conn, &task_id)
        })?;

        if task_result.is_none() {
            log::info!("Task does not exist. Will not update status.");
            return Ok(());
        }

        let mut task = task_result.unwrap();

        // Update task fields
        task.status = render_task::entity::TaskStatus::from(input.status());

        let now = chrono::offset::Utc::now();
        match input.status() {
            v1::TaskStatus::Unspecified => (),
            v1::TaskStatus::Pending => (),
            v1::TaskStatus::Running => task.started_at = Some(now),
            v1::TaskStatus::Failed => task.completed_at = Some(now),
            v1::TaskStatus::Succeeded => task.completed_at = Some(now),
        }

        // Fetch job
        let mut job = with_conn(&self.pool, |conn| {
            render_job::repo::get_by_id(conn, &task.job_id)
        })?
        .unwrap();

        // See if other tasks are still running
        let remaining_task_count = with_conn(&self.pool, |conn| {
            render_task::repo::count_unfinished_tasks(conn, &job.id, &task.id)
        })?;
        let other_tasks_complete = remaining_task_count == 0;

        // Update job fields
        let job_failed = job.status == render_job::entity::JobStatus::Failed;
        if task.status == render_task::entity::TaskStatus::Failed {
            job.status = render_job::entity::JobStatus::Failed;
        } else if task.status == render_task::entity::TaskStatus::Running && !job_failed {
            job.status = render_job::entity::JobStatus::Running;
        } else if task.status == render_task::entity::TaskStatus::Succeeded {
            job.frame_rendered_count += 1;

            if other_tasks_complete {
                job.status = render_job::entity::JobStatus::Succeeded;
            }
        }

        with_transaction(&self.pool, |tx| {
            render_task::repo::save(tx, &task)?;
            render_job::repo::save(tx, &job)?;
            Ok(())
        })?;

        Ok(())
    }
}
