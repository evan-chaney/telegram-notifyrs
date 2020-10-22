use serde_json::json;
use serde_json::{Map, Value};
use ureq;

pub fn send_message(msg: String, token: &str, chat_id: i32) -> ureq::Response {
    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));

    let resp = ureq::post(&format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    ))
    .send_json(json!(request_body));
    return resp;
}
