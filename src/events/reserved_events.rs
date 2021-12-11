use crate::events::{
    complex_field_types::{
        Address, App, Booking, Browser, Item, MerchantProfile, OrderedFrom, PaymentMethod,
        Promotion,
    },
    reserved_fields::*,
    AbuseType, Micros,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Core actions users take in your application.
///
/// The more detail we capture about user behaviors, the better we can distinguish between
/// fraudulent and legitimate events. We have two types of events:
///
/// * Reserved events are events are sent in a standard format, allowing us to do lots of advanced
///   analysis on the values sent. When possible, model the actions users take on your site or app
///   with reserved events.
/// * Custom events are events you create to capture actions unique to your application. If there
///   are key actions most of your users take that are not captured by our reserved events, send
///   these as custom event.
///
/// Each event has fields that provide details and accepts required, reserved, and custom fields.
///
/// <https://sift.com/developers/docs/curl/events-api/overview>
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum Event {
    /// Use `AddItemToCart` to record when a user adds an item to their shopping cart or list.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-item-to-cart>
    #[serde(rename = "$add_item_to_cart")]
    AddItemToCart {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        #[serde(rename = "$session_id")]
        session_id: Option<String>,

        /// Optional properties for the `AddItemToCart` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-item-to-cart>
        #[serde(flatten)]
        properties: AddItemToCartProperties,
    },

    /// Use `AddPromotion` to record when a user adds one or more promotions to their account.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-promotion>
    #[serde(rename = "$add_promotion")]
    AddPromotion {
        /// The user's account ID according to your systems.
        ///
        /// Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `AddPromotion` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-promotion>
        #[serde(flatten)]
        properties: AddPromotionProperties,
    },

    /// Use `Chargeback` to capture a chargeback reported on a transaction.
    ///
    /// This event can be called multiple times to record changes to the chargeback state.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/chargeback>
    #[serde(rename = "$chargeback")]
    Chargeback {
        /// The ID for the order that this chargeback is filed against.
        ///
        /// Note: Optional if the `transaction_id` is present. This field is not required if this
        /// chargeback was filed against a transaction with no `order_id`.
        #[serde(rename = "$order_id")]
        order_id: Option<String>,

        /// The ID for the transaction that this chargeback is filed against.
        ///
        /// Note: Optional if `order_id` is present.
        #[serde(rename = "$transaction_id")]
        transaction_id: Option<String>,

        /// Optional properties for the `Chargeback` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/chargeback>
        #[serde(flatten)]
        properties: ChargebackProperties,
    },

    /// Use `CreateAccount` to capture user details at account creation. To capture updates to an
    /// account after it is initially created, use `Event::UpdateAccount`.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-account>
    #[serde(rename = "$create_account")]
    CreateAccount {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        #[serde(rename = "$session_id")]
        session_id: Option<String>,

        /// Optional properties for the `CreateAccount` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-account>
        #[serde(flatten)]
        properties: Box<CreateAccountProperties>,
    },

    /// Use `CreateOrder` to record when a user submits an order for products or services they
    /// intend to purchase.
    ///
    /// This API event should contain the products/services ordered, the payment instrument(s), and
    /// user identification data.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-order>
    #[serde(rename = "$create_order")]
    CreateOrder {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `CreateOrder` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-order>
        #[serde(flatten)]
        properties: OrderProperties,
    },

    /// The Labels API is a way to tell Sift which transactions or events are fraudulent or
    /// legitimate.
    ///
    /// By telling us this information, Sift can identify abuse patterns unique to your business.
    /// Labels are used by the platform to generate the risk scores you within your application to
    /// automate your fraud fighting.
    ///
    /// Labels API is no longer recommended for new customers. Decisions are now the recommended
    /// integration, they enable you to send more granular and powerful feedback to our machine
    /// learning system. Learn more about Decisions.
    ///
    /// For customers already using Labels API, don't worry! It is still a supported integration
    /// method. If you are interested in migrating to Decisions, please contact your account
    /// manager or support@sift.com and we can help.
    ///
    /// <https://sift.com/developers/docs/curl/labels-api>
    #[serde(rename = "$label")]
    Label {
        /// Indicates whether a user is engaging in behavior deemed harmful to your business.
        ///
        /// Set to true if the user is engaging in abusive activity. Set to false if the user is
        /// engaging in valid activity.
        #[serde(rename = "$is_fraud")]
        is_fraud: bool,

        /// The type of abuse for which you want to send a label.
        ///
        /// It's important to send a label specific to the type of abuse the user is committing so that
        /// Sift can learn about specific patterns of behavior. You'll end up with more accurate
        /// results this way.
        #[serde(rename = "$abuse_type")]
        abuse_type: AbuseType,

        /// Optional properties for the `Label` event.
        ///
        /// <https://sift.com/developers/docs/curl/labels-api>
        #[serde(flatten)]
        properties: LabelProperties,
    },

    /// Use `Login` to record when a user attempts to log in.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/login>
    #[serde(rename = "$login")]
    Login {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        #[serde(rename = "$session_id")]
        session_id: Option<String>,

        /// Optional properties for the `Login` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/login>
        #[serde(flatten)]
        properties: LoginProperties,
    },

    /// Use `Logout` to record when a user logs out.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/logout>
    #[serde(rename = "$logout")]
    Logout {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `Logout` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/logout>
        #[serde(flatten)]
        properties: LogoutProperties,
    },

    /// Use `OrderStatus` to track the order processing workflow of a previously submitted order.
    ///
    /// For example, `OrderStatus` can be used to indicate that an order has been held for review,
    /// canceled due to suspected fraud, or fulfilled. This event can be called multiple times to
    /// record changes an order's status.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/order-status>
    #[serde(rename = "$order_status")]
    OrderStatus {
        /// The user's account ID according to your systems.
        ///
        /// Note that user IDs are case sensitive. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The ID for tracking this order in your system.
        #[serde(rename = "$order_id")]
        order_id: String,

        /// Indicates the high-level state of the order.
        #[serde(rename = "$order_status")]
        order_status: OrderStatus,

        /// Optional properties for the `OrderStatus` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/order-status>
        #[serde(flatten)]
        properties: OrderStatusProperties,
    },

    /// Use `RemoveItemFromCart` to record when a user removes an item from their shopping cart or
    /// list.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/remove-item-from-cart>
    #[serde(rename = "$remove_item_from_cart")]
    RemoveItemFromCart {
        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        ///
        /// Note: required if no User ID is provided.
        #[serde(rename = "$session_id")]
        session_id: Option<String>,

        /// The user's account ID according to your systems.
        ///
        /// Note that user IDs are case sensitive. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `RemoveItemFromCart` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/order-status>
        #[serde(flatten)]
        properties: RemoveItemFromCartProperties,
    },

    /// When you identify suspicious activity on a user account, you may want to notify the user of
    /// this activity.
    ///
    /// For example, a login may seem suspicious because the login attempt was made from a new
    /// device. You may choose to inform the user that this incident happened. Ideally, these
    /// notifications should contain a summary of the activity and also have a response mechanism
    /// where the user may confirm or deny if the suspicious activity was them. The
    /// `SecurityNotification` event is used to capture this lifecycle of issuing the notification
    /// and the user response.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/security-notification>
    #[serde(rename = "$security_notification")]
    SecurityNotification {
        /// The user's account ID according to your systems.
        ///
        /// Note that user IDs are case sensitive. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        #[serde(rename = "$session_id")]
        session_id: String,

        /// The status of the notification event: records the follow-up action taken by the
        /// notified user.
        #[serde(rename = "$notification_status")]
        notification_status: String,

        /// Optional properties for the `SecurityNotification` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/security-notification>
        #[serde(flatten)]
        properties: SecurityNotificationProperties,
    },

    /// Use `Transaction` to record attempts to exchange money, credit or other tokens of value.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/transaction>
    #[serde(rename = "$transaction")]
    Transaction {
        /// The user's account ID according to your systems.
        ///
        /// Note that user IDs are case sensitive. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Total transaction amount in micros in the base unit of the `currency_code`.
        #[serde(rename = "$amount")]
        amount: Micros,

        /// [ISO-4217] currency code for the amount.
        ///
        /// If your site uses alternative currencies, specify them here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: String,

        /// Optional properties for the `Transaction` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/transaction>
        #[serde(flatten)]
        properties: Box<TransactionProperties>,
    },

    /// Use `UpdateAccount` to record changes to the user's account information.
    /// For user accounts created prior to integrating with Sift, it's recommended that
    /// `CreateAccount` is called before `UpdateAccount` to enable Sift to track the account's age.
    /// Otherwise, call `UpdateAccount` and we'll infer that account was created before integration.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-account>
    #[serde(rename = "$update_account")]
    UpdateAccount {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `UpdateAccount` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-account>
        #[serde(flatten)]
        properties: Box<UpdateAccountProperties>,
    },

    /// Use `UpdateOrder` to record when a user updates an order for products or services they
    /// intend to purchase.
    ///
    /// This event contains the same fields as `CreateOrder`. The existing order will be completely
    /// replaced by the values sent in `UpdateOrder`. Be sure to specify all values for the order,
    /// not just those that changed. If no matching `OrderId` found, a new order will be created.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-order>
    #[serde(rename = "$update_order")]
    UpdateOrder {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// Optional properties for the `UpdateOrder` event.
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-order>
        #[serde(flatten)]
        properties: OrderProperties,
    },

    /// Use `UpdatePassword` to record all password changes, whether initiated by the user or the
    /// service.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-password>
    #[serde(rename = "$update_password")]
    UpdatePassword {
        /// The user's internal ID. Users without an assigned `user_id` will not show up in
        /// the console. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The reason the password was updated or an update was attempted. The process may trigger
        /// a verification (with `verified_event = update_password`).
        #[serde(rename = "$reason")]
        reason: UpdatePasswordReason,

        /// The status of the password update event.
        #[serde(rename = "$status")]
        status: UpdatePasswordStatus,

        /// Optional properties for the `UpdatePassword` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-password>
        #[serde(flatten)]
        properties: UpdatePasswordProperties,
    },

    /// When a user attempts a high-value activity (e.g., login, view or change account
    /// information) that you deem risky, you may decide to verify whether the user is who they say
    /// they are. This is typically done by asking the user to enter a one-time passcode that is
    /// sent to their email, phone, or app. Other supported methods are detailed below. Sift models
    /// this interaction with the `Events::Verification` event.
    ///
    /// <https://sift.com/developers/docs/curl/events-api/reserved-events/verification>
    #[serde(rename = "$verification")]
    Verification {
        /// The user's account ID according to your systems.
        ///
        /// Note that user IDs are case sensitive. Find valid `user_id` values [here].
        ///
        /// [here]: https://sift.com/developers/docs/curl/events-api/fields
        #[serde(rename = "$user_id")]
        user_id: String,

        /// The user's current session ID, used to tie a user's action before and after log in or
        /// account creation.
        #[serde(rename = "$session_id")]
        session_id: String,

        /// The status of the verification event.
        #[serde(rename = "$status")]
        status: VerificationStatus,

        /// Optional properties for the `Verification` event
        ///
        /// <https://sift.com/developers/docs/curl/events-api/reserved-events/verification>
        #[serde(flatten)]
        properties: VerificationProperties,
    },
}

/// Properties of the `AddItemToCart` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-item-to-cart>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddItemToCartProperties {
    /// The product item added to cart.
    ///
    /// The quantity is specified as a subfield.
    #[serde(rename = "$item")]
    pub item: Option<Item>,

    /// The user agent of the browser that is used to add the item to cart.
    ///
    /// Represented by the [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to add the item to cart.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from.
    ///
    /// Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with.
    ///
    /// Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `AddPromotion` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/add-promotion>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddPromotionProperties {
    /// Contains all promotions that have been newly applied to the referenced user.
    #[serde(rename = "$promotions")]
    pub promotions: Option<Vec<Promotion>>,

    /// The user agent of the browser that is used to add the promotion
    ///
    /// Represented by the [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to add the promotion.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from.
    ///
    /// Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with.
    ///
    /// Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `Chargeback` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/chargeback>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChargebackProperties {
    /// The user's account ID according to your systems.
    ///
    /// Recommended for better chargeback matching. Note that user IDs are case sensitive. Find
    /// valid `user_id` values [here](https://sift.com/developers/docs/curl/events-api/fields).
    #[serde(rename = "$user_id")]
    pub user_id: Option<String>,

    /// The current state of the chargeback.
    #[serde(rename = "$chargeback_state")]
    pub chargeback_state: Option<ChargebackState>,

    /// This field can be used to capture the reason given.
    #[serde(rename = "$chargeback_reason")]
    pub chargeback_reason: Option<ChargebackReason>,
}

/// Properties of the `CreateAccount` event
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-account>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateAccountProperties {
    /// Email of the user creating the account. Note: If the user's email is also their account
    /// ID in your system, set **both** the `user_id` and `user_email` fields to their email
    /// address.
    #[serde(rename = "$user_email")]
    pub user_email: Option<String>,

    /// The full name of the user.
    #[serde(rename = "$name")]
    pub name: Option<String>,

    /// The primary phone number of the user associated with this account. Provide the phone
    /// number as a string starting with the country code. Use [E.164] format or send in the
    /// standard national format of number's origin. For example: "+14155556041" or
    /// "1-415-555-6041" for a U.S. number. If you collect other phone numbers for the account,
    /// provide them as additional custom fields, e.g `work_phone`
    ///
    /// [E.164]: https://en.wikipedia.org/wiki/E.164
    #[serde(rename = "$phone")]
    pub phone: Option<String>,

    ///  The ID of the user that referred the current user to your business. This field is
    ///  required for detecting referral fraud. Note: User IDs are case sensitive. You may need
    ///  to normalize the capitalization of your user IDs. Follow our [guidelines] for
    ///  `user_id` values.
    ///
    ///  [guidelines]: https://sift.com/developers/docs/curl/events-api/fields
    #[serde(rename = "$referrer_user_id")]
    pub referrer_user_id: Option<String>,

    /// The payment method(s) associated with this account.
    #[serde(rename = "$payment_methods")]
    pub payment_methods: Option<Vec<PaymentMethod>>,

    /// The billing address associated with this user.
    #[serde(rename = "$billing_address")]
    pub billing_address: Option<Address>,

    /// The shipping address associated with this user.
    #[serde(rename = "$shipping_address")]
    pub shipping_address: Option<Address>,

    /// The list of promotions that apply to this account. You can add one or more promotions
    /// when creating or updating the account. It is particularly useful to add the promotion
    /// with this event if the account is receiving some referral incentive. You can also
    /// separately add promotions to the account via the `AddPromotion` event.
    #[serde(rename = "$promotions")]
    pub promotions: Option<Vec<Promotion>>,

    /// If the user logged in with a social identify provider, give the name here.
    #[serde(rename = "$social_sign_on_type")]
    pub social_sign_on_type: Option<SocialSignOn>,

    /// The user agent of the browser that is used to create the account. Represented by the
    /// [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to create the account. Represented
    /// by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Capture the type(s) of the account: "merchant" or "shopper", "regular" or "premium",
    /// etc. The array supports multiple types for a single account, e.g. ["merchant",
    /// "premium"].
    #[serde(rename = "$account_types")]
    pub account_types: Option<Vec<AccountType>>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `CreateOrder` and `UpdateOrder` events.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/create-order>
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-order>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderProperties {
    /// The user's current session ID, used to tie a user's action before and after login or
    /// account creation.
    ///
    /// Required if no `user_id` value is provided.
    #[serde(rename = "$session_id")]
    pub session_id: Option<String>,

    /// The ID for tracking this order in your system.
    #[serde(rename = "$order_id")]
    pub order_id: Option<String>,

    /// Email of the user creating this order.
    ///
    /// Note: If the user's email is also their account ID in your system, set both the `user_id`
    /// and `user_email` fields to their email address.
    #[serde(rename = "$user_email")]
    pub user_email: Option<String>,

    /// Total transaction amount in micros in the base unit of the $currency_code. 1 cent = 10,000
    /// micros. $1.23 USD = 123 cents = 1,230,000 micros. For currencies without cents of
    /// fractional denominations, like the Japanese Yen, use 1 JPY = 1000000 micros.
    #[serde(rename = "$amount")]
    pub amount: Option<Micros>,

    /// [ISO-4217] currency code for the amount.
    ///
    /// If your site uses alternative currencies, specify them here.
    ///
    /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
    #[serde(rename = "$currency_code")]
    pub currency_code: Option<String>,

    /// The billing address as entered by the user.
    #[serde(rename = "$billing_address")]
    pub billing_address: Option<Address>,

    /// The payment information associated with this order.
    ///
    /// Note: As opposed to `Transaction`, `CreateOrder` takes an array of `PaymentMethod` structs,
    /// so you can record orders that are paid for using multiple payments. See [PaymentMethod]
    /// under for more details.
    #[serde(rename = "$payment_methods")]
    pub payment_methods: Option<Vec<PaymentMethod>>,

    /// The shipping address as entered by the user.
    #[serde(rename = "$shipping_address")]
    pub shipping_address: Option<Address>,

    /// Whether the user requested priority/expedited shipping on their order.
    #[serde(rename = "$expedited_shipping")]
    pub expedited_shipping: Option<bool>,

    /// The list of items ordered.
    ///
    /// This may include physical products, gift cards, in-app purchases etc. Travel (Flights,
    /// Hotels, Rideshare, etc) and Event Ticketing customers should use `bookings` instead of
    /// `items`. `bookings` supports specialized fields for modeling specific to Travel, Ticketing,
    /// and other cases where users make bookings.
    ///
    /// Note: cannot be used in conjunction with $bookings.
    #[serde(rename = "$items")]
    pub items: Option<Vec<Item>>,

    /// The list of bookings made.
    ///
    /// This may include tickets and reservations like flights, hotels, rideshares etc.
    ///
    /// Note: cannot be used in conjunction with `items`.
    #[serde(rename = "$bookings")]
    pub bookings: Option<Vec<Booking>>,

    /// For marketplace businesses, this is the seller's user ID, typically a database primary key.
    ///
    /// Follow our [guidelines] for `user_id` values.
    ///
    /// [guidelines]: https://sift.com/developers/docs/curl/events-api/fields
    #[serde(rename = "$seller_user_id")]
    pub seller_user_id: Option<String>,

    /// The list of promotions that apply to this order.
    ///
    /// You can add one or more promotions when creating or updating an order. You can also
    /// separately add promotions to the account via the `AddPromotion` event.
    #[serde(rename = "$promotions")]
    pub promotions: Option<Vec<Promotion>>,

    /// Indicates the method of delivery to the user.
    #[serde(rename = "$shipping_method")]
    pub shipping_method: Option<ShippingMethod>,

    /// Shipping carrier for the shipment of the product.
    #[serde(rename = "$shipping_carrier")]
    pub shipping_carrier: Option<String>,

    /// Shipping tracking number(s) for the shipment of the product(s).
    #[serde(rename = "$shipping_tracking_numbers")]
    pub shipping_tracking_numbers: Option<Vec<String>>,

    /// The details about the specific physical location providing the good or service.
    ///
    /// This can also be used to capture pickup, delivery locations, etc.
    #[serde(rename = "$ordered_from")]
    pub ordered_from: Option<OrderedFrom>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,

    /// The details about the merchant or seller providing the goods or service.
    #[serde(rename = "$merchant_profile")]
    pub merchant_profile: Option<MerchantProfile>,
}

/// Optional properties of the `Label` event
///
/// <https://sift.com/developers/docs/curl/labels-api>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LabelProperties {
    /// Freeform text description of the user and/or incident triggering the label.
    ///
    /// Useful as annotation on why the label was added.
    #[serde(rename = "$description")]
    pub description: Option<String>,

    /// Describes the original source of the label information.
    ///
    /// e.g. payment gateway, manual review, etc.
    #[serde(rename = "$source")]
    pub source: Option<String>,

    /// Unique identifier (e.g. email address) of the analyst who applied the label.
    ///
    /// Useful for tracking purposes after the fact.
    #[serde(rename = "$analyst")]
    pub analyst: Option<String>,
}

/// Properties of the `Login` event
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/login>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginProperties {
    /// Use `login_status` to represent the success or failure of the login attempt.
    #[serde(rename = "$login_status")]
    pub login_status: Option<LoginStatus>,

    /// Email of the user logging in. Note: If the user's email is also their account ID
    /// Set both the `user_id` and `user_email` fields to their email address.
    #[serde(rename = "$user_email")]
    pub user_email: Option<String>,

    /// IP address of the user that is logging in.
    #[serde(rename = "$ip")]
    pub ip: Option<String>,

    /// The user agent of the browser that is used to create the account. Represented by the
    /// [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to create the account. Represented
    /// by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Capture the reason for the failure of the login.
    ///
    /// Allowed Values:
    /// `FailureReason::AccountUnknown` Username never existed on this site.
    /// `FailureReason::AccountSuspended` Username exists, but the account is locked or temporarily deactivated.
    /// `FailureReason::AccountDisabled` Username exists, account was closed or permanently deactivated.
    /// `FailureReason::WrongPassword` Username exists, but the password is incorrect for this user.
    pub failure_reason: Option<LoginFailureReason>,

    /// The username entered at the login prompt.
    pub username: Option<String>,

    /// If the user logged in with a social identify provider, give the name here.
    pub social_sign_on_type: Option<SocialSignOn>,

    /// Capture the type(s) of the account: "merchant" or "shopper", "regular" or "premium",
    /// etc. The array supports multiple types for a single account, e.g. ["merchant",
    /// "premium"].
    #[serde(rename = "$account_types")]
    pub account_types: Option<Vec<AccountType>>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `Logout` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/logout>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogoutProperties {
    /// The user agent of the browser that is used to logout.  Represented by the [Browser] object.
    /// Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to logout. Represented by the [App]
    /// struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `OrderStatus` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/order-status>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderStatusProperties {
    /// The reason for a cancellation.
    #[serde(rename = "$reason")]
    pub reason: Option<OrderCancellationReason>,

    /// The source of a decision.
    #[serde(rename = "$source")]
    pub source: Option<DecisionSource>,

    /// The analyst who made the decision, if manual.
    #[serde(rename = "$analyst")]
    pub analyst: Option<String>,

    /// An alternative to using `source` and `analyst`, this is the ID of the Sift Action webhook
    /// that triggered the status change.
    #[serde(rename = "$webhook_id")]
    pub webhook_id: Option<String>,

    /// Any additional information about this order status change.
    #[serde(rename = "$description")]
    pub description: Option<String>,

    /// The user agent of the browser that is used to add the item to cart.
    ///
    /// Represented by the [Browser] struct. Use this field if the client is a browser. Note: cannot
    /// be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to add the item to cart.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app. Note: cannot be
    /// used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `RemoveItemFromCart` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/remove-item-from-cart>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RemoveItemFromCartProperties {
    /// The product item removed from cart.
    #[serde(rename = "$item")]
    pub item: Option<Item>,

    /// The user agent of the browser that is used to remove the item from cart.
    ///
    /// Represented by the [Browser] struct. Use this field if the client is a browser. Note: cannot
    /// be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to remove the item from cart.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app. Note: cannot be
    /// used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `SecurityNotification` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/security-notification>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SecurityNotificationProperties {
    /// The type of notification issued.
    #[serde(rename = "$notification_type")]
    pub notification_type: Option<SecurityNotificationType>,

    /// The phone number or email address to which the notification was sent.
    ///
    /// Examples
    /// phone: 14155551212, 442072193000
    /// email: bob@example.com
    ///
    /// This value should be passed when the `notification_type` is set to `NotificationType::Sms`
    /// or `NotificationType::Email`.
    #[serde(rename = "$notified_value")]
    pub notified_value: Option<String>,

    /// The user agent of the browser.
    ///
    /// Represented by the [Browser] struct. Use this field if the client is a browser. Note: cannot
    /// be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app. Note: cannot be
    /// used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `Transaction` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/transaction>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionProperties {
    /// Email of the user creating this transaction.
    ///
    /// Note: If the user's email is also their account ID in your system, set both the `user_id`
    /// and `user_email` fields to their email address.
    #[serde(rename = "$user_email")]
    pub user_email: Option<String>,

    /// The type of transaction being recorded.
    #[serde(rename = "$transaction_type")]
    pub transaction_type: Option<TransactionType>,

    /// Indicates the status of the transaction.
    ///
    /// If the transaction was rejected by the payment gateway, set the value to `Failure`.
    #[serde(rename = "$transaction_status")]
    pub transaction_status: Option<TransactionStatus>,

    /// The ID for this order in your system.
    ///
    /// Used for cross referencing an order in your internal systems.
    #[serde(rename = "$order_id")]
    pub order_id: Option<String>,

    /// The ID for identifying this transaction.
    ///
    /// Important for tracking transactions, and linking different parts of the same transaction
    /// together, e.g., linking a refund to its original transaction.
    #[serde(rename = "$transaction_id")]
    pub transaction_id: Option<String>,

    /// The billing address as entered by the user.
    #[serde(rename = "$billing_address")]
    pub billing_address: Option<Address>,

    /// The payment information associated with this transaction.
    #[serde(rename = "$payment_method")]
    pub payment_method: Option<PaymentMethod>,

    /// The shipping address as entered by the user.
    #[serde(rename = "$shipping_address")]
    pub shipping_address: Option<Address>,

    /// The user's current session ID, used to tie a user's action before and after log in or
    /// account creation.
    #[serde(rename = "$session_id")]
    pub session_id: Option<String>,

    /// For marketplace businesses, this is the seller's user ID, typically a database primary key.
    ///
    /// Follow our [guidelines] for `user_id` values.
    ///
    /// [guidelines]: https://sift.com/developers/docs/curl/events-api/fields
    #[serde(rename = "$seller_user_id")]
    pub seller_user_id: Option<String>,

    /// For transfer transactions, the user ID of the user receiving the transfer.
    ///
    /// If `transfer_recipient_user_id` is specified, `transaction_type` must be set to
    /// `TransferType::Transfer`; otherwise, the system will give an error. Follow our [guidelines]
    /// for `user_id` values.
    ///
    /// [guidelines]: https://sift.com/developers/docs/curl/events-api/fields
    #[serde(rename = "$transfer_recipient_user_id")]
    pub transfer_recipient_user_id: Option<String>,

    /// Use `decline_category` to indicate the category of a transaction decline sent by the PSP.
    ///
    /// Please note: Only send this field when `transaction_status` is
    /// `TransactionStatus::Failure`. Sending for transactions with `TransactionStatus::Success`,
    /// `TransactionStatus::Pending` or `None` will result in error.
    ///
    /// This field trains the model on decline reasons across PSPs, helping Sift catch card testing
    /// and traditional payments fraud.
    #[serde(rename = "$decline_category")]
    pub decline_category: Option<DeclineCategory>,

    /// The details about the specific physical location providing the good or service.
    ///
    /// This can also be used to capture pickup, delivery locations, etc.
    #[serde(rename = "$ordered_from")]
    pub ordered_from: Option<OrderedFrom>,

    /// The user agent of the browser that is used to create the transaction.
    ///
    /// Represented by the [Browser] struct. Use this field if the client is a browser. Note: cannot
    /// be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to create the transaction.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app. Note: cannot be
    /// used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,

    /// Used to indicate the status of a 3DS request.
    #[serde(rename = "$status_3ds")]
    pub status_3ds: Option<Status3Ds>,

    /// Used to indicate the source of a 3DS request.
    #[serde(rename = "$triggered_3ds")]
    pub triggered_3ds: Option<Triggered3Ds>,

    /// Used to indicate if this is a recurring payment for the same amount to the same merchant
    /// (recurring payments are considered out of scope for SCA).
    #[serde(rename = "$merchant_initiated_transaction")]
    pub merchant_initiated_transaction: Option<bool>,

    /// The details about the merchant or seller providing the goods or service.
    #[serde(rename = "$merchant_initiated_transaction")]
    pub merchant_profile: Option<MerchantProfile>,

    /// The address to the specific physical location of the person sending a transaction.
    #[serde(rename = "$sent_address")]
    pub sent_address: Option<Address>,

    /// The address to the specific physical location of the person receiving a transaction.
    #[serde(rename = "$received_address")]
    pub received_address: Option<Address>,
}

/// Properties of the `UpdateAccount` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-account>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateAccountProperties {
    /// Track password changes via `Event::UpdatePassword`.
    ///
    /// If the user changed their password, set this field and mark as `true`. Additionally, Sift's
    /// recommended approach is to send the `Event::UpdatePassword` reserved event.
    #[serde(rename = "$changed_password")]
    pub changed_password: Option<bool>,

    /// Updated value of the user's email address.
    ///
    /// Note: If the user's email is also their account ID in your system, set both the `user_id`
    /// and `user_email` fields to their email address.
    #[serde(rename = "$user_email")]
    pub user_email: Option<String>,

    /// The full name of the user.
    #[serde(rename = "$name")]
    pub name: Option<String>,

    /// The primary phone number of the user associated with this account.
    ///
    /// Provide the phone number as a string starting with the country code. Use [E.164] format or
    /// send in the standard national format of number's origin. For example: "+14155556041" or
    /// "1-415-555-6041" for a U.S. number. If you collect other phone numbers for the account,
    /// provide them as additional custom fields, e.g `work_phone`
    ///
    /// [E.164]: https://en.wikipedia.org/wiki/E.164
    #[serde(rename = "$phone")]
    pub phone: Option<String>,

    ///  The ID of the user that referred the current user to your business.
    ///
    ///  This field is required for detecting referral fraud. Note: User IDs are case sensitive.
    ///  You may need to normalize the capitalization of your user IDs. Follow our [guidelines] for
    ///  `user_id` values.
    ///
    ///  [guidelines]: https://sift.com/developers/docs/curl/events-api/fields
    #[serde(rename = "$referrer_user_id")]
    pub referrer_user_id: Option<String>,

    /// The payment method(s) associated with this account.
    #[serde(rename = "$payment_methods")]
    pub payment_methods: Option<Vec<PaymentMethod>>,

    /// The billing address associated with this user.
    #[serde(rename = "$billing_address")]
    pub billing_address: Option<Address>,

    /// The shipping address associated with this user.
    #[serde(rename = "$shipping_address")]
    pub shipping_address: Option<Address>,

    /// If the user logged in with a social identify provider, give the name here.
    #[serde(rename = "$social_sign_on_type")]
    pub social_sign_on_type: Option<SocialSignOn>,

    /// The user agent of the browser that is used to create the account.
    ///
    /// Represented by the [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to create the account.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Capture the type(s) of the account: "merchant" or "shopper", "regular" or "premium",
    /// etc. The array supports multiple types for a single account, e.g. ["merchant",
    /// "premium"].
    #[serde(rename = "$account_types")]
    pub account_types: Option<Vec<AccountType>>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from.
    ///
    /// Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with.
    ///
    /// Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `UpdatePassword` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/update-password>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdatePasswordProperties {
    /// The user agent of the browser that is used to update the password.
    ///
    /// Represented by the [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to update the password.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}

/// Properties of the `Verification` event.
///
/// <https://sift.com/developers/docs/curl/events-api/reserved-events/verification>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VerificationProperties {
    /// The user agent of the browser that is verifying.
    ///
    /// Represented by the [Browser] object. Use this field if the client is a browser.
    ///
    /// Note: cannot be used in conjunction with `app`.
    #[serde(rename = "$browser")]
    pub browser: Option<Browser>,

    /// The details of the app, os, and device that is used to update the password.
    ///
    /// Represented by the [App] struct. Use this field if the client is an app.
    ///
    /// Note: cannot be used in conjunction with `browser`.
    #[serde(rename = "$app")]
    pub app: Option<App>,

    /// The type of the reserved event being verified.
    #[serde(rename = "$verified_event")]
    pub verified_event: Option<VerifiedEvent>,

    /// The ID of the entity impacted by the event being verified.
    ///
    /// Event to id mapping:
    ///
    /// * `login` -> Session ID
    /// * `create_order` -> Order ID
    /// * `create_content` -> Content ID
    /// * `create_account`, `update_account`, or `update_password` -> No ID needed
    #[serde(rename = "$verified_entity_id")]
    pub verified_entity_id: Option<String>,

    /// The type of verification being performed.
    #[serde(rename = "$verification_type")]
    pub verification_type: Option<VerificationType>,

    /// The phone number, email address or the question  used for verification.
    ///
    /// **Do NOT send the answer to the security question!**
    ///
    /// Examples:
    /// * Phone: 14155551212, 442072193000
    /// * Email: bob@example.com
    /// * Question: "what is your mother's maiden name?"
    ///
    /// This value should be passed when the `verification_type` is set to `sms`, `phone_call`,
    /// `email` or `shared_knowledge`.
    #[serde(rename = "$verified_value")]
    pub verified_value: Option<String>,

    /// The trigger for the verification
    #[serde(rename = "$reason")]
    pub reason: Option<VerificationReason>,

    /// Name of the brand of product or service being purchased.
    #[serde(rename = "$brand_name")]
    pub brand_name: Option<String>,

    /// Country the company is providing service from. Use [ISO-3166] country code.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$site_country")]
    pub site_country: Option<String>,

    /// Domain being interfaced with. Use [fully qualified domain name].
    ///
    /// [fully qualified domain name]: https://en.wikipedia.org/wiki/Fully_qualified_domain_name
    #[serde(rename = "$site_domain")]
    pub site_domain: Option<String>,
}
