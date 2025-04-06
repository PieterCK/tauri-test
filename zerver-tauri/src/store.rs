use crate::server_settings::ServerConfig;
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct AppData {
    domains: HashMap<String, ServerConfig>,
}
pub type AppState = Mutex<AppData>;

#[tauri::command]
pub async fn get_domains(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let app_data = state.lock().unwrap();

    Ok(app_data.clone())
}
