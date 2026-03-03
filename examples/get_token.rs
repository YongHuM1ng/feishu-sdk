//! Example: Get Tenant Access Token
//!
//! This example demonstrates how to get a tenant access token using the SDK.
//!
//! ```bash
//! cargo run --example get_token
//! ```

use feishu_sdk::core::{Config, FEISHU_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").expect("FEISHU_APP_ID not set");
    let app_secret = std::env::var("FEISHU_APP_SECRET").expect("FEISHU_APP_SECRET not set");

    let config = Config::builder(&app_id, &app_secret)
        .base_url(FEISHU_BASE_URL)
        .build();

    let client = feishu_sdk::Client::new(config)?;

    println!("Getting tenant access token...");

    let response = client
        .operation("auth.v3.tenant_access_token.internal.post")
        .body_json(&serde_json::json!({
            "app_id": app_id,
            "app_secret": app_secret
        }))?
        .send()
        .await?;

    if response.status == 200 {
        let json = response.json_value()?;
        if let Some(token) = json.get("tenant_access_token") {
            println!("Tenant Access Token: {}", token);
            if let Some(expire) = json.get("expire") {
                println!("Expires in: {} seconds", expire);
            }
        } else {
            println!("Failed to get token: {:?}", json);
        }
    } else {
        println!("Request failed with status: {}", response.status);
    }

    Ok(())
}
