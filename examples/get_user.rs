//! Example: Get User Info
//!
//! This example demonstrates how to get user information using the SDK.
//!
//! ```bash
//! FEISHU_APP_ID=xxx FEISHU_APP_SECRET=xxx USER_ID=xxx cargo run --example get_user
//! ```

use feishu_sdk::Client;
use feishu_sdk::core::{Config, FEISHU_BASE_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("FEISHU_APP_ID").expect("FEISHU_APP_ID not set");
    let app_secret = std::env::var("FEISHU_APP_SECRET").expect("FEISHU_APP_SECRET not set");
    let user_id = std::env::var("USER_ID").unwrap_or_else(|_| "ou_xxx".to_string());

    let config = Config::builder(&app_id, &app_secret)
        .base_url(FEISHU_BASE_URL)
        .build();

    let client = Client::new(config)?;

    println!("Getting user info for: {}", user_id);

    let response = client
        .operation("contact.v3.user.get")
        .path_param("user_id", &user_id)
        .query_param("user_id_type", "open_id")
        .query_param("department_id_type", "open_department_id")
        .send()
        .await?;

    if response.status == 200 {
        let json = response.json_value()?;
        if let Some(user) = json.get("data").and_then(|d| d.get("user")) {
            println!("User Info:");
            println!("  Name: {:?}", user.get("name"));
            println!("  Open ID: {:?}", user.get("open_id"));
            println!("  Email: {:?}", user.get("email"));
            println!("  Mobile: {:?}", user.get("mobile"));
            println!("  Status: {:?}", user.get("status"));
        } else {
            println!("User not found or error: {:?}", json);
        }
    } else {
        let body = String::from_utf8_lossy(&response.body);
        println!(
            "Failed to get user. Status: {}, Body: {}",
            response.status, body
        );
    }

    Ok(())
}
