//! Use the Events API to record the core actions users take in your application.
//!
//! The more detail we capture about user behaviors, the better we can distinguish between
//! fraudulent and legitimate events.
//!
//! ```no_run
//! use sift_science::{
//!     events::{CreateAccountProperties, Event, EventOptions},
//!     AbuseType, Client,
//! };
//! use std::env;
//! use tracing::{info, Level};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     tracing_subscriber::fmt().with_max_level(Level::INFO).init();
//!
//!     let user_id = env::var("USER_ID").expect("must specify USER_ID env var");
//!     let session_id = env::var("SESSION_ID").ok();
//!     let http_client = reqwest::Client::default();
//!     let api_key = env::var("API_KEY").expect("must specify API_KEY env var");
//!
//!     // Instantiate sift client
//!     let sift = Client::new(api_key, http_client);
//!
//!     // Track an event
//!     let response = sift
//!         .track(
//!             Event::CreateAccount {
//!                 user_id,
//!                 session_id,
//!                 properties: Box::new(CreateAccountProperties {
//!                     user_email: Some("test@example.com".into()),
//!                     ..Default::default()
//!                 }),
//!             },
//!             EventOptions {
//!                 return_score: Some(true),
//!                 abuse_types: Some(vec![AbuseType::AccountTakeover]),
//!                 ..Default::default()
//!             },
//!         )
//!         .await;
//!
//!     info!(?response, "Got sift event response");
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use std::fmt;
use std::time::Duration;

mod complex_field_types;
mod reserved_events;
mod reserved_fields;

pub use complex_field_types::*;
pub use reserved_events::*;
pub use reserved_fields::*;

use crate::{
    common::{abuse_type_serialize, AbuseType},
    score::ScoreResponse,
};

/// Base unit for currencies.
///
/// 1 cent = 10,000 micros. $1.23 USD = 123 cents = 1,230,000 micros.
#[derive(Debug, Serialize, Deserialize)]
pub struct Micros(u64);

impl Micros {
    /// Create a new `Micros` instance from a value in a currency's base unit.
    ///
    /// E.g. USD base unit is cents:
    /// * 1 cent = 10,000 micros.
    /// * $1.23 USD = 123 cents = 1,230,000 micros.
    pub fn from_base_units(base_units: u64) -> Self {
        Micros(base_units * 10_000)
    }

    /// Create a new `Micros` instance from a converted currency's base unit value.
    ///
    /// E.g. USD base unit is cents:
    /// * 1 cent = 10,000 micros.
    /// * $1.23 USD = 123 cents = 1,230,000 micros.
    pub fn from_raw(raw: u64) -> Self {
        Micros(raw)
    }
}

/// Optional parameters for event requests.
#[derive(Debug, Default)]
pub struct EventOptions {
    /// If true, requests that the response include a score for this user, computed using the
    /// submitted event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/score-api/synchronous-scores>
    pub return_score: Option<bool>,

    /// List of abuse types, specifying for which abuse types a score should be returned (if
    /// scoring was requested).
    ///
    /// By default, a score is returned for every abuse type to which you are subscribed.
    pub abuse_types: Option<Vec<AbuseType>>,

    /// If true, requests that the response include any actions triggered as a result of the
    /// tracked event.
    pub return_action: Option<bool>,

    /// If true, requests that the response include the status of any workflow run as a result of
    /// the tracked event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/workflows-api/workflow-decisions>
    pub return_workflow_status: Option<bool>,

    /// Overrides the timeout for this call.
    pub timeout: Option<Duration>,

    /// Overrides the API key for this call.
    pub api_key: Option<String>,

    /// Overrides the version of the Events API to call.
    pub version: Option<ApiVersion>,

    /// Overrides the URI path for this API call.
    pub path: Option<Cow<'static, str>>,
}

/// Query params accepted by the events API.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub(crate) struct EventQueryParams {
    /// If true, requests that the response include a score for this user, computed using the
    /// submitted event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/score-api/synchronous-scores>
    pub(crate) return_score: Option<bool>,

    /// List of abuse types, specifying for which abuse types a score should be returned (if
    /// scoring was requested).
    ///
    /// By default, a score is returned for every abuse type to which you are subscribed.
    #[serde(serialize_with = "abuse_type_serialize")]
    pub(crate) abuse_types: Option<Vec<AbuseType>>,

    /// If true, requests that the response include any actions triggered as a result of the
    /// tracked event.
    pub(crate) return_action: Option<bool>,

    /// If true, requests that the response include the status of any workflow run as a result of
    /// the tracked event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/workflows-api/workflow-decisions>
    pub(crate) return_workflow_status: Option<bool>,
}

impl From<EventOptions> for EventQueryParams {
    fn from(options: EventOptions) -> Self {
        EventQueryParams {
            return_score: options.return_score,
            abuse_types: options.abuse_types,
            return_action: options.return_action,
            return_workflow_status: options.return_workflow_status,
        }
    }
}

/// Events API response.
///
/// <https://sift.com/developers/docs/curl/score-api/synchronous-scores/overview>
#[derive(Debug, Serialize, Deserialize)]
pub struct EventResponse {
    pub(crate) status: i32,
    pub(crate) error_message: String,
    pub(crate) score_response: Option<ScoreResponse>,
}

/// Events API version
#[derive(Copy, Clone, Debug)]
pub enum ApiVersion {
    /// Version 205
    V205,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiVersion::V205 => write!(f, "v205"),
        }
    }
}
