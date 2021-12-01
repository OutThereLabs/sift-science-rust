use serde::{Deserialize, Serialize};

/// The status of the verification event.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerificationStatus {
    /// Your customer has attempted and passed the verification process.
    #[serde(rename = "$success")]
    Success,

    /// Your customer has attempted and failed the verification process.
    #[serde(rename = "$failure")]
    Failure,

    /// Verification has been sent to your customer but the customer has not attempted to perform
    /// the verification attempt.
    #[serde(rename = "$pending")]
    Pending,
}

/// The type of the reserved event being verified
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerifiedEvent {
    /// The `Event::AddItemToCart` event.
    #[serde(rename = "$add_item_to_cart")]
    AddItemToCart,

    /// The `Event::AddPromotion` event.
    #[serde(rename = "$add_promotion")]
    AddPromotion,

    /// The `Event::ContentStatus` event.
    #[serde(rename = "$content_status")]
    ContentStatus,

    /// The `Event::CreateAccount` event.
    #[serde(rename = "$create_account")]
    CreateAccount,

    /// The `Event::CreateContent` event.
    #[serde(rename = "$create_content")]
    CreateContent,

    /// The `Event::CreateOrder` event.
    #[serde(rename = "$create_order")]
    CreateOrder,

    /// The `Event::FlagContent` event.
    #[serde(rename = "$flag_content")]
    FlagContent,

    /// The `Event::Login` event.
    #[serde(rename = "$login")]
    Login,

    /// The `Event::OrderStatus` event.
    #[serde(rename = "$order_status")]
    OrderStatus,

    /// The `Event::RemoveItemFromCart` event.
    #[serde(rename = "$remove_item_from_cart")]
    RemoveItemFromCart,

    /// The `Event::Transaction` event.
    #[serde(rename = "$transaction")]
    Transaction,

    /// The `Event::UpdateAccount` event.
    #[serde(rename = "$update_account")]
    UpdateAccount,

    /// The `Event::UpdateContent` event.
    #[serde(rename = "$update_content")]
    UpdateContent,

    /// The `Event::UpdateOrder` event.
    #[serde(rename = "$update_order")]
    UpdateOrder,

    /// The `Event::UpdatePassword` event.
    #[serde(rename = "$update_password")]
    UpdatePassword,
}

/// The type of verification being performed.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerificationType {
    /// An SMS is sent to the user's phone containing a code, URL or other process to authenticate
    /// the user.
    #[serde(rename = "$sms")]
    Sms,

    /// A phone call is made to the user's phone containing a code or other process to authenticate
    /// the user.
    #[serde(rename = "$phone_call")]
    PhoneCall,

    /// An email is sent to the user's email address containing a code, URL or other process to
    /// authenticate the user.
    #[serde(rename = "$email")]
    Email,

    /// A passcode is generated for the user via an application.
    #[serde(rename = "$app_tfa")]
    AppTfa,

    /// A captcha is used to detect and stop possible automated or scripted activity.
    ///
    /// e.g. bots.
    #[serde(rename = "$captcha")]
    Captcha,

    /// A shared secret.
    ///
    /// e.g. former address, mother's maiden name, photo)
    #[serde(rename = "$shared_knowledge")]
    SharedKnowledge,

    /// A selfie processed via face recognition algorithms is used to authenticate the user.
    #[serde(rename = "$face")]
    Face,

    /// A fingerprint is used to authenticate the user.
    #[serde(rename = "$fingerprint")]
    Fingerprint,

    /// A notification is sent to a known device, and the user needs to approve it to authenticate.
    #[serde(rename = "$push")]
    Push,

    /// A hardware token (e.g., USB stick) is used to authenticate the user.
    #[serde(rename = "$security_key")]
    SecurityKey,
}

/// The trigger for the verification.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerificationReason {
    /// The user opted to require a verification with every login.
    #[serde(rename = "$user_setting")]
    UserSetting,

    /// A representative of the service provider (e.g., analyst, security engineer) forced a
    /// verification (e.g., upon noticing a suspicious behavior on the account).
    #[serde(rename = "$manual_review")]
    ManualReview,

    /// Input from Sift score, workflows or another system (in-house or third-party) triggered the
    /// verification.
    #[serde(rename = "$automated_rule")]
    AutomatedRule,
}

/// The current state of the chargeback.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChargebackState {
    /// Received
    #[serde(rename = "$received")]
    Received,

    /// Accepted
    #[serde(rename = "$accepted")]
    Accepted,

    /// Disputed
    #[serde(rename = "$disputed")]
    Disputed,

    /// Won
    #[serde(rename = "$won")]
    Won,

    /// Lost
    #[serde(rename = "$lost")]
    Lost,
}

/// The reason given for a chargeback.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChargebackReason {
    /// Fraud
    #[serde(rename = "$fraud")]
    Fraud,

    /// Duplicate
    #[serde(rename = "$duplicate")]
    Duplicate,

    /// Product not received
    #[serde(rename = "$product_not_received")]
    ProductNotReceived,

    /// Product unacceptable
    #[serde(rename = "$product_unacceptable")]
    ProductUnacceptable,

    /// Other
    #[serde(rename = "$other")]
    Other,
}

/// Captures the reason for the failure of a given login.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoginFailureReason {
    /// AccountUnknown username never existed on this site.
    #[serde(rename = "$account_unknown")]
    AccountUnknown,

    /// AccountSuspended username exists, but the account is locked or temporarily deactivated.
    #[serde(rename = "$account_suspended")]
    AccountSuspended,

    /// AccountDisabled username exists, account was closed or permanently deactivated.
    #[serde(rename = "$account_disabled")]
    AccountDisabled,

    /// WrongPassword username exists, but the password is incorrect for this user.
    #[serde(rename = "$wrong_password")]
    WrongPassword,
}

/// Supported social sign on types.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SocialSignOn {
    /// Facebook
    #[serde(rename = "$facebook")]
    FaceBook,

    /// Google
    #[serde(rename = "$google")]
    Google,

    /// LinkedIn
    #[serde(rename = "$linkedin")]
    LinkedIn,

    /// Twitter
    #[serde(rename = "$twitter")]
    Twitter,

    /// Yahoo
    #[serde(rename = "$yahoo")]
    Yahoo,

    /// Microsoft
    #[serde(rename = "$microsoft")]
    Microsoft,

    /// Amazon
    #[serde(rename = "$amazon")]
    Amazon,

    /// Apple
    #[serde(rename = "$apple")]
    Apple,

    /// Other
    #[serde(rename = "$other")]
    Other,
}

/// The type of account a given user has.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AccountType {
    /// Merchant
    Merchant,

    /// Shopper
    Shopper,

    /// Regular
    Regular,

    /// Premium
    Premium,
}

/// Represents the success or failure of a login attempt.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum LoginStatus {
    /// Login success
    #[serde(rename = "$success")]
    Success,

    /// Login failure
    #[serde(rename = "$failure")]
    Failure,
}

/// The reason the password was updated or an update was attempted.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UpdatePasswordReason {
    /// The user updates the password on their own while logged into the account. The update can be
    /// motivated by, e.g., desire to use a stronger password from a password manager or because
    /// the password expired after 90 days.
    #[serde(rename = "$user_update")]
    UserUpdate,

    /// The user forgot the password and initiates a self-service process to create a new password.
    /// The old password becomes invalid only once the process is complete.
    #[serde(rename = "$forgot_password")]
    ForgotPassword,

    /// The service provider reset the password following suspicious account behavior or a support
    /// ticket. The old password becomes invalid once the process is initiated
    #[serde(rename = "$forced_reset")]
    ForcedReset,
}

/// The status of the password update event.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UpdatePasswordStatus {
    /// New password was set. This is the only status needed for password updates from within the
    /// account (`reason` is `UpdatePasswordReason::UserUpdate`).
    #[serde(rename = "$success")]
    Success,

    /// User clicks an expired password link.
    #[serde(rename = "$failure")]
    Failure,

    /// Password change initiated, waiting for user to act.
    #[serde(rename = "$pending")]
    Pending,
}

/// Indicates the high-level state of the order.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OrderStatus {
    /// An approved order.
    #[serde(rename = "$approved")]
    Approved,

    /// A canceled order.
    #[serde(rename = "$canceled")]
    Canceled,

    /// An order that has been held for review.
    #[serde(rename = "$held")]
    Held,

    /// A fulfilled order.
    #[serde(rename = "$fulfilled")]
    Fulfilled,

    /// A returned order.
    #[serde(rename = "$returned")]
    Returned,
}

/// The reason for a cancellation.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OrderCancellationReason {
    /// Canceled for payment risk.
    #[serde(rename = "$payment_risk")]
    PaymentRisk,

    /// Canceled for abuse.
    #[serde(rename = "$abuse")]
    Abuse,

    /// Canceled for a policy reason.
    #[serde(rename = "$policy")]
    Policy,

    /// Canceled for another reason.
    #[serde(rename = "$other")]
    Other,
}

/// The source of a decision.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DecisionSource {
    /// Automated decision.
    #[serde(rename = "$automated")]
    Automated,

    /// Decision made after manual review.
    #[serde(rename = "$manual_review")]
    ManualReview,
}

/// The type of notification issued.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SecurityNotificationType {
    /// The notification was sent via email.
    #[serde(rename = "$email")]
    Email,

    /// The notification was sent via sms.
    #[serde(rename = "$sms")]
    Sms,

    /// The notification was sent as a push notification via your mobile app.
    #[serde(rename = "$push")]
    Push,
}

/// The type of transaction being recorded.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TransactionType {
    /// Authorization and capture of a payment performed together in one step.
    ///
    /// This is the most commonly used transaction type. This is the default `transaction_type` if
    /// the transaction type is not provided.
    #[serde(rename = "$sale")]
    Sale,

    /// Authorizing a payment by reserving the payment amount from the buyer's account.
    ///
    /// Money does not change hands until capture.
    #[serde(rename = "$authorize")]
    Authorize,

    /// Capturing a payment reserved in the authorization step.
    #[serde(rename = "$capture")]
    Capture,

    /// Cancelling a pending authorization or capture.
    #[serde(rename = "$void")]
    Void,

    /// Returning part or all of a captured payment to the buyer.
    #[serde(rename = "$refund")]
    Refund,

    /// Depositing money into an account.
    #[serde(rename = "$deposit")]
    Deposit,

    /// Withdrawing money from an account.
    #[serde(rename = "$withdrawal")]
    Withdrawal,

    /// Transferring money from one account to another.
    #[serde(rename = "$transfer")]
    Transfer,
}

/// Indicates the status of the transaction.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TransactionStatus {
    /// A successful transaction
    #[serde(rename = "$success")]
    Success,

    /// A failed transaction.
    #[serde(rename = "$failure")]
    Failure,

    /// A pending transaction.
    #[serde(rename = "$pending")]
    Pending,
}

///
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeclineCategory {
    /// Decliened for fraud.
    #[serde(rename = "$fraud")]
    Fraud,

    /// Decliened because lost or stolen card.
    #[serde(rename = "$lost_or_stolen")]
    LostOrStolen,

    /// Declined as risky.
    #[serde(rename = "$risky")]
    Risky,

    /// Bank declined.
    #[serde(rename = "$bank_decline")]
    BankDeclined,

    /// Declined as invalid.
    #[serde(rename = "$invalid")]
    Invalid,

    /// Card expired.
    #[serde(rename = "$expired")]
    Expired,

    /// Insufficient funds.
    #[serde(rename = "$insufficient_funds")]
    InsufficientFunds,

    /// Limit exceeded.
    #[serde(rename = "$limit_exceeded")]
    LimitExceeded,

    /// Additional validation required
    #[serde(rename = "$additional_verification_required")]
    AdditionalValidationRequired,

    /// Other decline category
    #[serde(rename = "$other")]
    Other,
}
