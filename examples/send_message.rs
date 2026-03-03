//! Example: Send Message
//!
//! This example demonstrates how to send a message using the SDK.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx cargo run --example send_message
//! ```

use feishu_sdk::core::{Config, FEISHU_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").expect("FEISHU_APP_ID not set");
    let app_secret = std::env::var("FEISHU_APP_SECRET").expect("FEISHU_APP_SECRET not set");
    let receive_id = std::env::var("RECEIVE_ID").unwrap_or_else(|_| "oc_xxx".to_string());

    let config = Config::builder(&app_id, &app_secret)
        .base_url(FEISHU_BASE_URL)
        .build();

    let client = feishu_sdk::Client::new(config)?;

    println!("Sending message to: {}", receive_id);

    let message = serde_json::json!({
        "receive_id": receive_id,
        "msg_type": "text",
        "content": r#"{"text":"Hello from Feishu SDK Rust!"}"#
    });

    let response = client
        .operation("im.v1.messages.create")
        .path_param("receive_id_type", "chat_id")
        .body_json(&message)?
        .send()
        .await?;

    if response.status == 200 {
        let json = response.json_value()?;
        println!("Message sent successfully!");
        if let Some(msg_id) = json.get("data").and_then(|d| d.get("message_id")) {
            println!("Message ID: {}", msg_id);
        }
    } else {
        let body = String::from_utf8_lossy(&response.body);
        println!(
            "Failed to send message. Status: {}, Body: {}",
            response.status, body
        );
    }

    Ok(())
}
