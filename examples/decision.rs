//! Sift decision examples
//!
//! In order to run the example call:
//!
//! ```sh
//! export ACCOUNT_ID=87243905872349857240
//! export API_KEY=YOUR_API_KEY
//! export ORDER_ID=order-1
//! export ORDER_USER_ID=order-user-1
//!
//! cargo run --example decision --features=reqwest
//! ```

use std::env;

use sift_science::{decisions::Entity, Client};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");
    let account_id = env::var("ACCOUNT_ID").expect("must specify ACCOUNT_ID env var");
    let order_id = env::var("ORDER_ID").expect("must specify ORDER_ID env var");
    let user_id = env::var("ORDER_USER_ID").expect("must specify ORDER_USER_ID env var");

    // Instantiate sift client
    let sift = Client::new(api_key, reqwest::Client::new()).with_account_id(account_id);

    // Get a decision status
    let status = sift
        .decision_status(Entity::Order { order_id, user_id })
        .await?;

    info!(?status, "response");

    Ok(())
}
