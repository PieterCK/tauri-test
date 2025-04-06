pub mod server_settings;
pub mod store;
use std::sync::Mutex;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(store::AppData::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            server_settings::get_server_settings,
            store::get_domains
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
