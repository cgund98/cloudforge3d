// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
          tauri_plugin_log::TargetKind::LogDir {
            file_name: Some("logs".to_string()),
          },
        ))
        .build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(cloudforge3d_lib::init::deps::init_deps)
        .invoke_handler(tauri::generate_handler![cloudforge3d_lib::cmd::greet::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
