# Migration Guide

## From Go `oapi-sdk-go` to this Rust SDK

## 1. Client Construction

Go:

```go
client := lark.NewClient(appID, appSecret)
```

Rust:

```rust
let config = Config::builder(app_id, app_secret).build();
let client = Client::new(config)?;
```

## 2. API Calls

Go methods map to operation IDs:

```rust
client.operation("im.v1.chat.get")
    .path_param("chat_id", "oc_xxx")
    .send()
    .await?;
```

Typed wrappers are also available in `feishu_sdk::api::all_services`.

## 3. Token and Cache

Token cache is enabled by default.
For custom cache logic, inject a `Cache` implementation with `with_token_cache`.

## 4. Events and Cards

Use `EventDispatcher` and `FeishuServer` for webhook handling.
Card callbacks are supported via `CardActionHandler`.

## 5. Optional Components

- Use `server` feature for webhook server.
- Use `websocket` feature for long-connection client.
