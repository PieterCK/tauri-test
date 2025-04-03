use std::fs;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::{Value, from_str};

pub async fn mock_zulip_server(
    response_body: Value,
    status_code: u16,
) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/server_settings"))
        .respond_with(ResponseTemplate::new(status_code).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    mock_server
}

pub fn get_json_fixture_data(filename: &str) -> Value {
    let path = &format!("./tests/fixtures/{}", filename);
    let response_body: String = fs::read_to_string(path).expect(&format!("Fail to find file. Path: {}", path));
    let json: Value = from_str(&response_body).expect("Fail to read json file");
    json
}

pub async fn default_mock_server() -> MockServer {
    let json = get_json_fixture_data("server_settings_complete_response.json");
    mock_zulip_server(
        json,
        200
    )
    .await
}