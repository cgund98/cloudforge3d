use std::{str::FromStr, sync::Arc};

use rusqlite::OptionalExtension;

use crate::errors::AppError;
use crate::infra::db::pool::PoolType;
use super::entity::{JobStatus, RenderJob};

// Parse JobStatus from a sqlite query row
fn parse_status_string(row: &rusqlite::Row) -> Result<JobStatus, rusqlite::Error> {
    let status_str: String = row.get("status")?; // Get status as a string
    let status = JobStatus::from_str(&status_str)
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    Ok(status)
}

// Parse RenderJob from a sqlite query row
fn parse_render_job(row: &rusqlite::Row) -> Result<RenderJob, rusqlite::Error> {
    Ok(RenderJob {
        id: row.get("id")?,
        name: row.get("name")?,
        status: parse_status_string(row)?,
        created_at: row.get("created_at")?,
        queued_at: row.get("queued_at")?,
        completed_at: row.get("completed_at")?,
        file_name: row.get("file_name")?,
        file_size_mb: row.get("file_size_mb")?,
        frame_count: row.get("frame_count")?,
        frame_rate: row.get("frame_rate")?,
        frame_start: row.get("frame_start")?,
        download_path: row.get("download_path")?,
        has_preview: row.get("has_preview")?,
        frame_rendered_count: row.get("frame_rendered_count")?,
    })
}

pub struct Repo {
    pool: Arc<PoolType>,
}

impl Repo {
    pub fn new(pool: Arc<PoolType>) -> Repo {
        Repo { pool }
    }

    // Fetch a RenderJob by its primary key
    pub fn get_by_id(&self, id: &str) -> Result<RenderJob, AppError> {
        let conn = self.pool.get()?;

        let result = conn.query_row(
            "SELECT * FROM render_job WHERE id = ?1",
            rusqlite::params![id],
            parse_render_job,
        ).optional()?;

        match result {
            Some(ent) => Ok(ent),
            None => Err(AppError::NotFound(format!("id = '{}'", id,))),
        }

    }

    // Fetch a page of RenderJob entities. 
    // Order by creation date in reverse.
    pub fn list(&self, limit: u32, offset: u32) -> Result<Vec<RenderJob>, AppError> {
        let conn = self.pool.get()?;

        let mut stmt = conn.prepare("
            SELECT 
                *
            FROM 
                render_job
            ORDER BY created_at DESC
            LIMIT ?
            OFFSET ?
        ")?;

        // Map result
        let ent_iter = stmt.query_map([limit, offset], parse_render_job)?;

        let mut entries: Vec<RenderJob> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok(entries)
    }

    // Persist a RenderJob to the database
    pub fn save(&self, job: RenderJob) -> Result<RenderJob, AppError> {
        let conn = self.pool.get()?;

        // SQL statement
        let sql: &str = "
        INSERT INTO render_job (
            id, 
            name, 
            status, 
            created_at, 
            queued_at, 
            completed_at, 
            file_name, 
            file_size_mb, 
            frame_count, 
            frame_rate, 
            frame_start, 
            download_path, 
            has_preview, 
            frame_rendered_count
        ) 
        VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        ON CONFLICT(id) DO UPDATE SET 
            name = excluded.name,
            status = excluded.status,
            created_at = excluded.created_at,
            queued_at = excluded.queued_at,
            completed_at = excluded.completed_at,
            file_name = excluded.file_name,
            file_size_mb = excluded.file_size_mb,
            frame_count = excluded.frame_count,
            frame_rate = excluded.frame_rate,
            frame_start = excluded.frame_start,
            download_path = excluded.download_path,
            has_preview = excluded.has_preview,
            frame_rendered_count = excluded.frame_rendered_count;
        ";

        // Execute query
        conn.execute(
            sql,
            (
                &job.id, &job.name, &job.status.to_string(), &job.created_at, &job.queued_at, 
                &job.completed_at, &job.file_name, &job.file_size_mb, &job.frame_count, 
                &job.frame_rate, &job.frame_rate, &job.frame_start, &job.download_path, 
                &job.has_preview, &job.frame_rendered_count
            ),
        )?;

        let new_job = self.get_by_id(&job.id)?;
        Ok(new_job)
    }
}