# telegram-notifyrs
[![CircleCI](https://circleci.com/gh/evan-chaney/telegram-notifyrs/tree/master.svg?style=svg&circle-token=7d362c7645d90000e2ee147558ba18ab23ff7c1e)](https://circleci.com/gh/evan-chaney/telegram-notifyrs/tree/master)

A simple way to send messages via Telegram in Rust. I found it difficult to find a Crate for sending messages via Telegram without initializing a whole bot listener so I decided to make one.

## Usage

Include the crate under dependencies in your ```Cargo.toml```
```
[dependencies]
telegram_notifyrs = "0.1.0"
```

Use it from your application
```
use telegram_notifyrs;

telegram_notifyrs::send_message("This is my message".to_string(), "this-is-my-api-token", 1234567890);
```
