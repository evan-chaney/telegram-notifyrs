use reqwest;
use serde_json::json;
use serde_json::{Map, Value};
use std::env;
use url::Url;

pub fn send_message(msg: String) {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let chat_id: i32 = env::var("TELEGRAM_CHAT_ID")
        .expect("Missing TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("Error parsing TELEGRAM_CHAT_ID as i32");
    let conn_url = Url::parse(&format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    ))
    .expect("Error building URL for Telegram-sendMessage");

    let client = reqwest::blocking::Client::builder()
        .build()
        .expect("Error building Telegram sendMessage request client");

    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));

    client
        .post(&conn_url.to_string())
        .json(&request_body)
        .send()
        .expect("Error sending Telegram message");
    println!("Telegram message sent successfully.")
}

fn main() {
    send_message(String::from("Hello, world!"));
}
