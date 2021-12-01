//! A two-factor authentication solution built into Sift.
//!
//! Verification enables the triggering of one-time passwords for risky login attempts based on our
//! ATO risk score. Configuration includes integrating with our Verification API as well as an
//! in-console setup. The in-console setup involves creating TFA email templates and generating
//! CNAMEs to update your DNS records so we can send emails on your behalf.
//!
//! This service is designed for sending emails to the already-confirmed emails of account owners.
//! It is not designed as a tool to confirm whether an email address is real or is owned by a user.
//! Therefore, it works best when you use email addresses that you have already validated are real
//! and owned by the user account the 2FA code is being sent for.

use crate::{
    common::{deserialize_ms, deserialize_opt_ms, serialize_ms, serialize_opt_ms},
    events::{App, Browser, VerificationReason, VerificationType, VerifiedEvent},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use std::time::{Duration, SystemTime};

/// Verification request data
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SendRequest {
    /// User ID of user being verified.
    ///
    /// e.g. johndoe123
    #[serde(rename = "$user_id")]
    pub user_id: String,

    /// The phone / email to send the OTP to.
    #[serde(rename = "$send_to")]
    pub send_to: String,

    /// The channel used for verification
    #[serde(rename = "$verification_type")]
    pub verification_type: VerificationType,

    /// The ID of the entity impacted by the event being verified.
    #[serde(rename = "$verified_entity_id")]
    pub verified_entity_id: Option<String>,

    /// Name of the brand of product or service the user interacts with.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country of the content of the site.
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// The event being verified
    #[serde(rename = "$event")]
    pub event: SendRequestEvent,
}

/// Resend verification request data
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ResendRequest {
    /// User ID of user being verified.
    ///
    /// e.g. johndoe123
    #[serde(rename = "$user_id")]
    pub user_id: String,

    /// This will be the event type that triggered the verification
    #[serde(rename = "$verified_event")]
    pub verified_event: Option<VerifiedEvent>,

    /// The ID of the entity impacted by the event being verified.
    #[serde(rename = "$verified_entity_id")]
    pub verified_entity_id: Option<String>,
}

/// The event being verified by the request
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SendRequestEvent {
    /// The session being verified.
    ///
    /// See [verification] in the Sift Events API documentation.
    ///
    /// [verification]: https://sift.com/developers/docs/curl/events-api/reserved-events/update-password
    #[serde(rename = "$session_id")]
    pub session_id: String,

    /// The type of the reserved event being verified.
    #[serde(rename = "$verified_event")]
    pub verified_event: VerifiedEvent,

    /// The user's IP address
    #[serde(rename = "$ip")]
    pub ip: Option<String>,

    /// The trigger for the verification.
    ///
    /// See [verification] in the Sift Events API documentation.
    ///
    /// [verification]: https://sift.com/developers/docs/curl/events-api/reserved-events/update-password
    #[serde(rename = "$reason")]
    pub reason: Option<VerificationReason>,

    /// The user agent of the browser that is verifying.
    ///
    /// Use this field if the client is a browser. Note: cannot be used in conjunction with $app.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, OS, and device that is verifying.
    ///
    /// Use this field if the client is an app. Note: cannot be used in conjunction with $browser.
    #[serde(rename = "$app")]
    pub app: Option<App>,
}

/// Send verification response
#[derive(Debug, Serialize, Deserialize)]
pub struct SendResponse {
    /// The success or error code (see [relevant error codes]).
    ///
    /// [relevant error codes]: https://sift.com/developers/docs/curl/events-api/error-codes
    pub status: i32,

    /// Human readable description of the error.
    pub error_message: String,

    /// The time the OTP was sent.
    #[serde(
        deserialize_with = "deserialize_opt_ms",
        serialize_with = "serialize_opt_ms"
    )]
    pub sent_at: Option<SystemTime>,

    /// Name of the brand of product or service the user interacts with.
    pub brand_name: Option<String>,

    /// Country of the content of the site.
    pub site_country: Option<String>,

    /// The content language of the OTP
    pub content_language: Option<String>,

    /// The notification segment id
    pub segment_id: Option<String>,

    /// The notification segment name
    pub segment_name: Option<String>,
}

/// Options that may be passed when checking a verification
#[derive(Debug, Default)]
pub struct CheckOptions {
    /// This will be the event type that triggered the verification.
    pub verified_event: Option<VerifiedEvent>,

    /// The ID of the entity impacted by the event being verified.
    pub verified_entity_id: Option<String>,

    /// Overrides the timeout for this call.
    pub timeout: Option<Duration>,

    /// Overrides the version of the Events API to call.
    pub version: Option<ApiVersion>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CheckRequest {
    /// User ID of user being verified.
    ///
    /// e.g. johndoe123.
    #[serde(rename = "$user_id")]
    pub(crate) user_id: String,

    /// The code the user sent to the customer for validation.
    #[serde(rename = "$code")]
    pub(crate) code: u32,

    /// This will be the event type that triggered the verification.
    #[serde(rename = "$verified_event")]
    pub(crate) verified_event: Option<VerifiedEvent>,

    /// The ID of the entity impacted by the event being verified.
    #[serde(rename = "$verified_entity_id")]
    pub(crate) verified_entity_id: Option<String>,
}

/// Check verification response
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckResponse {
    /// The success or error code (see [relevant error codes]).
    ///
    /// [relevant error codes]: https://sift.com/developers/docs/curl/events-api/error-codes
    pub status: i32,

    /// Human readable description of the error.
    pub error_message: String,

    /// The time the OTP verified.
    #[serde(deserialize_with = "deserialize_ms", serialize_with = "serialize_ms")]
    pub checked_at: SystemTime,
}

/// Verification API version
#[derive(Copy, Clone, Debug)]
pub enum ApiVersion {
    /// Version 1
    V1,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiVersion::V1 => write!(f, "v1"),
        }
    }
}
