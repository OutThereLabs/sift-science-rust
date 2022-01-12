//! Receive notifications about events in Sift.
//!
//! When one of the events is triggered, Sift will send a JSON payload to the webhook's specified
//! URL. Webhooks can be used to update your own support tool, data warehouses, and more.

use crate::common::deserialize_ms;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::SystemTime;

/// Options when creating a new [Webhook].
///
/// See <https://sift.com/developers/docs/curl/webhooks-api/create> for examples.
#[derive(Debug, Serialize)]
pub struct WebhookRequest {
    /// The type of webhook payload
    pub payload_type: PayloadType,

    /// The webhook status.
    pub status: Status,

    /// The URL of the webhook endpoint.
    ///
    /// This must be HTTPS.
    pub url: String,

    /// The list of events to enable for this endpoint.
    ///
    /// These correspond to the Reserved Events in the [Events API].
    ///
    /// [Events API]: https://sift.com/developers/docs/curl/events-api/overview
    pub enabled_events: Vec<EnabledEvent>,

    /// A name you specify for this webhook.
    pub name: Option<String>,

    /// A description about what the webhook is used for.
    pub description: Option<String>,
}

/// Webhook data
///
/// See <https://sift.com/developers/docs/curl/webhooks-api/create> for examples.
#[derive(Debug, Deserialize, Serialize)]
pub struct Webhook {
    /// The id of the webhook.
    pub id: u64,

    /// The name of the webhook
    #[serde(default)]
    pub name: Option<String>,

    /// The description of the webhook
    #[serde(default)]
    pub description: Option<String>,

    /// The type of payload.
    pub payload_type: PayloadType,

    /// The webhook status.
    pub status: Status,

    /// The URL of the webhook endpoint.
    ///
    /// This must be HTTPS.
    pub url: String,

    /// The list of events to enable for this endpoint.
    ///
    /// These correspond to the Reserved Events in the [Events API].
    ///
    /// [Events API]: https://sift.com/developers/docs/curl/events-api/overview
    pub enabled_events: Vec<EnabledEvent>,

    /// The time at which the webhook was created
    #[serde(skip_serializing, deserialize_with = "deserialize_ms")]
    pub created: SystemTime,

    /// The time at which the webhook was updated
    #[serde(skip_serializing, deserialize_with = "deserialize_ms")]
    pub last_updated: SystemTime,
}

/// The type of webhook payload.
#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadType {
    /// This payload type provides an order data response.
    ///
    /// See the [order object](https://sift.com/developers/docs/curl/orders-api/order).
    #[serde(rename = "ORDER_V1_0")]
    OrderV10,
}

/// The webhook status.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    /// Indicates the webhook is in a draft state.
    ///
    /// No webhooks are sent.
    Draft,

    /// Indicates the webhook is active.
    ///
    /// The webhook is live.
    Active,
}

/// The event to be included as enabled for a webhook.
///
/// These correspond to the Reserved Events in the [Events API].
///
/// [Events API]: https://sift.com/developers/docs/curl/events-api/overview
#[derive(Debug, Serialize, Deserialize)]
pub enum EnabledEvent {
    /// Occurs whenever a [Event::CreateOrder] event is tracked.
    ///
    /// [Event::CreateOrder]: crate::events::Event::CreateOrder
    #[serde(rename = "$create_order")]
    CreateOrder,

    /// Occurs whenever a [Event::UpdateOrder] event is tracked.
    ///
    /// [Event::UpdateOrder]: crate::events::Event::UpdateOrder
    #[serde(rename = "$update_order")]
    UpdateOrder,

    /// Occurs whenever a [Event::OrderStatus] event is tracked.
    ///
    /// [Event::OrderStatus]: crate::events::Event::OrderStatus
    #[serde(rename = "$order_status")]
    OrderStatus,

    /// Occurs whenever a [Event::Transaction] event is tracked.
    ///
    /// [Event::Transaction]: crate::events::Event::Transaction
    #[serde(rename = "$transaction")]
    Transaction,

    /// Occurs whenever a [Event::Chargeback] event is tracked.
    ///
    /// [Event::Chargeback]: crate::events::Event::Chargeback
    #[serde(rename = "$chargeback")]
    Chargeback,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum WebhooksResponse {
    Error(Error),
    Webhooks { data: Vec<Webhook> },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum WebhookResponse {
    Error(Error),
    Webhook(Webhook),
}

/// Webhook API version
#[derive(Copy, Clone, Debug)]
pub enum ApiVersion {
    /// Version 3
    V3,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiVersion::V3 => write!(f, "v3"),
        }
    }
}
