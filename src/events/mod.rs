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
use std::time::SystemTime;

mod complex_field_types;
mod reserved_events;
mod reserved_fields;

pub use complex_field_types::*;
pub use reserved_events::*;
pub use reserved_fields::*;

use crate::common::{abuse_type_serialize, deserialize_ms, serialize_ms, AbuseType};

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

/// The requested scoring information for the given user.
///
/// <https://sift.com/developers/docs/curl/score-api/get-score/overview>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreResponse {
    /// The success or error code.
    pub status: i32,

    /// Description of error if applicable.
    pub error_message: String,

    /// Contains the computed scores for all applicable abuse types.
    pub scores: Option<Scores>,

    /// The `id` for which the score was requested.
    pub entity_id: Option<String>,

    /// What type of entity is the score in reference to.
    ///
    /// This defaults to user.
    pub entity_type: Option<String>,

    /// Entries for all abuse types for which the given event has been labeled.
    ///
    /// NOTE: `latest_labels` is only intended for customers using the Labels API.
    ///
    /// The content of this struct is not subject to the abuse types specified in the request; we
    /// always include all labels that have been applied to the given entity.
    pub latest_labels: Option<LatestLabels>,

    /// All abuse types for which Decisions have been applied on the given entity.
    ///
    /// Note that the content of this map is not subject to the abuse types specified in the
    /// request; we always include all decisions that have been applied to the given entity.
    ///
    /// The map is keyed by abuse type, which could be one of: `payment_abuse`, `account_abuse`,
    /// `content_abuse`, `promotion_abuse`, `account_takeover`.
    pub latest_decisions: Option<serde_json::Value>,
}

/// Contains all computed scores for all applicable abuse types for a given user.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Scores {
    /// Score associated with the payment abuse type
    pub payment_abuse: Option<AbuseScore>,

    /// Score associated with the promotion abuse type
    pub promotion_abuse: Option<AbuseScore>,

    /// Score associated with the account abuse type
    pub account_abuse: Option<AbuseScore>,

    /// Score associated with the account takeover abuse type
    pub account_takeover: Option<AbuseScore>,

    /// Score associated with the content abuse type
    pub content_abuse: Option<AbuseScore>,
}

/// Computed score for an abuse type for a given user.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AbuseScore {
    /// Score for the user between 0.0 and 1.0. A score of 0.5 translates to a score a 50 in the
    /// console.
    pub score: f32,

    /// A list of the most significant reasons for the score and the values associated with the
    /// user. The included values will vary based on the user. Includes related users in the
    /// details object when applicable.
    pub reasons: Vec<AbuseScoreReason>,
}

/// A list of the most significant reasons for the score and the values associated with the user.
///
/// The included values will vary based on the user. Includes related users in the details object
/// when applicable.
#[derive(Debug, Serialize, Deserialize)]
pub struct AbuseScoreReason {
    /// Name of the risk signal.
    pub name: String,

    /// Value of the risk signal.
    pub value: String,

    /// Additional details. Provided only when relevant. E.g., may contain a details field which
    /// contains the IDs of related users.
    pub details: Option<serde_json::Value>,
}

/// Contains all computed labels for all applicable abuse types for a given entity.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestLabels {
    /// Label associated with the payment abuse type
    pub payment_abuse: Option<Label>,

    /// Label associated with the promotion abuse type
    pub promotion_abuse: Option<Label>,

    /// Label associated with the account abuse type
    pub account_abuse: Option<Label>,

    /// Label associated with the account takeover abuse type
    pub account_takeover: Option<Label>,

    /// Label associated with the content abuse type
    pub content_abuse: Option<Label>,
}

/// Entry for an abuse types for which a given event has been labeled.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    /// Indicates whether a user is engaging in behavior deemed harmful to your business.
    ///
    /// Set to true if the user is engaging in abusive activity. Set to false if the user is
    /// engaging in valid activity.
    is_bad: bool,

    /// The time the label was applied
    #[serde(serialize_with = "serialize_ms", deserialize_with = "deserialize_ms")]
    time: SystemTime,

    /// Freeform text description of the user and/or incident triggering the label.
    description: Option<String>,
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
