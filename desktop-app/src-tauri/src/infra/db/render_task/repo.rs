use std::str::FromStr;

use rusqlite::{params, OptionalExtension};

use crate::{
    errors::AppError,
    infra::db::pool::ConnType,
};

use super::entity::{RenderTask, TaskStatus};

// Function to parse the query result and map it to a RenderTask struct
fn parse_render_task(row: &rusqlite::Row) -> Result<RenderTask, rusqlite::Error> {
    let status_str: String = row.get("status")?;
    let status = TaskStatus::from_str(&status_str).map_err(|_| rusqlite::Error::InvalidQuery)?;

    Ok(RenderTask {
        id: row.get("id")?,
        job_id: row.get("job_id")?,
        frame_number: row.get("frame_number")?,
        created_at: row.get("created_at")?,
        started_at: row.get("started_at")?,
        queued_at: row.get("queued_at")?,
        completed_at: row.get("completed_at")?,
        status,
        retry_count: row.get("retry_count")?,
    })
}

// Fetch a single task by its primary key
pub fn get_by_id(conn: &ConnType, id: &str) -> Result<Option<RenderTask>, AppError> {
    let result = conn
        .query_row(
            "SELECT * FROM render_task WHERE id = ?1",
            rusqlite::params![id],
            parse_render_task,
        )
        .optional()?;

    Ok(result)
}

// Fetch a list of tasks belonging to a job
pub fn list_by_job_id(conn: &ConnType, job_id: &str) -> Result<Vec<RenderTask>, AppError> {
    let mut stmt = conn.prepare(
        "
            SELECT
                *
            FROM
                render_task
            WHERE job_id = ?1
            ORDER BY frame_number ASC
        ",
    )?;

    // Map result
    let ent_iter = stmt.query_map([job_id], parse_render_task)?;

    let mut entries: Vec<RenderTask> = Vec::new();
    for db_entry in ent_iter {
        let entry = db_entry?;

        entries.push(entry);
    }

    Ok(entries)
}

// Count the number of unfinished tasks
pub fn count_unfinished_tasks(
    conn: &ConnType,
    job_id: &str,
    exclude_id: &str,
) -> Result<u32, AppError> {
    let count: u32 = conn.query_row(
        "
                SELECT COUNT(*) as count
                FROM render_task
                WHERE job_id = ?1 AND id != ?2 AND status IN ('pending', 'running')
            ",
        rusqlite::params![job_id, exclude_id],
        |r| {
            let count: u32 = r.get("count")?;
            Ok(count)
        },
    )?;

    Ok(count)
}

// Upsert a single render task
pub fn save(tx: &rusqlite::Transaction, task: &RenderTask) -> Result<(), AppError> {
    tx.execute(
        "INSERT INTO render_task (
                id,
                job_id,
                frame_number,
                created_at,
                started_at,
                queued_at,
                completed_at,
                status,
                retry_count
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            ON CONFLICT(id) DO UPDATE SET
                job_id = excluded.job_id,
                frame_number = excluded.frame_number,
                created_at = excluded.created_at,
                started_at = excluded.started_at,
                queued_at = excluded.queued_at,
                completed_at = excluded.completed_at,
                status = excluded.status,
                retry_count = excluded.retry_count;",
        params![
            task.id,
            task.job_id,
            task.frame_number,
            task.created_at,
            task.started_at,
            task.queued_at,
            task.completed_at,
            task.status.to_string(),
            task.retry_count
        ],
    )?;

    Ok(())
}
