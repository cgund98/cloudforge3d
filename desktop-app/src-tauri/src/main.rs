// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cloudforge3d_lib::interface::cmd;

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
        .setup(|app| cloudforge3d_lib::init::deps::init_deps(app))
        .invoke_handler(tauri::generate_handler![
            cloudforge3d_lib::interface::greet::greet,
            cmd::settings::update_aws_credentials,
            cmd::settings::get_aws_credentials,
          ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
