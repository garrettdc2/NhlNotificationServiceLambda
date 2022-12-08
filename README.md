# NHL Notification Service

## About
This is a passion project used to learn Rust. The goal of the project is to create a lambda that will send a daily alert if the specified NHL team is playing that day. in current state, the generated lamba will return game data if the specified team is playing on the current date, unless another date is specified

## How to package for lambda on MAC
```
TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --target=x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/nhl_notification_service_lambda bootstrap
zip lambda.zip bootstrap
```
