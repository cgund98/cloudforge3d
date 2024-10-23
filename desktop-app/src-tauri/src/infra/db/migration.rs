use rusqlite_migration::{Migrations, M};

/**
 * Generate a series of migrations to be run on our SQLite datbases.
 */
pub fn gen_migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up("
            CREATE TABLE render_job (
                id VARCHAR(255) PRIMARY KEY,
                name VARCHAR(1024) NOT NULL,
                status VARCHAR(255) CHECK(status IN ('uploading', 'upload-failed', 'pending', 'running', 'succeeded', 'failed', 'canceled')) NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                queued_at DATETIME,
                completed_at DATETIME,
                file_name VARCHAR(1024) NOT NULL,
                file_size_mb FLOAT NOT NULL,
                frame_count INTEGER NOT NULL,
                frame_rate INTEGER NOT NULL,
                frame_start INTEGER NOT NULL,
                download_path VARCHAR(1024) NOT NULL,
                has_preview BOOLEAN NOT NULL DEFAULT FALSE,
                frame_rendered_count INTEGER NOT NULL DEFAULT 0
            );
            
            CREATE INDEX idx_render_job_created_at ON render_job (created_at);
        "),
        M::up("
            CREATE TABLE render_task (
                id VARCHAR(255) PRIMARY KEY,
                job_id VARCHAR(255) NOT NULL, -- Foreign key to render_job table
                frame_number INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                started_at DATETIME,
                queued_at DATETIME,
                completed_at DATETIME,
                status VARCHAR(255) CHECK(status IN ('pending', 'running', 'succeeded', 'failed')) NOT NULL,
                retry_count INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (job_id) REFERENCES render_job(id) -- Foreign key constraint
            );
        "),
    ])
}