# telegram-notifyrs
[![CircleCI](https://circleci.com/gh/evan-chaney/telegram-notifyrs/tree/master.svg?style=svg&circle-token=7d362c7645d90000e2ee147558ba18ab23ff7c1e)](https://circleci.com/gh/evan-chaney/telegram-notifyrs/tree/master)

A simple way to send messages via Telegram in Rust. I found it difficult to find a Crate for sending messages via Telegram without initializing a whole bot listener so I decided to make one. Feel free to open issues/PRs!

## Usage

Include the crate under dependencies in your ```Cargo.toml```
```
[dependencies]
telegram_notifyrs = "0.1.0"
```

### Simple Example
```
use telegram_notifyrs;

telegram_notifyrs::send_message("This is my message".to_string(), "this-is-my-api-token", 1234567890);
```

### Expanded Example
```
use std::env;
use telegram_notifyrs;

fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id: i32 = env::var("TELEGRAM_CHAT_ID")
        .expect("Missing TELEGRAM_CHAT_ID environment variable")
        .parse()
        .expect("Error parsing TELEGRAM_CHAT_ID as i32");
    telegram_notifyrs::send_message("Test from library".to_string(), &token, chat_id).unwrap();
}
```
