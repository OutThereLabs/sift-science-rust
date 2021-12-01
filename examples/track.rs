//! Sift event tracking example
//!
//! In order to run the example call:
//!
//! ```sh
//! export USER_ID=billy_jones_301
//! export SESSION_ID=gigtleqddo84l8cm15qe4il
//! export API_KEY=YOUR_API_KEY
//!
//! cargo run --example track --features=reqwest
//! ```
//!
use sift_science::{
    events::{CreateAccountProperties, Event, EventOptions},
    AbuseType, Client,
};
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let user_id = env::var("USER_ID").expect("must specify USER_ID env var");
    let session_id = env::var("SESSION_ID").ok();
    let http_client = reqwest::Client::default();
    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");

    // Instantiate sift client
    let sift = Client::new(api_key, http_client);

    // Track an event
    let response = sift
        .track(
            Event::CreateAccount {
                user_id,
                session_id,
                properties: Box::new(CreateAccountProperties {
                    user_email: Some("test@example.com".into()),
                    ..Default::default()
                }),
            },
            EventOptions {
                return_score: Some(true),
                abuse_types: Some(vec![AbuseType::AccountTakeover]),
                ..Default::default()
            },
        )
        .await;

    info!(?response, "Got sift event response");

    Ok(())
}
