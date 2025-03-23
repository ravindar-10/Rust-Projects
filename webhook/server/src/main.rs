use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let webhook_url = "http://127.0.0.1:9001/webhook";
    loop {
        let payload = json!({
            "event":"test_event",
            "data":{
                "message":"Hello from the server",
                "timestamp":chrono::Utc::now().to_rfc3339()
            }
        });
        let response = client.post(webhook_url).json(&payload).send().await?;
        if response.status().is_success() {
            println!("Webhook call successful: {:?}", response.text().await?);
        } else {
            println!(
                "Failed to call webhook. Status: {},Error: {:?}",
                response.status(),
                response.text().await?
            );
        }
    }
    sleep(Duration::from_secs(10)).await;
}
