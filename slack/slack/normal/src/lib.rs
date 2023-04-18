use slack_flows::{listen_to_channel, send_message_to_channel};
use std::env;

#[no_mangle]
pub fn run() {
    let login: String = match env::var("login") {
        Err(_) => "jyccloud".to_string(),
        Ok(name) => name,
    };
    listen_to_channel(&login, "localtest", |sm| {
        send_message_to_channel(&login, "localtest", format!("Hello, {}", sm.text));
    });
}
