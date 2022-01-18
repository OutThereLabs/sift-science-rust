//! Use the Score API to get risk scores associated with users of your application.
//!
//! The Sift APIs give you multiple ways to get a risk score for any of your users. A risk score is
//! a measure of how likely a user is to commit abuse using your service. Scores are computed in
//! real time from the data sent via the Events API and the Sift Javascript snippet.
//!
//! Sift calculates risk scores for six types of abuse:
//!
//! * **Payment Abuse** - A user fraudulently using a credit card or other payment method.
//! * **Content Abuse** - A user creating content (e.g. spammy/scammy posts) or sending messages on
//!   your site in an abusive manner.
//! * **Promotion Abuse** - A user gaming the terms of a promotion in an excessive or abusive way.
//! * **Account Abuse** - A user using your service in a manner you deem abusive (e.g. fake accounts).
//! * **Account Takeover** - the risk that an account is accessed through stolen credentials
//! * **Legacy** - this is a legacy version of the Sift Score that scored users for custom fraud
//!   types, similar to the Account Abuse score.
//!
//! You can get one or more of these abuse scores at a time. We will calculate scores for any of
//! the abuse detection products you've enabled. Here is the pricing for our abuse prevention
//! products.
//!
//! There are four different ways to get scores from Sift:
//!
//! * [Get a score synchronously when you send an event]
//! * [Get a score and workflows status synchronously when you send an event]
//! * [Get the latest score without sending an event]
//! * [Get a score by forcing the re-scoring of a user]
//!
//! ## Getting risk scores synchronously when sending events
//!
//! Whenever you send an event, you can receive a Sift Score back synchronously in the response
//! back from the API. This is particularly useful for when you are sending an event that maps to a
//! key decision point for your business. Typically, customers find it most useful to get scores
//! back after creating an account, order, content, or adding a promotion, but you can get scores
//! back after passing any of the events we support.
//!
//! For events that you don't plan on using the score, it's best that you don't ask for the score
//! as it will add some latency to the request.
//!
//! [Get a score synchronously when you send an event]: https://sift.com/developers/docs/curl/score-api/synchronous-scores/overview
//! [Get a score and workflows status synchronously when you send an event]: https://sift.com/developers/docs/curl/workflows-api/overview
//! [Get the latest score without sending an event]: https://sift.com/developers/docs/curl/score-api/get-score/overview
//! [Get a score by forcing the re-scoring of a user]: https://sift.com/developers/docs/curl/score-api/rescore/overview

use crate::{
    common::{abuse_type_serialize, AbuseType},
    events::ApiVersion,
};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::time::Duration;

/// Optional parameters for the score api.
#[derive(Default, Debug, Clone)]
pub struct ScoreOptions {
    /// List of abuse types, specifying for which abuse types a score should be returned.
    ///
    /// By default, a score is returned for every abuse type to which you are subscribed.
    pub abuse_types: Option<Vec<AbuseType>>,

    /// Overrides the API key for this call.
    pub api_key: Option<String>,

    /// Overrides the timeout for this call.
    pub timeout: Option<Duration>,

    /// Overrides the version of the Events API to call.
    pub version: Option<ApiVersion>,

    /// Overrides the URI path prefix for this API call.
    pub path_prefix: Option<&'static str>,

    /// Overrides the URI path suffix for this API call.
    pub path_suffix: Option<&'static str>,
}

/// Query params for the score api.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub(crate) struct ScoreQueryParams {
    /// Sift API Key
    pub(crate) api_key: String,

    /// List of abuse types, specifying for which abuse types a score should be returned (if
    /// scoring was requested).
    ///
    /// By default, a score is returned for every abuse type to which you are subscribed.
    #[serde(serialize_with = "abuse_type_serialize")]
    pub(crate) abuse_types: Option<Vec<AbuseType>>,
}

impl From<ScoreOptions> for ScoreQueryParams {
    fn from(opts: ScoreOptions) -> Self {
        ScoreQueryParams {
            api_key: opts.api_key.unwrap_or_default(),
            abuse_types: opts.abuse_types,
        }
    }
}
