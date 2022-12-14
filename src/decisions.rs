//! Manage Sift decisions.
//!
//! ## Overview
//!
//! Decisions represent business actions taken on a user, order, content or session (eg "Block
//! Order", "Approve User", etc). You use Decisions to record of what has happened. Sift uses this
//! information to continuously improve the accuracy of your risk scores.
//!
//! When integrating with Sift, you need to create Decisions that represent the different business
//! actions your team takes. These will vary based on your business but some examples could
//! include: "Accept Order", "Approve Post", "Put User on Watchlist", "Block Order", "Ban User",
//! etc. Decisions are entirely customizable by you to meet the needs of your business. Decisions
//! are created and updated using the [Decisions page] of the Console.
//!
//! ## Using Decisions
//!
//! Decisions can be applied from within the Sift console, sent by your application to the Sift
//! API, or from a Sift Workflow. Whenever a Decision is applied, it should be accompanied by some
//! business action you are taking on your side. For example:
//!
//! * From the Sift console - When an analyst manually reviews a user and decides an order should
//!   be blocked, the analyst would click a Decision button in the console to cancel the order. Once
//!   it’s clicked, Sift sends a webhook to your system so that you can cancel the order within your
//!   system.
//! * From your application - When your application logic decides to block an order, you’d first
//!   block the order within your system and then send that Decision to the Sift API to record what
//!   took place.
//! * From a Workflow - When your Sift Workflow logic determines to block the creation of a post
//!   (eg Content Abuse Score > 95), Sift generates the Decision on that content, and sends a Webhook
//!   to your system so you can block the post within your system.
//!
//! [Decisions page]: https://sift.com/console/decisions

use crate::{
    common::{deserialize_ms, serialize_opt_ms},
    AbuseType, Error,
};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

/// A sift entity about which decisions can be made
#[derive(Debug)]
pub enum Entity {
    /// Decisions about a user.
    User {
        /// The id of the user
        user_id: String,
    },

    /// Decisions about an order.
    Order {
        /// The order's user id
        user_id: String,
        /// The id of the order
        order_id: String,
    },

    /// Decisions about a session.
    Session {
        /// The session's user id
        user_id: String,
        /// The id of the session
        session_id: String,
    },

    /// Decisions about content.
    Content {
        /// The content's user id
        user_id: String,
        /// The id of the content
        content_id: String,
    },
}

/// The types of entities about which decisions can be made.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    /// Decisions applied to users.
    User,

    /// Decisions applied to orders.
    Order,

    /// Decisions applied to sessions.
    Session,

    /// Decisions applied to content.
    Content,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entity::User { user_id } => f.write_fmt(format_args!("users/{}", user_id)),
            Entity::Order { user_id, order_id } => {
                f.write_fmt(format_args!("users/{}/orders/{}", user_id, order_id))
            }
            Entity::Session {
                user_id,
                session_id,
            } => f.write_fmt(format_args!("users/{}/sessions/{}", user_id, session_id)),
            Entity::Content {
                user_id,
                content_id,
            } => f.write_fmt(format_args!("users/{}/content/{}", user_id, content_id)),
        }
    }
}

/// Used to apply new decisions
#[derive(Debug, Serialize)]
pub struct DecisionRequest {
    /// The unique identifier of the decision to be applied to an entity.
    ///
    /// `decision_id` and `description` can be retrieved using the [GET decisions API].
    ///
    /// [GET decisions API]: https://sift.com/developers/docs/curl/decisions-api/apply-decisions/get-decisions
    pub decision_id: String,

    /// The source of this decision.
    pub source: Source,

    /// Analyst who applied the decision.
    ///
    /// Only required when source is set to [Source::ManualReview]. Does not need to be an email,
    /// can be any analyst identifier.
    pub analyst: Option<String>,

    /// The time the decision was applied.
    ///
    /// This is only necessary to send for historical backfill.
    #[serde(serialize_with = "serialize_opt_ms")]
    pub time: Option<SystemTime>,

    /// A description of the decision that will be applied.
    pub description: Option<String>,
}

/// The source of a sift [Decision].
#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    /// This decision was applied by an analyst during review of a user/order.
    ManualReview,

    /// This decision was applied to a user/order by an automated rules engine or internal system.
    ///
    /// There was no human analysis before this decision was made.
    AutomatedRule,

    /// This decision was applied to a user/order in response to a chargeback received.
    ///
    /// Source of chargeback should only be used for decisions your system automatically takes in
    /// response to a chargeback. Note: Whether or not you take automated action in response to
    /// Chargebacks, you should send Sift the [Chargeback] events.
    ///
    /// [Chargeback]: crate::events::Event::Chargeback
    Chargeback,
}

/// The Sift response to decisions
#[derive(Debug, Deserialize)]
pub struct Decision {
    /// The decision entity
    pub entity: EntityIdentifier,

    /// The created decision
    pub decision: DecisionIdentifier,

    /// The time the decision was applied.
    #[serde(deserialize_with = "deserialize_ms")]
    pub time: SystemTime,
}

/// An entity is identified by a type and an id
#[derive(Debug, Deserialize)]
pub struct EntityIdentifier {
    /// The type of entity on which the decision was taken.
    #[serde(rename = "type")]
    pub entity_type: EntityType,

    /// The unique identifier of the entity on which the decision was taken.
    pub id: String,
}

/// The status of a decision
#[derive(Debug, Deserialize)]
pub struct DecisionStatus {
    /// The latest decision
    pub decisions: Decisions,
}

/// The decisions for a given entity
#[derive(Debug, Deserialize)]
pub struct Decisions {
    /// Latest payment abuse decision
    pub payment_abuse: Option<LatestDecision>,

    /// Latest promo abuse decision
    pub promo_abuse: Option<LatestDecision>,

    /// Latest content abuse decision
    pub content_abuse: Option<LatestDecision>,

    /// Latest account abuse decision
    pub account_abuse: Option<LatestDecision>,

    /// Latest account takeover decision
    pub account_takeover: Option<LatestDecision>,

    /// Latest legacy decision
    pub legacy: Option<LatestDecision>,
}

/// The latest decision for an abuse type
#[derive(Debug, Deserialize)]
pub struct LatestDecision {
    /// Latest legacy decision
    pub decision: DecisionIdentifier,

    /// Webhook success status
    ///
    /// `true` if the webhook was successfully sent, `false` if the webhook failed to send, `None`
    /// if no webhook is configured.
    pub webhook_succeeded: Option<bool>,

    /// The time the decision was applied.
    #[serde(deserialize_with = "deserialize_ms")]
    pub time: SystemTime,
}

/// The latest decision reference
#[derive(Debug, Deserialize)]
pub struct DecisionIdentifier {
    /// The decision's id
    pub id: String,
}

/// A page of decisions
#[derive(Debug, Deserialize)]
pub struct DecisionPage {
    /// Decisions in this page
    #[serde(rename = "data")]
    pub decisions: Vec<DecisionData>,

    /// There are more pages of data
    pub has_more: bool,

    /// The response schema
    pub schema: String,

    /// The number of results
    pub total_results: u32,
}

/// The data for paginated decisions
#[derive(Debug, Deserialize)]
pub struct DecisionData {
    /// The id of the decision.
    ///
    /// This is auto generated when the decision is created based on the initial display name of
    /// the decision.
    pub id: String,

    /// Display name of the decision.
    pub name: Option<String>,

    /// A description of the decision.
    ///
    /// This field is intended as a way to describe the business action(s) associated with the
    /// Decision.
    pub description: Option<String>,

    /// The decision entity type
    pub entity_type: EntityType,

    /// The decision abuse type
    pub abuse_type: AbuseType,

    /// Roughly categorizes the type of business action that this decision represents.
    ///
    /// For example, if the decision was named "Cancel Order" and every time this decision was
    /// applied your application was configured to cancel the user’s order, this should be
    /// categorized as a BLOCK decision.
    pub category: String,

    /// URL configured as webhook for this decision.
    ///
    /// Only necessary if you are receiving Webhooks. When a decision with a webhook is applied via
    /// API, no webhook notification will be sent.
    #[serde(default)]
    pub webhook_url: Option<String>,

    /// The time the decision was created
    #[serde(deserialize_with = "deserialize_ms")]
    pub created_at: SystemTime,

    /// User who created the decision.
    #[serde(default)]
    pub created_by: Option<String>,

    /// The time at which the decision was last updated
    #[serde(deserialize_with = "deserialize_ms")]
    pub updated_at: SystemTime,

    /// User who last updated the decision.
    #[serde(default)]
    pub updated_by: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum DecisionResult<T> {
    Error(Error),
    Decision(T),
}

/// Decisions API version
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
