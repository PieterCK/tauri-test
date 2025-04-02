use serde_json::{from_str, Value};
use std::fs;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn mock_zulip_server(
    response_body: &Value,
    status_code: u16,
    endpoint: &str,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(endpoint))
        .respond_with(ResponseTemplate::new(status_code).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    mock_server
}

pub fn get_json_fixture_data(filename: &str) -> Value {
    let path = &format!("./tests/fixtures/{}", filename);
    let response_body: String =
        fs::read_to_string(path).expect(&format!("Fail to find file. Path: {}", path));
    let json: Value = from_str(&response_body).expect("Fail to read json file");
    json
}

pub async fn mock_default_server_settings_endpoint() -> MockServer {
    let json = get_json_fixture_data("server_settings_complete_response.json");
    mock_zulip_server(&json, 200, "/api/v1/server_settings").await
}
