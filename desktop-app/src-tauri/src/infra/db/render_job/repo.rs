use std::{str::FromStr, sync::Arc};

use rusqlite::OptionalExtension;

use crate::{errors::AppError, infra::db::pool::ConnType};
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

    // Fetch a RenderJob by its primary key
    pub fn get_by_id(conn: &ConnType, id: &str) -> Result<Option<RenderJob>, AppError> {
        let result = conn.query_row(
            "SELECT * FROM render_job WHERE id = ?1",
            rusqlite::params![id],
            parse_render_job,
        ).optional()?;

        Ok(result)
    }

    // Fetch a page of RenderJob entities. 
    // Order by creation date in reverse.
    pub fn list(conn: &ConnType, limit: u32, offset: u32) -> Result<(Vec<RenderJob>, u32), AppError> {

        let count: u32 = conn.query_row("SELECT COUNT(*) as count FROM render_job", (), |r| {
            let count: u32 = r.get("count")?;
            Ok(count)
        })?;

        let mut sel_stmt = conn.prepare("
            SELECT 
                *
            FROM 
                render_job
            ORDER BY created_at DESC
            LIMIT ?
            OFFSET ?
        ")?;

        // Map result
        let ent_iter = sel_stmt.query_map([limit, offset], parse_render_job)?;

        let mut entries: Vec<RenderJob> = Vec::new();
        for db_entry in ent_iter {
            let entry = db_entry?;

            entries.push(entry);
        }

        Ok((entries, count))
    }

    // Persist a RenderJob to the database
    pub fn save(tx: &rusqlite::Transaction, job: RenderJob) -> Result<(), AppError> {
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
        tx.execute(
            sql,
            (
                &job.id, &job.name, &job.status.to_string(), &job.created_at, &job.queued_at, 
                &job.completed_at, &job.file_name, &job.file_size_mb, &job.frame_count, 
                &job.frame_rate, &job.frame_start, &job.download_path, 
                &job.has_preview, &job.frame_rendered_count
            ),
        )?;

        Ok(())
    }