//! Sift label user example
//!
//! Labels API is no longer recommended for new customers. Decisions are now the recommended
//! integration, they enable you to send more granular and powerful feedback to our machine
//! learning system.
//!
//! In order to run the example call:
//!
//! ```sh
//! export USER_ID=billy_jones_301
//! export API_KEY=YOUR_API_KEY
//!
//! cargo run --example label --features=reqwest
//! ```
//!
use sift_science::{
    labels::{LabelOptions, LabelProperties},
    score::ScoreOptions,
    AbuseType, Client,
};
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let user_id = env::var("USER_ID").expect("must specify USER_ID env var");
    let http_client = reqwest::Client::default();
    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");

    // Instantiate sift client
    let sift = Client::new(api_key, http_client);

    // Label a user
    sift.label(
        user_id.clone(),
        LabelProperties {
            is_fraud: true,
            abuse_type: AbuseType::PaymentAbuse,
            description: None,
            source: Some("manual review".into()),
            analyst: None,
        },
        LabelOptions::default(),
    )
    .await?;

    // Retrieve score to check labels
    let response = sift
        .get_user_score(
            user_id,
            ScoreOptions {
                abuse_types: Some(vec![AbuseType::PaymentAbuse]),
                ..Default::default()
            },
        )
        .await?;

    info!(?response.latest_labels, "Got sift score response with labels");

    Ok(())
}
