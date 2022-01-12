use crate::{
    common::{abuse_type_serialize, AbuseType},
    events::{self, Event, EventOptions, EventQueryParams, EventResponse},
    labels::{LabelOptions, LabelProperties},
    score::{ScoreOptions, ScoreQueryParams, ScoreResponse, Scores},
    verification::{
        self, CheckOptions, CheckRequest, CheckResponse, ResendRequest, SendRequest, SendResponse,
    },
    webhooks::{self, Webhook, WebhookRequest, WebhookResponse, WebhooksResponse},
    Error, Result,
};
use async_trait::async_trait;
#[cfg(any(feature = "awc", feature = "reqwest"))]
use futures::future::TryFutureExt;
use serde::Serialize;
use std::borrow::Cow;
use std::fmt;
use std::time::Duration;
use tracing::{debug, instrument, trace, warn};

const SIFT_ORIGIN: &str = "https://api.sift.com";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(2);

/// A client for the Sift Science API
///
/// This client allows access to all of Sifts APIs Each method corresponds to an endpoint defined
/// in their corresponding files.
///
/// Once you have a client set up, you can access the service's endpoints by calling the
/// appropriate method on [Client].
pub struct Client<T> {
    /// Sift api key
    pub api_key: String,

    /// Sift account id
    ///
    /// Your Sift account id can on the [Profile Tab] in the console.
    ///
    /// [Profile Tab]: https://sift.com/console/account/profile
    pub account_id: Option<String>,

    /// Http client
    pub http_client: T,

    /// Sift api origin
    pub origin: String,
}

impl<T: HttpClient> Client<T> {
    /// construct a new sift client client with a given api and HTTP client
    pub fn new(api_key: impl Into<String>, http_client: T) -> Self {
        Client {
            api_key: api_key.into(),
            account_id: None,
            http_client,
            origin: SIFT_ORIGIN.into(),
        }
    }

    /// Override the sift api origin.
    ///
    /// Useful for testing environments and debugging.
    pub fn with_origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = origin.into();
        self
    }

    /// Override the sift account id.
    pub fn with_account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }

    /// Sends an event to the Sift Science Events API.
    #[instrument(skip(self, event, options))]
    pub async fn track(&self, event: Event, options: EventOptions) -> Result<Option<Scores>> {
        let version = options.version.unwrap_or(events::ApiVersion::V205);
        let path = options.path.clone().unwrap_or(Cow::Borrowed("events"));
        let timeout = options.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let url = format!("{}/{}/{}", self.origin, version, path);
        let mut body = serde_json::json!(&event);
        body["$api_key"] = serde_json::json!(options.api_key.as_deref().unwrap_or(&self.api_key));
        trace!(?event, ?options, "preparing event");

        let query_params = EventQueryParams::from(options);
        debug!(
            ?url,
            query_params = ?serde_urlencoded::to_string(&query_params),
            body = ?serde_json::to_string(&body),
            "tracking event"
        );

        let sift_response = self
            .http_client
            .post(&url, Some(&query_params.into()), Some(&body), timeout, None)
            .await?;

        // if no response options set, there will be no body
        if sift_response.is_none() {
            return Ok(None);
        }
        let event_json = sift_response.unwrap();
        trace!(?event_json, "sift event API response");

        // Else there is a (nested) set of success or failure responses in the json body
        // ¯\_(ツ)_/¯
        match serde_json::from_value(event_json)? {
            EventResponse {
                score_response:
                    Some(ScoreResponse {
                        scores: Some(scores),
                        ..
                    }),
                ..
            } => Ok(Some(scores)),
            EventResponse {
                status,
                error_message,
                ..
            }
            | EventResponse {
                score_response:
                    Some(ScoreResponse {
                        status,
                        error_message,
                        ..
                    }),
                ..
            } if status != 0 => Err(Error::Request {
                status,
                error_message,
            }),
            _ => Ok(None),
        }
    }

    /// Fetches the latest score(s) computed for the specified user and abuse types.
    ///
    /// See <https://sift.com/developers/docs/curl/score-api/get-score/overview>
    #[instrument(skip(self, opts))]
    pub async fn get_user_score<U>(
        &self,
        user_id: U,
        mut opts: ScoreOptions,
    ) -> Result<ScoreResponse>
    where
        U: AsRef<str> + fmt::Debug,
    {
        let version = opts.version.unwrap_or(events::ApiVersion::V205);
        let path_prefix = opts.path_prefix.unwrap_or("users");
        let path_suffix = opts.path_prefix.unwrap_or("score");
        let timeout = opts.timeout.unwrap_or(DEFAULT_TIMEOUT);
        let user_id = urlencoding::encode(user_id.as_ref()).to_string();

        let url = format!(
            "{}/{}/{}/{}/{}",
            self.origin, version, path_prefix, user_id, path_suffix
        );
        opts.api_key.get_or_insert_with(|| self.api_key.clone());

        let query_params = ScoreQueryParams::from(opts);
        debug!(?url, query_params = ?serde_urlencoded::to_string(&query_params), "retrieving score");

        let score_json = self
            .http_client
            .get(&url, &query_params.into(), timeout, None)
            .await?;

        trace!(?score_json, "sift score API response");
        let score_response = serde_json::from_value(score_json)?;

        Ok(score_response)
    }

    /// Rescores the specified user for the specified abuse types and returns the resulting
    /// score(s).
    ///
    /// See <https://sift.com/developers/docs/curl/score-api/rescore>
    #[instrument(skip(self, opts))]
    pub async fn rescore_user<U>(&self, user_id: U, mut opts: ScoreOptions) -> Result<ScoreResponse>
    where
        U: AsRef<str> + fmt::Debug,
    {
        let version = opts.version.unwrap_or(events::ApiVersion::V205);
        let path_prefix = opts.path_prefix.unwrap_or("users");
        let path_suffix = opts.path_prefix.unwrap_or("score");
        let timeout = opts.timeout.unwrap_or(DEFAULT_TIMEOUT);
        let user_id = urlencoding::encode(user_id.as_ref()).to_string();

        let url = format!(
            "{}/{}/{}/{}/{}",
            self.origin, version, path_prefix, user_id, path_suffix
        );
        opts.api_key.get_or_insert_with(|| self.api_key.clone());

        let query_params = ScoreQueryParams::from(opts);
        debug!(?url, query_params = ?serde_urlencoded::to_string(&query_params), "rescoring");

        let score_json = self
            .http_client
            .post(&url, Some(&query_params.into()), None, timeout, None)
            .await?;

        trace!(?score_json, "sift score API response");

        match score_json {
            Some(score_json) => {
                let score_response = serde_json::from_value(score_json)?;

                Ok(score_response)
            }
            None => Err(Error::Server(
                "Expected a score, but received empty server response".into(),
            )),
        }
    }

    /// Labels a user.
    ///
    /// Labels API is no longer recommended for new customers. Decisions are now the recommended
    /// integration, they enable you to send more granular and powerful feedback to our machine
    /// learning system. Learn more about Decisions.
    ///
    /// See <https://sift.com/developers/docs/curl/labels-api/label-user>
    #[instrument(skip(self, properties, opts))]
    pub async fn label<U>(
        &self,
        user_id: U,
        properties: LabelProperties,
        opts: LabelOptions,
    ) -> Result<()>
    where
        U: AsRef<str> + fmt::Debug,
    {
        let formatted_id = urlencoding::encode(user_id.as_ref()).to_string();
        self.track(properties.into(), (opts, formatted_id.as_str()).into())
            .await?;

        Ok(())
    }

    /// Send a OTP to an end user.
    ///
    /// Sift **strongly** recommends using Verification with Workflows. However, you may want to use
    /// the `send_verification` method for testing purposes. `send_verification` initiates a user's
    /// 2FA flow: it triggers the generation of a OTP code that is stored by Sift and emails the
    /// code to the user. It will also produce a pending `Verification` event in the user's activity
    /// log.
    ///
    /// <https://sift.com/developers/docs/curl/verification-api/send>
    #[instrument(skip(self, req))]
    pub async fn send_verification(&self, req: SendRequest) -> Result<SendResponse> {
        let timeout = DEFAULT_TIMEOUT;
        let api_version = verification::ApiVersion::V1;
        let url = format!("{}/{}/verification/send", self.origin, api_version);
        let body = serde_json::json!(req);
        let auth = Some(self.api_key.as_str());

        debug!(?url, ?req, "sending verification");
        trace!(body = ?serde_json::to_string(&body), "verification data");

        let response_json = self
            .http_client
            .post(&url, None, Some(&body), timeout, auth)
            .await?;

        trace!(?response_json, "sift verification API response");

        match response_json {
            Some(response_json) => match serde_json::from_value(response_json)? {
                SendResponse {
                    status,
                    error_message,
                    ..
                } if status != 0 => {
                    warn!(status, ?error_message, "verification send error");
                    Err(Error::Request {
                        status,
                        error_message,
                    })
                }
                send_success => {
                    debug!(?send_success, "verification send success");
                    Ok(send_success)
                }
            },
            None => Err(Error::Server(
                "Expected a verification, but received empty server response".into(),
            )),
        }
    }

    /// Re-send a OTP to an end user.
    ///
    /// A user can ask for a new OTP (one-time password) if they haven't received the previous one,
    /// or in case the previous OTP expired. The /resend call generates a new OTP and sends it to
    /// the original recipient with the same settings (template, verified event info).
    ///
    /// <https://sift.com/developers/docs/curl/verification-api/resend>
    #[instrument(skip(self, req))]
    pub async fn resend_verification(&self, req: ResendRequest) -> Result<SendResponse> {
        let timeout = DEFAULT_TIMEOUT;
        let api_version = verification::ApiVersion::V1;
        let url = format!("{}/{}/verification/resend", self.origin, api_version);
        let body = serde_json::json!(req);
        let auth = Some(self.api_key.as_str());

        debug!(?url, ?req, "resending verification");
        trace!(body = ?serde_json::to_string(&body), "verification data");

        let response_json = self
            .http_client
            .post(&url, None, Some(&body), timeout, auth)
            .await?;

        trace!(?response_json, "sift verification API response");

        match response_json {
            Some(response_json) => match serde_json::from_value(response_json)? {
                SendResponse {
                    status,
                    error_message,
                    ..
                } if status != 0 => {
                    warn!(status, ?error_message, "verification resend error");
                    Err(Error::Request {
                        status,
                        error_message,
                    })
                }
                resend_success => {
                    debug!(?resend_success, "verification resend success");
                    Ok(resend_success)
                }
            },
            None => Err(Error::Server(
                "Expected a verification, but received empty server response".into(),
            )),
        }
    }

    /// Check a OTP provided by the end user.
    ///
    /// Sift checks the validity of the OTP, checks rate limits, and responds with a decision
    /// whether the user should be able to proceed or not.
    ///
    /// Use Sift's response to determine what action to take:
    ///
    /// * If the user was successfully verified, then let the user log in to the site.
    /// * If the user failed to verify (wrong code, too many attempts, etc.), then present an error
    ///   message to the user. The message should inform the user what to do next ("click resend
    ///   and try again" or "wait for minutes and try again")
    ///
    /// See <https://sift.com/developers/docs/curl/verification-api/check>
    #[instrument(skip(self, code, opts))]
    pub async fn check_verification<U>(
        &self,
        user_id: U,
        code: u32,
        opts: CheckOptions,
    ) -> Result<CheckResponse>
    where
        U: Into<String> + fmt::Debug,
    {
        let CheckOptions {
            verified_event,
            verified_entity_id,
            timeout,
            version,
        } = opts;

        let req = CheckRequest {
            user_id: user_id.into(),
            code,
            verified_event,
            verified_entity_id,
        };
        let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
        let api_version = version.unwrap_or(verification::ApiVersion::V1);
        let url = format!("{}/{}/verification/check", self.origin, api_version);
        let body = serde_json::json!(req);
        let auth = Some(self.api_key.as_str());

        debug!(?url, ?req, "checking verification");

        let response_json = self
            .http_client
            .post(&url, None, Some(&body), timeout, auth)
            .await?;

        trace!(?response_json, "sift verification API response");

        match response_json {
            Some(response_json) => match serde_json::from_value(response_json)? {
                CheckResponse {
                    status,
                    error_message,
                    ..
                } if status != 0 => {
                    warn!(status, ?error_message, "verification check error");
                    Err(Error::Request {
                        status,
                        error_message,
                    })
                }
                check_success => {
                    debug!(?check_success, "verification check success");
                    Ok(check_success)
                }
            },
            None => Err(Error::Server(
                "Expected a verification, but received empty server response".into(),
            )),
        }
    }

    /// Creates a new webhook with a specified URL.
    ///
    /// See <https://sift.com/developers/docs/curl/webhooks-api/create> for examples.
    ///
    /// # Errors
    ///
    /// This errors if an `account_id` is not set for this client.
    #[instrument(skip(self, req))]
    pub async fn create_webhook(&self, req: WebhookRequest) -> Result<Webhook> {
        let account_id = self
            .account_id
            .as_ref()
            .ok_or_else(|| Error::Server("account id not specified".into()))?;

        let timeout = DEFAULT_TIMEOUT;
        let api_version = webhooks::ApiVersion::V3;
        let url = format!(
            "{}/{}/accounts/{}/webhooks",
            self.origin, api_version, account_id
        );
        let body = serde_json::json!(req);
        let auth = Some(self.api_key.as_str());

        debug!(?url, ?req, "creating webhook");
        trace!(body = ?serde_json::to_string(&body), "webhook data");

        let response_json = self
            .http_client
            .post(&url, None, Some(&body), timeout, auth)
            .await?;

        trace!(?response_json, "sift webhook API response");

        match response_json {
            Some(response_json) => Ok(serde_json::from_value(response_json)?),
            None => Err(Error::Server(
                "Expected a webhook, but received empty server response".into(),
            )),
        }
    }

    /// Get all webhooks.
    ///
    /// See <https://sift.com/developers/docs/curl/webhooks-api/list> for examples.
    ///
    /// # Errors
    ///
    /// This errors if an `account_id` is not set for this client.
    #[instrument(skip(self))]
    pub async fn get_webhooks(&self) -> Result<Vec<Webhook>> {
        let account_id = self
            .account_id
            .as_ref()
            .ok_or_else(|| Error::Server("account id not specified".into()))?;

        let timeout = DEFAULT_TIMEOUT;
        let api_version = webhooks::ApiVersion::V3;
        let url = format!(
            "{}/{}/accounts/{}/webhooks",
            self.origin, api_version, account_id,
        );
        let auth = Some(self.api_key.as_str());

        debug!(?url, "Retrieving webhooks");

        let response_json = self
            .http_client
            .get(&url, &QueryParams::default(), timeout, auth)
            .await?;

        trace!(body = ?serde_json::to_string(&response_json), "sift webhook API response");

        match serde_json::from_value(response_json)? {
            WebhooksResponse::Webhooks { data } => Ok(data),
            WebhooksResponse::Error(err) => Err(err),
        }
    }

    /// Get a webhook by id.
    ///
    /// See <https://sift.com/developers/docs/curl/webhooks-api/retrieve> for examples.
    ///
    /// # Errors
    ///
    /// This errors if an `account_id` is not set for this client.
    #[instrument(skip(self, id))]
    pub async fn get_webhook(&self, id: u64) -> Result<Webhook> {
        let account_id = self
            .account_id
            .as_ref()
            .ok_or_else(|| Error::Server("account id not specified".into()))?;

        let timeout = DEFAULT_TIMEOUT;
        let api_version = webhooks::ApiVersion::V3;
        let url = format!(
            "{}/{}/accounts/{}/webhooks/{}",
            self.origin, api_version, account_id, id
        );
        let auth = Some(self.api_key.as_str());

        debug!(?url, "Retrieving webhook");

        let response_json = self
            .http_client
            .get(&url, &QueryParams::default(), timeout, auth)
            .await?;

        trace!(?response_json, "sift webhook API response");

        match serde_json::from_value(response_json)? {
            WebhookResponse::Webhook(webhook) => Ok(webhook),
            WebhookResponse::Error(err) => Err(err),
        }
    }

    /// Update a webhook.
    ///
    /// See <https://sift.com/developers/docs/curl/webhooks-api/update> for examples.
    ///
    /// # Errors
    ///
    /// This errors if an `account_id` is not set for this client.
    #[instrument(skip(self, webhook))]
    pub async fn update_webhook(&self, webhook: Webhook) -> Result<Webhook> {
        let account_id = self
            .account_id
            .as_ref()
            .ok_or_else(|| Error::Server("account id not specified".into()))?;

        let timeout = DEFAULT_TIMEOUT;
        let api_version = webhooks::ApiVersion::V3;
        let url = format!(
            "{}/{}/accounts/{}/webhooks/{}",
            self.origin, api_version, account_id, webhook.id,
        );
        let body = serde_json::json!(webhook);
        let auth = self.api_key.as_str();

        debug!(?url, "updating webhook");
        trace!(body = ?serde_json::to_string(&body), "webhook data");

        let response_json = self.http_client.put(&url, &body, timeout, auth).await?;

        trace!(?response_json, "sift webhook update response");

        match serde_json::from_value(response_json)? {
            WebhookResponse::Webhook(webhook) => Ok(webhook),
            WebhookResponse::Error(err) => Err(err),
        }
    }

    /// Delete a webhook.
    ///
    /// See <https://sift.com/developers/docs/curl/webhooks-api/delete> for examples.
    ///
    /// # Errors
    ///
    /// This errors if an `account_id` is not set for this client.
    #[instrument(skip(self))]
    pub async fn delete_webhook(&self, id: u64) -> Result<()> {
        let account_id = self
            .account_id
            .as_ref()
            .ok_or_else(|| Error::Server("account id not specified".into()))?;

        let timeout = DEFAULT_TIMEOUT;
        let api_version = webhooks::ApiVersion::V3;
        let url = format!(
            "{}/{}/accounts/{}/webhooks/{}",
            self.origin, api_version, account_id, id,
        );
        let auth = self.api_key.as_str();

        debug!(?url, "deleting webhook");

        self.http_client.delete(&url, timeout, auth).await
    }
}

impl<T: HttpClient + Default> Client<T> {
    /// construct a new client with a given api key and default HTTP client
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Client {
            api_key: api_key.into(),
            account_id: None,
            http_client: Default::default(),
            origin: SIFT_ORIGIN.into(),
        }
    }
}

impl<T> fmt::Debug for Client<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("api_key", &"****")
            .field("account_id", &self.account_id)
            .field("origin", &self.origin)
            .finish()
    }
}

/// Sift API query params
#[derive(Default, Debug, Serialize)]
pub struct QueryParams {
    /// Sift API Key
    api_key: Option<String>,

    /// If true, requests that the response include a score for this user, computed using the
    /// submitted event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/score-api/synchronous-scores>
    return_score: Option<bool>,

    /// List of abuse types, specifying for which abuse types a score should be returned (if
    /// scoring was requested).
    ///
    /// By default, a score is returned for every abuse type to which you are subscribed.
    #[serde(serialize_with = "abuse_type_serialize")]
    abuse_types: Option<Vec<AbuseType>>,

    /// If true, requests that the response include any actions triggered as a result of the
    /// tracked event.
    return_action: Option<bool>,

    /// If true, requests that the response include the status of any workflow run as a result of
    /// the tracked event.
    ///
    /// See <https://siftscience.com/developers/docs/ruby/workflows-api/workflow-decisions>
    return_workflow_status: Option<bool>,
}

impl From<EventQueryParams> for QueryParams {
    fn from(eqp: EventQueryParams) -> Self {
        let EventQueryParams {
            return_score,
            abuse_types,
            return_action,
            return_workflow_status,
        } = eqp;

        QueryParams {
            return_score,
            abuse_types,
            return_action,
            return_workflow_status,
            ..Default::default()
        }
    }
}

impl From<ScoreQueryParams> for QueryParams {
    fn from(sqp: ScoreQueryParams) -> Self {
        let ScoreQueryParams {
            api_key,
            abuse_types,
        } = sqp;

        QueryParams {
            api_key: Some(api_key),
            abuse_types,
            ..Default::default()
        }
    }
}

/// Http implementation to talk to the sift API
#[async_trait(?Send)]
pub trait HttpClient {
    /// Create a new GET request
    async fn get(
        &self,
        url: &str,
        query_params: &QueryParams,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<serde_json::Value>;

    /// Create a new POST request
    async fn post(
        &self,
        url: &str,
        query_params: Option<&QueryParams>,
        body: Option<&serde_json::Value>,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<Option<serde_json::Value>>;

    /// Create a new PUT request
    async fn put(
        &self,
        url: &str,
        body: &serde_json::Value,
        timeout: Duration,
        username: &str,
    ) -> Result<serde_json::Value>;

    /// Create a new DELETE request
    async fn delete(&self, url: &str, timeout: Duration, username: &str) -> Result<()>;
}

#[cfg(feature = "awc")]
#[async_trait(?Send)]
impl HttpClient for awc::Client {
    async fn get(
        &self,
        url: &str,
        query_params: &QueryParams,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut req = self
            .get(url)
            .header(
                awc::http::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .timeout(timeout)
            .query(&query_params)
            .map_err(|err| Error::Server(err.to_string()))?;

        if let Some(username) = username {
            req = req.basic_auth(username, None);
        }

        let mut res = req
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .await
    }

    async fn post(
        &self,
        url: &str,
        query_params: Option<&QueryParams>,
        body: Option<&serde_json::Value>,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<Option<serde_json::Value>> {
        let mut req = self
            .post(url)
            .header(
                awc::http::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .timeout(timeout);

        if let Some(username) = username {
            req = req.basic_auth(username, None);
        }

        if let Some(query_params) = query_params {
            req = req
                .query(&query_params)
                .map_err(|err| Error::Server(err.to_string()))?;
        }

        let mut res = if let Some(body) = body {
            req.send_json(&body)
                .map_err(|err| {
                    tracing::error!(?err, "request error");
                    Error::Server(err.to_string())
                })
                .await?
        } else {
            req.send()
                .map_err(|err| {
                    tracing::error!(?err, "request error");
                    Error::Server(err.to_string())
                })
                .await?
        };

        if res.status() == awc::http::StatusCode::NO_CONTENT {
            return Ok(None);
        } else if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .map_ok(Some)
            .await
    }

    async fn put(
        &self,
        url: &str,
        body: &serde_json::Value,
        timeout: Duration,
        username: &str,
    ) -> Result<serde_json::Value> {
        let mut res = self
            .put(url)
            .header(
                awc::http::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .basic_auth(username, None)
            .timeout(timeout)
            .send_json(&body)
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .await
    }

    async fn delete(&self, url: &str, timeout: Duration, username: &str) -> Result<()> {
        let mut res = self
            .delete(url)
            .header(
                awc::http::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .basic_auth(username, None)
            .timeout(timeout)
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        Ok(())
    }
}

/// Sift client using `awc` as http client
#[cfg(feature = "awc")]
pub type AwcClient = Client<awc::Client>;

#[cfg(feature = "reqwest")]
#[async_trait(?Send)]
impl HttpClient for reqwest::Client {
    async fn get(
        &self,
        url: &str,
        query_params: &QueryParams,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut req = self
            .get(url)
            .header(
                reqwest::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .query(query_params)
            .timeout(timeout);

        if let Some(username) = username {
            req = req.basic_auth::<_, String>(username, None);
        }

        let res = req
            .query(&query_params)
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .await
    }

    async fn post(
        &self,
        url: &str,
        query_params: Option<&QueryParams>,
        body: Option<&serde_json::Value>,
        timeout: Duration,
        username: Option<&str>,
    ) -> Result<Option<serde_json::Value>> {
        let mut req = self
            .post(url)
            .header(
                reqwest::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .timeout(timeout);

        if let Some(username) = username {
            req = req.basic_auth::<_, String>(username, None);
        }

        if let Some(query_params) = query_params {
            req = req.query(query_params);
        }

        if let Some(body) = body {
            req = req.json(&body);
        }

        let res = req
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        if res.status() == reqwest::StatusCode::NO_CONTENT {
            return Ok(None);
        } else if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .map_ok(Some)
            .await
    }

    async fn put(
        &self,
        url: &str,
        body: &serde_json::Value,
        timeout: Duration,
        username: &str,
    ) -> Result<serde_json::Value> {
        let res = self
            .put(url)
            .header(
                reqwest::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .basic_auth::<_, String>(username, None)
            .timeout(timeout)
            .json(&body)
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        res.json()
            .map_err(|err| Error::Server(err.to_string()))
            .await
    }

    async fn delete(&self, url: &str, timeout: Duration, username: &str) -> Result<()> {
        let res = self
            .delete(url)
            .header(
                reqwest::header::USER_AGENT,
                format!("sift-rust/{}", env!("CARGO_PKG_VERSION")),
            )
            .basic_auth::<_, String>(username, None)
            .timeout(timeout)
            .send()
            .map_err(|err| {
                tracing::error!(?err, "request error");
                Error::Server(err.to_string())
            })
            .await?;

        if !res.status().is_success() {
            let error: Error = res
                .json()
                .map_err(|err| Error::Server(err.to_string()))
                .await?;
            return Err(error);
        }

        Ok(())
    }
}

/// Sift client using `reqwest` as http client
#[cfg(feature = "reqwest")]
pub type ReqwestClient = Client<reqwest::Client>;
