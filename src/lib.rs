use reqwest;
use serde_json::json;
use serde_json::{Map, Value};
use url::Url;

pub fn send_message(
    msg: String,
    token: &str,
    chat_id: i32,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let conn_url = Url::parse(&format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    ))
    .unwrap();
    //.expect("Error building URL for Telegram-sendMessage");

    let client = reqwest::blocking::Client::builder().build().unwrap();
    //.expect("Error building Telegram sendMessage request client");

    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));

    let res = client
        .post(&conn_url.to_string())
        .json(&request_body)
        .send();
    return res;
}
