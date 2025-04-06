use serde::{Deserialize, Serialize};
use tauri::command;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub alias: String,
    pub url: String,
    pub icon: String,
    pub zulip_version: String,
    pub zulip_feature_level: i32,
}

#[derive(Deserialize)]
struct ServerConfigZulipResponse {
    realm_name: String,
    realm_uri: String,
    realm_icon: String,
    zulip_version: Option<String>,
    zulip_feature_level: Option<i32>,
}

#[command]
pub async fn get_server_settings(domain: &str) -> Result<ServerConfig, String> {
    let client = reqwest::Client::new();

    let request = client.get(format!("{}/api/v1/server_settings", domain));

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to connect to server: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Invalid Zulip server at {}", domain));
    }

    let data: ServerConfigZulipResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse server response: {}", e))?;

    let icon = if data.realm_icon.starts_with("/") {
        format!("{}{}", data.realm_uri, data.realm_icon)
    } else {
        data.realm_icon
    };

    Ok(ServerConfig {
        alias: data.realm_name,
        icon,
        url: data.realm_uri,
        zulip_version: data.zulip_version.unwrap_or_else(|| "unknown".to_string()),
        zulip_feature_level: data.zulip_feature_level.unwrap_or(0),
    })
}
