//! Example: Card Action Handler
//!
//! This example demonstrates how to handle card actions using the SDK with Salvo server.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx FEISHU_VERIFICATION_TOKEN=xxx cargo run --features server --example card_handler
//! ```

use feishu_sdk::card::CardActionHandler;
use feishu_sdk::core::{Config, FEISHU_BASE_URL, noop_logger};
use feishu_sdk::event::EventDispatcherConfig;
use feishu_sdk::server::FeishuServer;

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

    let card_handler = CardActionHandler::new(noop_logger())
        .verification_token(verification_token.clone())
        .handler(|action| {
            Box::pin(async move {
                println!("Received card action: {:?}", action);

                if let Some(action_value) = &action.action {
                    println!("  Action value: {:?}", action_value);
                }

                Ok(None)
            })
        });

    let event_config = EventDispatcherConfig::new().verification_token(verification_token);

    let dispatcher = feishu_sdk::event::EventDispatcher::new(event_config, noop_logger());

    let server = FeishuServer::new(dispatcher)
        .card_handler(card_handler)
        .port(port);

    println!("Starting card handler server on port {}...", port);
    println!(
        "Card webhook endpoint: http://localhost:{}/webhook/card",
        port
    );

    server.run().await;

    Ok(())
}
