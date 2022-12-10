# NHL Notification Service

## About
This is a passion project used to learn Rust. Holds code for a lambda that will send a daily alert if the specified NHL team is playing that day. relies on env vars to determine team, timezone, and destination webhook url.

## How to package for lambda on MAC
```
TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --target=x86_64-unknown-linux-musl --release && \
cp target/x86_64-unknown-linux-musl/release/nhl_notification_service_lambda bootstrap && \
zip lambda.zip bootstrap
```

## Infrastructure
Infrastructure is tracked as code via cdk. Code can be view [here](https://github.com/garrettdc2/NhlNotificationServiceCdk)
