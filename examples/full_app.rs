//! Example: Full Application
//!
//! This example wires client + event dispatcher + card handler into one process.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx FEISHU_VERIFICATION_TOKEN=xxx cargo run --features server --example full_app
//! ```

use std::future::Future;
use std::pin::Pin;

use feishu_sdk::Client;
use feishu_sdk::card::CardActionHandler;
use feishu_sdk::core::{Config, Error, FEISHU_BASE_URL, noop_logger};
use feishu_sdk::event::{Event, EventDispatcher, EventDispatcherConfig, EventHandler, EventResp};
use feishu_sdk::server::FeishuServer;

struct MessageEventHandler {
    client: Client,
}

impl EventHandler for MessageEventHandler {
    fn event_type(&self) -> &str {
        "im.message.receive_v1"
    }

    fn handle(
        &self,
        event: Event,
    ) -> Pin<Box<dyn Future<Output = Result<Option<EventResp>, Error>> + Send + '_>> {
        Box::pin(async move {
            println!("Received event: {:?}", event.header);

            let _ = self
                .client
                .operation("im.v1.chat.list")
                .query_param("page_size", "1")
                .send()
                .await;
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

    let config = Config::builder(&app_id, &app_secret)
        .base_url(FEISHU_BASE_URL)
        .build();
    let client = Client::new(config)?;

    let event_config = EventDispatcherConfig::new().verification_token(verification_token.clone());
    let dispatcher = EventDispatcher::new(event_config, noop_logger());
    dispatcher
        .register_handler(Box::new(MessageEventHandler {
            client: client.clone(),
        }))
        .await;

    let card_handler = CardActionHandler::new(noop_logger())
        .verification_token(verification_token)
        .handler(|action| {
            Box::pin(async move {
                println!("Received card action: {:?}", action.action);
                Ok(None)
            })
        });

    let server = FeishuServer::new(dispatcher)
        .card_handler(card_handler)
        .port(port);

    println!("Full app server listening on 0.0.0.0:{port}");
    server.run().await;
    Ok(())
}
