//! Example: Event Handling Server
//!
//! This example demonstrates how to handle events using the SDK with Salvo server.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx FEISHU_VERIFICATION_TOKEN=xxx cargo run --features server --example event_server
//! ```

use std::future::Future;
use std::pin::Pin;

use feishu_sdk::core::{Config, Error, FEISHU_BASE_URL, noop_logger};
use feishu_sdk::event::{Event, EventDispatcher, EventDispatcherConfig, EventHandler, EventResp};
use feishu_sdk::server::FeishuServer;

struct MessageHandler;

impl EventHandler for MessageHandler {
    fn event_type(&self) -> &str {
        "im.message.receive_v1"
    }

    fn handle(
        &self,
        event: Event,
    ) -> Pin<Box<dyn Future<Output = Result<Option<EventResp>, Error>> + Send + '_>> {
        Box::pin(async move {
            println!("Received message event: {:?}", event);
            Ok(None)
        })
    }
}

struct UserCreatedHandler;

impl EventHandler for UserCreatedHandler {
    fn event_type(&self) -> &str {
        "contact.user.created_v3"
    }

    fn handle(
        &self,
        event: Event,
    ) -> Pin<Box<dyn Future<Output = Result<Option<EventResp>, Error>> + Send + '_>> {
        Box::pin(async move {
            println!("User created event: {:?}", event);
            Ok(None)
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").expect("FEISHU_APP_ID not set");
    let app_secret = std::env::var("FEISHU_APP_SECRET").expect("FEISHU_APP_SECRET not set");
    let verification_token =
        std::env::var("FEISHU_VERIFICATION_TOKEN").expect("FEISHU_VERIFICATION_TOKEN not set");
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let _config = Config::builder(&app_id, &app_secret)
        .base_url(FEISHU_BASE_URL)
        .build();

    let event_config = EventDispatcherConfig::new().verification_token(verification_token);

    let dispatcher = EventDispatcher::new(event_config, noop_logger());

    // Register event handlers
    dispatcher.register_handler(Box::new(MessageHandler)).await;
    dispatcher
        .register_handler(Box::new(UserCreatedHandler))
        .await;

    let server = FeishuServer::new(dispatcher).port(port);

    println!("Starting event server on port {}...", port);
    println!("Webhook endpoint: http://localhost:{}/webhook/event", port);

    server.run().await;

    Ok(())
}
