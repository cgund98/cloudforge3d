use std::{sync::Arc, time::Duration};

use r2d2_sqlite::SqliteConnectionManager;
use tauri::Manager;

use super::pool::PoolType;
use super::migration::gen_migrations;

pub fn init_db(app: &mut tauri::App) -> Result<Arc<PoolType>, Box<dyn std::error::Error + 'static>> {
    let handle = app.handle();

    // Find app path
    let binding = handle.path().app_data_dir().unwrap();
    let data_path = binding.as_path();

    // Create path if it doesn't exist
    if !data_path.exists() {
        std::fs::create_dir_all(data_path)?;
    }

    let db_file = data_path.join("main.db");

    // Generate migrations
    let migrations = gen_migrations();

    // Run migrations
    let mut conn = rusqlite::Connection::open(db_file.clone())?;
    migrations
    .to_latest(&mut conn)
    .inspect_err(|f| log::error!("Encountered error when running migrations: {f}"))?;
    drop(conn);


    // Initialize connection manager
    log::info!("Initializing sqlite database at {db_file:?}...");
    let manager = SqliteConnectionManager::file(db_file).with_init(
        move |c| {
            c.pragma_update(None, "foreign_keys", "ON")
                .and_then(|_| c.pragma_update(None, "synchronous", "NORMAL"))
                .and_then(|_| c.pragma_update(None, "journal_mode", "WAL"))
                .inspect_err(|f| log::error!("Encountered error when setting pragma values: {f}"))
        }
    );

    // Initialize connection pool
    let builder = r2d2::Pool::builder();
    let pool = builder
        .connection_timeout(Duration::from_secs(10))
        .build(manager)
        .unwrap();

    Ok(Arc::new(pool))
}