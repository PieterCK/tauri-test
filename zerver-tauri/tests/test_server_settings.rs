use zulip_tauri_lib::server_settings;

#[test]
fn test_get_server_settings() {
    let zulip_url = "https://foo.chat.zulip.org";
    let server_config = server_settings::get_server_settings(zulip_url);
}