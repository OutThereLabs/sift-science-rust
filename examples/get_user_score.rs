//! Sift get user score example
//!
//! In order to run the example call:
//!
//! ```sh
//! export USER_ID=billy_jones_301
//! export API_KEY=YOUR_API_KEY
//!
//! cargo run --example get_user_score --features=reqwest
//! ```
//!
use sift_science::{score::ScoreOptions, AbuseType, Client};
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let user_id = env::var("USER_ID").expect("must specify USER_ID env var");
    let http_client = reqwest::Client::default();
    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");

    // Instantiate sift client
    let sift = Client::new(api_key, http_client);

    // Retrieve a score for a given user
    let response = sift
        .get_user_score(
            user_id,
            ScoreOptions {
                abuse_types: Some(vec![AbuseType::PaymentAbuse]),
                ..Default::default()
            },
        )
        .await;

    info!(?response, "Got sift score response");

    Ok(())
}
