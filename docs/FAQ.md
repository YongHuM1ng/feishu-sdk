# FAQ

## Why do I get `MissingAppTicket`?

You are using marketplace mode without `app_ticket`.
Set `Config::builder(...).marketplace_app().app_ticket(...)`.

## How do I use a custom HTTP client?

Inject it with either:

- `Config::builder(...).http_client(custom_client)`
- `ClientBuilder::new(config).with(with_http_client(custom_client))`

## How do I retry failed requests?

Use request options:

```rust
use std::time::Duration;
use feishu_sdk::core::RequestOptions;

let options = RequestOptions::new().retry(3, Duration::from_millis(200));
```

## How do I handle webhooks locally?

Run:

```bash
cargo run --features server --example event_server
```

Expose the local port with your preferred tunnel tool and configure callback URL in Feishu Open Platform.
