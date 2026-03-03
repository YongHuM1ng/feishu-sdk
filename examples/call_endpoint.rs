//! Example: Call Endpoint
//!
//! This example demonstrates how to use typed API wrappers.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx cargo run --example call_endpoint
//! ```

use feishu_sdk::Client;
use feishu_sdk::api::all_services;
use feishu_sdk::core::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FEISHU_APP_SECRET").unwrap_or_default();
    if app_id.is_empty() || app_secret.is_empty() {
        println!("Set FEISHU_APP_ID and FEISHU_APP_SECRET to run this example.");
        return Ok(());
    }

    let config = Config::builder(&app_id, &app_secret).build();
    let client = Client::new(config)?;

    let response = if let Ok(chat_id) = std::env::var("FEISHU_CHAT_ID") {
        all_services::im::v1::chat::get(&client, &chat_id)
            .query_param("user_id_type", "open_id")
            .send()
            .await?
    } else {
        all_services::im::v1::chat::list(&client)
            .query_param("page_size", "20")
            .send()
            .await?
    };

    println!("status={}", response.status);
    println!("request_id={:?}", response.request_id());
    println!("{}", String::from_utf8_lossy(&response.body));
    Ok(())
}
