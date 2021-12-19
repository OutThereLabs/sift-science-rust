//! The Labels API is a way to tell Sift which transactions or events are fraudulent or legitimate.
//!
//! By telling us this information, Sift can identify abuse patterns unique to your business.
//! Labels are used by the platform to generate the risk scores you within your application to
//! automate your fraud fighting.
//!
//! Labels API is no longer recommended for new customers. Decisions are now the recommended
//! integration, they enable you to send more granular and powerful feedback to our machine
//! learning system. Learn more about Decisions.
//!
//! For customers already using Labels API, don't worry! It is still a supported integration
//! method. If you are interested in migrating to Decisions, please contact your account manager or
//! support@sift.com and we can help.

use crate::{
    common::{deserialize_ms, serialize_ms},
    events::{self, ApiVersion, Event, EventOptions},
    AbuseType,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::time::{Duration, SystemTime};

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

/// Details of the label
#[derive(Debug)]
pub struct LabelProperties {
    /// Indicates whether a user is engaging in behavior deemed harmful to your business.
    ///
    /// Set to true if the user is engaging in abusive activity. Set to false if the user is
    /// engaging in valid activity.
    pub is_fraud: bool,

    /// The type of abuse for which you want to send a label.
    ///
    /// It's important to send a label specific to the type of abuse the user is committing so that
    /// Sift can learn about specific patterns of behavior. You'll end up with more accurate
    /// results this way.
    pub abuse_type: AbuseType,

    /// Freeform text description of the user and/or incident triggering the label.
    ///
    /// Useful as annotation on why the label was added.
    pub description: Option<String>,

    /// Describes the original source of the label information.
    ///
    /// e.g. payment gateway, manual review, etc.
    pub source: Option<String>,

    /// Unique identifier (e.g. email address) of the analyst who applied the label.
    ///
    /// Useful for tracking purposes after the fact.
    pub analyst: Option<String>,

    /// Any extra non-reserved fields to be recorded with label.
    pub extra: Option<serde_json::Value>,
}

impl From<LabelProperties> for Event {
    fn from(props: LabelProperties) -> Self {
        let LabelProperties {
            is_fraud,
            abuse_type,
            description,
            source,
            analyst,
            extra,
        } = props;

        Event::Label {
            is_fraud,
            abuse_type,
            properties: events::LabelProperties {
                description,
                source,
                analyst,
                extra,
            },
        }
    }
}

/// Optional parameters for the label request
#[derive(Debug, Default)]
pub struct LabelOptions {
    /// Overrides the timeout for this call.
    pub timeout: Option<Duration>,

    /// Overrides the API key for this call.
    pub api_key: Option<String>,

    /// Overrides the version of the Events API to call.
    pub version: Option<ApiVersion>,
}

impl From<(LabelOptions, &str)> for EventOptions {
    fn from((opts, user_id): (LabelOptions, &str)) -> Self {
        let LabelOptions {
            api_key,
            timeout,
            version,
        } = opts;

        EventOptions {
            api_key,
            timeout,
            version,
            path: Some(format!("users/{}/labels", user_id).into()),
            ..Default::default()
        }
    }
}
