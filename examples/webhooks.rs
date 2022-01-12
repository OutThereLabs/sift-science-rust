//! Sift webhook examples
//!
//! In order to run the example call:
//!
//! ```sh
//! export ACCOUNT_ID=87243905872349857240
//! export API_KEY=YOUR_API_KEY
//!
//! cargo run --example webhooks --features=reqwest
//! ```

use sift_science::{
    webhooks::{EnabledEvent, PayloadType, Status, WebhookRequest},
    Client,
};
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");
    let account_id = env::var("ACCOUNT_ID").expect("must specify ACCOUNT_ID env var");

    // Instantiate sift client
    let sift = Client::new(api_key, reqwest::Client::new()).with_account_id(account_id);

    // Create a new webhook
    let webhook = sift
        .create_webhook(WebhookRequest {
            payload_type: PayloadType::OrderV10,
            status: Status::Draft,
            url: "https://example.com/chargebacks".into(),
            enabled_events: vec![EnabledEvent::Chargeback],
            name: Some("Test Chargeback Webhook".into()),
            description: Some("My webhook to record chargeback events".into()),
        })
        .await?;

    info!(?webhook, "create success");

    // Remove the example webhook
    let res = sift.delete_webhook(webhook.id).await;

    info!(?res, "removed example webhook");

    Ok(())
}
