
// Type aliases
type InitResult = Result<(), Box<dyn std::error::Error + 'static>>;


use crate::infra::db::init::init_db;

// Dependency initializes methods
pub fn init_deps(app: &mut tauri::App) -> InitResult {
    // Initialize tables
    let pool = init_db(app).unwrap();

    Ok(())
}