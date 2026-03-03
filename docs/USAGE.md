# Usage Guide

## 1. Configure Client

```rust
use feishu_sdk::core::{Config, FEISHU_BASE_URL};
use feishu_sdk::Client;

let config = Config::builder("app_id", "app_secret")
    .base_url(FEISHU_BASE_URL)
    .build();
let client = Client::new(config)?;
```

## 2. Call Generic Operations

```rust
let resp = client
    .operation("im.v1.chat.list")
    .query_param("page_size", "20")
    .send()
    .await?;
```

## 3. Call Typed Wrappers

```rust
use feishu_sdk::api::all_services;

let resp = all_services::contact::v3::user::get(&client, "ou_xxx")
    .query_param("user_id_type", "open_id")
    .send()
    .await?;
```

## 4. Enable Optional Features

- `server`: event + card HTTP endpoints.
- `websocket`: long-connection client.

```bash
cargo run --features server --example event_server
cargo run --features websocket --example websocket_client
```

## 5. Marketplace App

```rust
let config = Config::builder("app_id", "app_secret")
    .marketplace_app()
    .app_ticket("app_ticket")
    .build();
```
