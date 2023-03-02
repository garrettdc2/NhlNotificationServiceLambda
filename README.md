# NHL Notification Service

[![MIT licensed][mit-badge]][mit-url]
[![build Status][build-badge]][build-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/garrettdc2/NhlNotificationServiceLambda/blob/main/LICENSE
[build-badge]: https://github.com/garrettdc2/NhlNotificationServiceLambda/actions/workflows/build.yml/badge.svg?branch=main
[build-url]: https://github.com/garrettdc2/NhlNotificationServiceLambda/actions/workflows/build.yml

## About
This is a passion project used to learn Rust. Holds code for a lambda that will send a daily alert if the specified NHL team is playing that day. Relies on env vars to determine team, timezone, and destination webhook url.

## How to package for lambda on MAC
First install `musl-cross` using homebrew so we can cross compile to linux:
```
brew install filosottile/musl-cross/musl-cross
```

Now, run the following command to compile to linux and place the output in the location the CDK package expects:
```
TARGET_CC=x86_64-linux-musl-gcc RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" cargo build --target=x86_64-unknown-linux-musl --release && \
cp target/x86_64-unknown-linux-musl/release/nhl_notification_service_lambda bootstrap && \
zip lambda.zip bootstrap
```

## Infrastructure
Infrastructure is tracked as code via cdk. Code can be view [here](https://github.com/garrettdc2/NhlNotificationServiceCdk)
