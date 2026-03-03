# Feishu SDK Rust

[![Crates.io](https://img.shields.io/crates/v/feishu-sdk.svg)](https://crates.io/crates/feishu-sdk)
[![Documentation](https://docs.rs/feishu-sdk/badge.svg)](https://docs.rs/feishu-sdk)
[![License](https://img.shields.io/crates/l/feishu-sdk.svg)](https://github.com/your-repo/feishu-sdk-rust)

> Notice: This is a community-maintained crate and is not an official Feishu/Lark SDK.

Rust SDK for Feishu/Lark Open Platform API. Provides a complete, type-safe interface to interact with Feishu APIs.

## Features

- **Complete API Coverage**: 1448+ operations from Feishu Open Platform
- **Token Management**: Automatic app_access_token and tenant_access_token handling
- **Event Processing**: Event subscription and card callback handling
- **WebSocket Support**: Long connection for real-time messaging
- **HTTP Server**: Built-in Salvo server for webhook handling
- **Type Safety**: Generated operation constants and typed APIs
- **Async/Await**: Full async support with tokio runtime
- **Pluggable Components**: Customizable cache, logger, serializer, and HTTP client
- **Code Generation**: Operation wrappers + typed model stubs from endpoint catalog

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
feishu-sdk = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Optional Features

```toml
[dependencies]
feishu-sdk = { version = "0.1", features = ["server", "websocket"] }
```

- `server`: Enable HTTP server for event handling (Salvo)
- `websocket`: Enable WebSocket client for long connections

## Quick Start

### Basic Usage

```rust
use feishu_sdk::core::{Config, FEISHU_BASE_URL};
use feishu_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder("app_id", "app_secret")
        .base_url(FEISHU_BASE_URL)
        .build();

    let client = Client::new(config)?;

    // Get chat list
    let resp = client
        .operation("im.v1.chat.list")
        .query_param("page_size", "20")
        .send()
        .await?;

    println!("Status: {}", resp.status);
    println!("Body: {}", String::from_utf8_lossy(&resp.body));

    Ok(())
}
```

### Send Message

```rust
use feishu_sdk::core::Config;
use feishu_sdk::Client;

async fn send_message(client: &Client, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let message = serde_json::json!({
        "receive_id": chat_id,
        "msg_type": "text",
        "content": r#"{"text":"Hello!"}"#
    });

    let resp = client
        .operation("im.v1.messages.create")
        .path_param("receive_id_type", "chat_id")
        .body_json(&message)?
        .send()
        .await?;

    println!("Message sent: {:?}", resp.json_value()?);
    Ok(())
}
```

### Event Handling Server

```rust
use feishu_sdk::core::{noop_logger, Config};
use feishu_sdk::event::{EventDispatcher, EventDispatcherConfig};
use feishu_sdk::server::FeishuServer;

#[tokio::main]
async fn main() {
    let event_config = EventDispatcherConfig::new()
        .verification_token("your_token");

    let dispatcher = EventDispatcher::new(event_config, noop_logger());

    // Register event handlers
    dispatcher.on_event("im.message.receive_v1", |event| {
        Box::pin(async move {
            println!("Received message: {:?}", event);
            Ok(None)
        })
    });

    // Start server
    let server = FeishuServer::new(dispatcher).port(8080);
    server.run().await;
}
```

### WebSocket Client

```rust
use feishu_sdk::core::noop_logger;
use feishu_sdk::ws::{WebSocketClient, WsConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WsConfig::new("wss://ws.feishu.cn/...")
        .heartbeat_interval(Duration::from_secs(30))
        .reconnect_delay(Duration::from_secs(5));

    let client = WebSocketClient::new(config, noop_logger()).await?;

    // Check connection status
    if client.is_connected().await {
        println!("Connected to WebSocket");
    }

    Ok(())
}
```

## Configuration

### Basic Configuration

```rust
use feishu_sdk::core::{Config, LogLevel, FEISHU_BASE_URL, LARK_BASE_URL};
use std::time::Duration;

let config = Config::builder("app_id", "app_secret")
    .base_url(FEISHU_BASE_URL)        // or LARK_BASE_URL for international
    .log_level(LogLevel::Debug)        // Set log level
    .request_timeout(Duration::from_secs(30))  // Request timeout
    .enable_token_cache(true)          // Enable token caching
    .build();
```

### Marketplace App Configuration

```rust
let config = Config::builder("app_id", "app_secret")
    .marketplace_app()                 // Enable marketplace mode
    .app_ticket("your_app_ticket")     // Required for marketplace apps
    .build();
```

### Helpdesk Configuration

```rust
let config = Config::builder("app_id", "app_secret")
    .helpdesk("helpdesk_id", "helpdesk_token")
    .build();
```

### Client Options

```rust
use feishu_sdk::client::{ClientBuilder, with_http_client, with_log_level, with_request_timeout, with_serializer};
use feishu_sdk::core::{Config, LogLevel};

let client = ClientBuilder::new(Config::builder("app_id", "app_secret").build())
    .with(with_log_level(LogLevel::Debug))
    .with(with_request_timeout(Duration::from_secs(60)))
    .with(with_http_client(custom_http_client))
    .with(with_serializer(custom_serializer))
    .build()?;
```

## API Reference

### Operation IDs

Operations use the format: `{service}.{version}.{resource}.{method}`

Examples:
- `im.v1.messages.create` - Send message
- `im.v1.chat.list` - List chats
- `contact.v3.user.get` - Get user info
- `auth.v3.tenant_access_token.internal.post` - Get tenant token

### Typed API Wrappers

```rust
use feishu_sdk::api::all_services;

// Get chat info
let builder = all_services::im::v1::chat::get(&client, "oc_xxx")
    .query_param("user_id_type", "open_id");

// List users
let builder = all_services::contact::v3::user::list(&client)
    .query_param("department_id", "0")
    .query_param("page_size", "50");
```

## Examples

See the `examples/` directory:

- `get_token.rs` - Get tenant access token
- `send_message.rs` - Send a message
- `get_user.rs` - Get user information
- `event_server.rs` - Event handling server
- `card_handler.rs` - Card callback server
- `websocket_client.rs` - WebSocket client
- `full_app.rs` - End-to-end app example (client + event + card)

Run examples:

```bash
# Set environment variables
export FEISHU_APP_ID=your_app_id
export FEISHU_APP_SECRET=your_app_secret

# Run example
cargo run --example get_token

# Run server example (requires server feature)
cargo run --features server --example event_server

# Run websocket example (requires websocket feature)
cargo run --features websocket --example websocket_client
```

## Documentation

- Usage guide: [`docs/USAGE.md`](docs/USAGE.md)
- FAQ: [`docs/FAQ.md`](docs/FAQ.md)
- Migration guide: [`docs/MIGRATION.md`](docs/MIGRATION.md)

## Error Handling

```rust
use feishu_sdk::core::Error;

match client.operation("im.v1.messages.create").send().await {
    Ok(resp) => println!("Success: {:?}", resp.json_value()?),
    Err(Error::Api(api_error)) => {
        println!("API Error: code={}, msg={}", api_error.code, api_error.msg);
    }
    Err(Error::RequestTimeout(_)) => println!("Request timed out"),
    Err(e) => println!("Error: {}", e),
}
```

## License

MIT License
