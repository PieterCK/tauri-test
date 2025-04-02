use serde_json::json;
use zulip_tauri_lib::server_settings::{self, ServerConfig};
mod common;
use std::error::Error;
use wiremock::MockServer;

#[tokio::test]
async fn test_get_server_settings() -> Result<(), Box<dyn Error>> {
    let mock_server: MockServer = common::mock_default_server_settings_endpoint().await;
    let response_result: Result<ServerConfig, String> =
        server_settings::get_server_settings(&mock_server.uri()).await;
    let server_config: ServerConfig = match response_result {
        Ok(server_config) => server_config,
        Err(error_message) => panic!("{}", error_message),
    };

    assert_eq!(server_config.alias, "Zulip Dev");
    assert_eq!(server_config.url, "https://chat.zulip.org");
    assert_eq!(
        server_config.icon,
        "https://secure.gravatar.com/avatar/62429d594b6ffc712f54aee976a18b44?d=identicon"
    );
    assert_eq!(server_config.zulip_version, "5.0-dev-1650-gc3fd37755f");
    assert_eq!(server_config.zulip_feature_level, 500);

    Ok(())
}

#[tokio::test]
async fn test_reformat_response_data() -> Result<(), Box<dyn Error>> {
    let json = json!({
        "realm_name": "Zulip Dev",
        "realm_uri": "https://chat.zulip.org",
        // simulate local link for icon.
        "realm_icon":"/avatar/62429d594b6ffc712f54aee976a18b44?d=identicon",
        // no "zulip_version" field.
        // no "zulip_feature_level" field.
    });
    let mock_server = common::mock_zulip_server(&json, 200, "/api/v1/server_settings").await;
    let response_result: Result<ServerConfig, String> =
        server_settings::get_server_settings(&mock_server.uri()).await;
    let server_config: ServerConfig = match response_result {
        Ok(server_config) => server_config,
        Err(error_message) => panic!("{}", error_message),
    };

    assert_eq!(server_config.alias, "Zulip Dev");
    assert_eq!(server_config.url, "https://chat.zulip.org");
    // reformat realm_icon's URI with the realm's URI.
    assert_eq!(
        server_config.icon,
        format!("{}/avatar/62429d594b6ffc712f54aee976a18b44?d=identicon", server_config.url),
    );
    assert_eq!(server_config.zulip_version, "unknown");
    assert_eq!(server_config.zulip_feature_level, 0);

    Ok(())
}

#[tokio::test]
async fn test_failed_requests() -> Result<(), Box<dyn Error>> {
    // simulate server not available.
    let response_result: Result<ServerConfig, String> =
        server_settings::get_server_settings("http://localhost:0").await;
    assert!(response_result.is_err());
    assert_eq!(
        response_result.unwrap_err(),
        "Failed to connect to server: error sending request for url (http://localhost:0/api/v1/server_settings)",
    );

    // simulate invalid server.
    let json = common::get_json_fixture_data("server_settings_complete_response.json");
    let mock_server = common::mock_zulip_server(&json, 503, "/api/v1/server_settings").await;

    let response_result: Result<ServerConfig, String> =
        server_settings::get_server_settings(&mock_server.uri()).await;
    assert!(response_result.is_err());
    assert_eq!(
        response_result.unwrap_err(),
        format!("Invalid Zulip server at {}", mock_server.uri()),
    );

    Ok(())
}

#[tokio::test]
async fn test_unexpected_response() -> Result<(), Box<dyn Error>> {
    // simulate response with missing expected fields.
    let json = json!({
        "realm_name": "Zulip"
    });
    let mock_server = common::mock_zulip_server(&json, 200, "/api/v1/server_settings").await;

    let response_result: Result<ServerConfig, String> =
        server_settings::get_server_settings(&mock_server.uri()).await;
    assert!(response_result.is_err());
    assert_eq!(
        response_result.unwrap_err(),
        "Failed to parse server response: error decoding response body",
    );

    Ok(())
}
