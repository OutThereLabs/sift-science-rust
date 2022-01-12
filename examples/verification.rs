//! Sift verification example
//!
//! In order to run the example call:
//!
//! ```sh
//! export USER_ID=billy_jones_301
//! export SEND_TO=billy_jones_301@gmail.com
//! export SESSION_ID=gigtleqddo84l8cm15qe4il
//! export API_KEY=YOUR_API_KEY
//!
//! cargo run --example verification --features=reqwest
//! ```
use sift_science::{
    events::{
        Event, EventOptions, LoginProperties, LoginStatus, VerificationReason, VerificationType,
        VerifiedEvent,
    },
    verification::{CheckOptions, SendRequest, SendRequestEvent},
    Client,
};
use std::env;
use std::io;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let user_id = env::var("USER_ID").expect("must specify USER_ID env var");
    let session_id = env::var("SESSION_ID").expect("must specify SESSION_ID env var");
    let send_to = env::var("SEND_TO").expect("must specify SEND_TO env var");
    let http_client = reqwest::Client::default();
    let api_key = env::var("API_KEY").expect("must specify API_KEY env var");

    // Instantiate sift client
    let sift = Client::new(api_key, http_client);

    // Track an event to verify
    sift.track(
        Event::Login {
            user_id: user_id.clone(),
            session_id: Some(session_id.clone()),
            properties: LoginProperties {
                login_status: Some(LoginStatus::Success),
                user_email: Some(send_to.clone()),
                ..Default::default()
            },
        },
        EventOptions::default(),
    )
    .await?;

    // Initiate a verification
    let response = sift
        .send_verification(SendRequest {
            user_id: user_id.clone(),
            send_to,
            verification_type: VerificationType::Email,
            brand_name: None,
            site_country: None,
            event: SendRequestEvent {
                session_id: session_id.clone(),
                verified_event: VerifiedEvent::Login,
                verified_entity_id: Some(session_id.clone()),
                ip: None,
                reason: Some(VerificationReason::AutomatedRule),
                browser: None,
                app: None,
            },
        })
        .await;

    info!(?response, "Got sift verification send response");

    println!("What was the OTP code?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read input");
    let code = input.trim().parse().expect("input must be an integer");

    // Initiate a verification
    let response = sift
        .check_verification(
            user_id,
            code,
            CheckOptions {
                verified_event: Some(VerifiedEvent::Login),
                verified_entity_id: Some(session_id),
                ..Default::default()
            },
        )
        .await;

    info!(?response, "Got sift verification check response");

    Ok(())
}
