use crate::common::{deserialize_opt_ms, serialize_opt_ms};
use crate::events::{Micros, PaymentMethodVerificationStatus, PaymentType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::time::SystemTime;

/// A physical address, such as a billing or shipping address.
///
/// The value must be a nested object with the appropriate address subfields. We extract many
/// geolocation features from these values. An address is represented as a nested JSON object.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Address {
    /// Provide the full name associated with the address here.
    ///
    /// Concatenate first name and last name together if you collect them separately in your
    /// system.
    #[serde(rename = "$name")]
    pub name: Option<String>,

    /// Address first line.
    ///
    /// e.g. "2100 Main Street".
    #[serde(rename = "$address_1")]
    pub address_1: Option<String>,

    /// Address second line.
    ///
    /// e.g. "Apt 3B".
    #[serde(rename = "$address_2")]
    pub address_2: Option<String>,

    /// The city or town name.
    #[serde(rename = "$city")]
    pub city: Option<String>,

    /// The region portion of the address.
    ///
    /// In the USA, this corresponds to the state.
    #[serde(rename = "$region")]
    pub region: Option<String>,

    /// The [ISO-3166] country code for the address.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$country")]
    pub country: Option<String>,

    /// The postal code associated with the address.
    ///
    /// e.g., "90210".
    ///
    /// Send +4 postal codes with a '-', e.g. "90210-3344".
    #[serde(rename = "$zipcode")]
    pub zipcode: Option<String>,

    /// The phone number associated with this address. Provide the phone number as a string
    /// starting with the country code. Use [E.164] format or send in the standard national format of
    /// number's origin. For example: "+14155556041" or "1-415-555-6041" for a U.S. number.
    ///
    /// [E.164]: https://en.wikipedia.org/wiki/E.164
    #[serde(rename = "$phone")]
    pub phone: Option<String>,

    /// Any extra non-reserved fields to be recorded with the address.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// The details of an application as well as the device and OS it's running on.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    /// The operating system on which application is running.
    ///
    ///e.g. iOS, Android
    #[serde(rename = "$os")]
    pub os: Option<String>,

    /// The operating system version on which application is running.
    ///
    /// e.g. 10.3.1, 7.1.1
    #[serde(rename = "$os_version")]
    pub os_version: Option<String>,

    /// The manufacturer of the device on which application is running.
    ///
    /// e.g. Samsung, Apple, LG
    #[serde(rename = "$device_manufacturer")]
    pub device_manufacturer: Option<String>,

    /// The model of the device on which application is running.
    ///
    /// e.g. SM-G920x, iPhone8,1
    #[serde(rename = "$device_model")]
    pub device_model: Option<String>,

    /// The unique ID of the device on which application is running.
    ///
    /// For iOS, send the IFV identifier. For Android, send the Android ID.
    #[serde(rename = "$device_unique_id")]
    pub device_unique_id: Option<String>,

    /// The name of your application.
    #[serde(rename = "$app_name")]
    pub app_name: Option<String>,

    /// The version of your application.
    ///
    /// Our accepted format is numbers separated by periods.
    #[serde(rename = "$app_version")]
    pub app_version: Option<String>,

    /// The language the application content is being delivered in.
    ///
    /// Use [ISO-3166] format for country codes. Examples: "en", "en-us, de", "fr-CH, fr;q=0.9,
    /// en;q=0.8, de;q=0.7, *;q=0.5", etc.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$client_language")]
    pub client_language: Option<String>,

    /// Any extra non-reserved fields to be recorded with the app.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// A specialized field, analogous to [`Item`], for travel and event ticketing use cases.
///
/// A `Booking` represents a reservation purchased by a user.
///
/// Should be used in a `CreateOrder` or `UpdateOrder` event instead of `items` - an event cannot
/// have both.
///
/// Bookings support two additional complex types, `segments` and `guests`. `segments` support more
/// detailed fields for each part of a booking, for example, legs of a flight. `guests` supports
/// detailed fields for each guest on the booking.
///
/// A single `CreateOrder` or `UpdateOrder` event can have multiple types of bookings, such as
/// sending both `Booking::Flight` and `Booking::Accommodation` bookings in one order.
///
/// Please not that `Booking` does not accept custom fields.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "$booking_type")]
pub enum Booking {
    /// For tickets to sporting events, concerts, etc.
    #[serde(rename = "$event_ticket")]
    EventTicket {
        /// A description of the event.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The start time of the event.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The finish time of the event.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per ticket.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of reservations of the given type purchased by the user.
        ///
        /// e.g. 2 for a purchase of two tickets to the same event.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests using the tickets.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// For event ticket bookings, this field represents the internal identifier associated
        /// with the event.
        #[serde(rename = "$event_id")]
        event_id: Option<String>,

        /// This field represents the id of the venue.
        #[serde(rename = "$venue_id")]
        venue_id: Option<String>,

        /// This field represents the name and address of the venue.
        #[serde(rename = "$location")]
        location: Option<Address>,

        /// This field captures the genre of ticket.
        #[serde(rename = "$category")]
        category: Option<String>,

        /// This field captures any descriptors of the event.
        ///
        /// For example, tags might be team names, region, etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the event.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For hotel reservations.
    #[serde(rename = "$accommodation")]
    Accomodation {
        /// A description of the reservation
        ///
        /// e.g., "Deluxe King, Hotel Alpha".
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The check-in time for a hotel reservation.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The check-out time for a hotel reservation.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per room.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of reservations.
        ///
        /// e.g., 2 for two hotel rooms of the same type.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the reservation.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// This field represents the type of room.
        ///
        /// e.g., "Double Queen Deluxe"
        #[serde(rename = "$room_type")]
        room_type: Option<String>,

        /// This field represents the id of the hotel.
        #[serde(rename = "$venue_id")]
        venue_id: Option<String>,

        /// This field represents the name and address of the venue.
        #[serde(rename = "$location")]
        location: Option<Address>,

        /// This field captures any descriptors of the booking.
        ///
        /// For example, tags might be "non-smoking", "wi-fi", etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the accomodatio.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For airline tickets.
    #[serde(rename = "$flight")]
    Flight {
        /// A description of the flight reservation.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The departure time for the first flight leg in the booking.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The arrival time for the last flight leg in the booking etc.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per flight ticket (including all legs of the flight).
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of tickets of the given type purchased by the user.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the flight.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// Use this field to send information about each leg of the flight (even if there's only
        /// one).
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// This field captures any descriptors of the booking.
        ///
        /// For example, tags might be "premium economy", "summer sale", etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the flight.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For bus, train or rail tickets.
    #[serde(rename = "$bus")]
    Bus {
        /// A description of the trip.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The departure time for a trip.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The arrival time of the trip.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per ticket.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of tickets of the given type purchased by the user.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the trip.
        ///
        /// The number of guests and the quantity of tickets do not need to match. Send as much
        /// information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// Use this field to send information about each ride of the trip (even if there's only
        /// one).
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// This field captures any descriptors of the trip.
        ///
        /// For example, tags might be "sleeper", "summer sale", etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the bus.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For booking rides in a ridesharing marketplace.
    #[serde(rename = "$rideshare")]
    Rideshare {
        /// A description of the ride.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The pickup time of the ride.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The estimated drop-off time of the ride.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per ride of the booking.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of reservations of the given type purchased by the user.
        ///
        /// This does not necessarily reflect the number of guests (eg, a single ride may be booked
        /// for two guests).
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the booking.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// Add more specific information about the ride (even if there is only one segment).
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// This field captures any descriptors of the ride.
        ///
        /// For example, tags might be "sale", "first ride", etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the ride share.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For a reservation of a car or other vehicle.
    #[serde(rename = "$vehicle")]
    Vehicle {
        /// A description of the reservation.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The pickup time for the reservation.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The drop-off time of the reservation.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per vehicle of the reservation.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of reservations of the given type purchased by the user.
        ///
        /// This does not reflect the number of guests (eg, a single vehicle may be booked for two
        /// guests), but would reflect the number of vehicles of the same type.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the reservation.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// Add more specific information about the reservation.
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// This field captures any descriptors of the reservation.
        ///
        /// For example, tags might be "sale", "first ride", etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the vehicle.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For a cruise ticket.
    #[serde(rename = "$cruise")]
    Cruise {
        /// A description of the trip.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The departure time of the cruise.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The arrival time of the cruise.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per ticket of the cruise.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of tickets of the given type purchased by the user.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the cruise.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// For travel bookings, use this field to send information about the travel segments.
        ///
        /// Eg. each item in this array would represent a flight segment, a rideshare ride etc
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// This field captures any descriptors of the reservation.
        ///
        /// For example, tags might be "last minute deal", "abcd cruise line", "contest winner"
        /// etc.
        #[serde(rename = "$tags")]
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the cruise.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },

    /// For any reservation use case not covered above.
    #[serde(rename = "$other")]
    Other {
        /// A description of the booking.
        #[serde(rename = "$title")]
        title: Option<String>,

        /// The start time of the reservation.
        #[serde(
            rename = "$start_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        start_time: Option<SystemTime>,

        /// The finish time of the reservation.
        #[serde(
            rename = "$end_time",
            deserialize_with = "deserialize_opt_ms",
            serialize_with = "serialize_opt_ms"
        )]
        end_time: Option<SystemTime>,

        /// The price per unit of the booking.
        #[serde(rename = "$price")]
        price: Option<Micros>,

        /// [ISO-4217] currency code for the amount.
        ///
        /// e.g., USD, CAD, HKD. If your site uses alternative currencies, like bitcoin or points
        /// systems, specify that here.
        ///
        /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
        #[serde(rename = "$currency_code")]
        currency_code: Option<String>,

        /// The count of reservations of the given type purchased by the user.
        #[serde(rename = "$quantity")]
        quantity: Option<String>,

        /// Details of the guests on the booking.
        ///
        /// Send as much information about each guest as you capture.
        #[serde(rename = "$guests")]
        guests: Option<Vec<Guest>>,

        /// For bookings with multiple segments, use this field to send information about the
        /// travel segments.
        ///
        /// e.g., each item in this array would represent a flight leg.
        #[serde(rename = "$segments")]
        segments: Option<Vec<Segment>>,

        /// For hotels or other accommodation bookings, this field represents the type of room.
        ///
        /// Eg. "Double Queen Deluxe"
        #[serde(rename = "$room_type")]
        room_type: Option<String>,

        /// For event ticket bookings, this field represents the internal identifier associated
        /// with the event.
        #[serde(rename = "$event_id")]
        event_id: Option<String>,

        /// For event ticket bookings, this field represents the name of the venue.
        #[serde(rename = "$venue_id")]
        venue_id: Option<String>,

        /// For event ticket and accommodation bookings, this field represents the address of venue
        /// or hotel respectively.
        #[serde(rename = "$location")]
        location: Option<Address>,

        /// This field can be used to send the category of booking.
        ///
        /// For event tickets, this field captures the genre of a ticket.
        #[serde(rename = "$category")]
        category: Option<String>,

        /// This field captures any descriptors of the events.
        ///
        /// For event tickets, for example, tags might be team names, region, etc.
        tags: Option<Vec<String>>,

        /// Any extra non-reserved fields to be recorded with the booking.
        #[serde(flatten)]
        extra: Option<serde_json::Value>,
    },
}

/// Information about the user's web browser.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Browser {
    /// The user agent of the browser that is interacting with your website.
    #[serde(rename = "$user_agent")]
    pub user_agent: String,

    /// The language(s) that the client is requesting the site content be delivered in.
    ///
    /// Use [ISO-3166] format for country codes. Examples: "en", "en-us, de", "fr-CH, fr;q=0.9,
    /// en;q=0.8, de;q=0.7, *;q=0.5", etc.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$accept_language")]
    pub accept_language: Option<String>,

    /// The language(s) of the user that the delivered site content is intended for.
    ///
    /// Use [ISO-3166] format for country codes. Examples: "en", "en-us, de", "fr-CH, fr;q=0.9,
    /// en;q=0.8, de;q=0.7, *;q=0.5", etc.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$content_language")]
    pub content_language: Option<String>,

    /// Any extra non-reserved fields to be recorded with the browser.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Monetary and non-monetary rewards.
///
/// (e.g. in-game currency, stored account value, MBs storage, frequent flyer miles, etc)
/// associated with a promotion. Credit points are usually used for promotions that apply at the
/// account level. The value must be a nested JSON object populated with the appropriate
/// information to describe the credit_point. All values are required.
///
/// A `credit_point` is an object that gets included as part of promotion object. Learn more about
/// [Promotion]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPoint {
    /// The amount of credits the promotion is worth.
    #[serde(rename = "$amount")]
    pub amount: i64,

    /// The type of credit point. Particularly useful if you have multiple types of credit points
    /// that you give out. Enables us to distinguish amongst them to find patterns (e.g. days of
    /// free service, karma, frequent flyer miles, MBs of storage, etc.).
    #[serde(rename = "$credit_point_type")]
    pub credit_point_type: String,

    /// Any extra non-reserved fields to be recorded with the credit point.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Monetary discounts that are associated with a promotion.
///
/// (e.g. $25 off an order of $100 or more, 10% off, etc). Discounts are usually used for
/// promotions that apply at the order level. The value must be a nested JSON object populated with
/// the appropriate information to describe the discount. Not all sub-fields will likely apply to a
/// given discount. Populate only those that apply.
///
/// A discount is an object that gets included as part of promotion object. Learn more about [Promotion]s.
#[derive(Debug, Serialize, Deserialize)]
pub struct Discount {
    /// The percentage discount. If the discount is 10% off, you would send `0.1`.
    #[serde(rename = "$percentage_off")]
    pub percentage_off: f32,

    /// The amount of the discount that the promotion offers in micros in the base unit of the
    /// `currency_code`. 1 cent = 10,000 micros. $1.23 USD = 123 cents = 1,230,000 micros. For
    /// currencies without cents of fractional denominations, like the Japanese Yen, use 1 JPY =
    /// 1000000 micros.
    #[serde(rename = "$amount")]
    pub amount: i64,

    /// [ISO-4217] currency code for the amount. e.g., USD, CAD, HKD. If your site uses alternative
    /// currencies, like bitcoin or points systems, specify that here.
    ///
    /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
    #[serde(rename = "$currency_code")]
    pub currency_code: String,

    /// The minimum amount someone must spend in order for the promotion to be applied. The amount
    /// should be in micros in the base unit of the `currency_code`. 1 cent = 10,000 micros. $1.23
    /// USD = 123 cents = 1,230,000 micros. For currencies without cents of fractional
    /// denominations, like the Japanese Yen, use 1 JPY = 1000000 micros.
    #[serde(rename = "$minimum_purchase_amount")]
    pub minimum_purchase_amount: i64,

    /// Any extra non-reserved fields to be recorded with the discount.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// The Guest field type represents a person using a booking.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Guest {
    /// Name of the individual on the booking.
    #[serde(rename = "$name")]
    pub name: Option<String>,

    /// The email address provided for the guest.
    #[serde(rename = "$email")]
    pub email: Option<String>,

    /// The phone number provided for the guest.
    ///
    /// Provide the phone number as a string starting with the country code. Use [E.164] format or
    /// send in the standard national format of number's origin. For example: "+14155556041" or
    /// "1-415-555-6041" for a U.S. number.
    ///
    /// [E.164]: https://en.wikipedia.org/wiki/E.164
    #[serde(rename = "$phone")]
    pub phone: Option<String>,

    /// The name of the loyalty program used for this guest.
    #[serde(rename = "$loyalty_program")]
    pub loyalty_program: Option<String>,

    /// The membership id for the loyalty program used for this guest.
    #[serde(rename = "$loyalty_program_id")]
    pub loyalty_program_id: Option<String>,

    /// The date of birth of the guest. Use ISO 8601 format, e.g. "1985-03-20" or "19850320"
    #[serde(rename = "$birth_date")]
    pub birth_date: Option<String>,

    /// Any extra non-reserved fields to be recorded with the guest.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// The Image complex type represents an image hosted on your website or app,
/// typically uploaded by a user.
///
/// Used in the in one of the `Event::CreateContent` or `Event::UpdateContent`
/// events.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Image {
    /// The MD5 hash of the image file.
    ///
    /// A hexadecimal hash for a single file could look like this:
    /// `0cc175b9c0f1b6a831c399e269772661`.
    #[serde(rename = "$md5_hash")]
    pub md5_hash: Option<String>,

    /// A hyperlink to the image file.
    #[serde(rename = "$link")]
    pub link: Option<String>,

    /// The user-supplied caption with the image.
    #[serde(rename = "$description")]
    pub description: Option<String>,

    /// Any extra non-reserved fields to be recorded with the image.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Represents a product or service for sale in your business.
///
/// Generally used in the `AddItemToCart` and `RemoveItemFromCart` events.
///
/// Please note that `Item` cannot be used with `Booking`. Customers in event ticketing or travel
/// (such as OTAs, Rideshare, Vehicle rentals, Hotels, etc) should use `Booking` instead to
/// leverage Sift's specialization in Travel and Ticketing use cases.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Item {
    /// The item's unique identifier according to your systems.
    ///
    /// Use the same ID that you would use to look up items on your website's database.
    #[serde(rename = "$item_id")]
    pub item_id: Option<String>,

    /// The item's name.
    ///
    /// e.g. "Men's Running Springblade Drive Shoes, US10"
    #[serde(rename = "$product_title")]
    pub product_title: Option<String>,

    /// The item unit price in micros, in the base unit of the `currency_code`.
    ///
    /// 1 cent = 10,000 micros. $1.23 USD = 123 cents = 1,230,000 micros.
    #[serde(rename = "$price")]
    pub price: Option<Micros>,

    /// [ISO-4217] currency code for the price.
    ///
    /// [ISO-4217]: http://en.wikipedia.org/wiki/ISO_4217
    #[serde(rename = "$currency_code")]
    pub currency_code: Option<String>,

    /// Quantity of the item.
    #[serde(rename = "$quantity")]
    pub quantity: Option<u64>,

    /// If the item has a [Universal Product Code] (UPC), provide it here.
    ///
    /// [Universal Product Code]: http://en.wikipedia.org/wiki/Universal_Product_Code
    #[serde(rename = "$upc")]
    pub upc: Option<String>,

    /// If the item has a [Stock-keeping Unit ID] (SKU), provide it here.
    ///
    /// [Stock-keeping Unit ID]: http://en.wikipedia.org/wiki/Stock_keeping_unit
    #[serde(rename = "$sku")]
    pub sku: Option<String>,

    /// If the item is a book with an [International Standard Book Number] (ISBN), provide it here.
    ///
    /// [International Standard Book Number]: http://en.wikipedia.org/wiki/International_Standard_Book_Number
    #[serde(rename = "$isbn")]
    pub isbn: Option<String>,

    /// The brand name of the item.
    #[serde(rename = "$brand")]
    pub brand: Option<String>,

    /// Name of the item's manufacturer.
    #[serde(rename = "$manufacturer")]
    pub manufacturer: Option<String>,

    /// The category this item is listed under in your business.
    ///
    /// e.g. "kitchen appliance", "menswear > pants".
    #[serde(rename = "$category")]
    pub category: Option<String>,

    /// The tags used to describe this item in your business.
    ///
    /// e.g. "funny", "halloween".
    #[serde(rename = "$tags")]
    pub tags: Option<Vec<String>>,

    /// The color of the item.
    #[serde(rename = "$color")]
    pub color: Option<String>,

    /// The size of the item.
    #[serde(rename = "$size")]
    pub size: Option<String>,

    /// Any extra non-reserved fields to be recorded with the item.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Contains information about the merchant or seller providing goods or service.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantProfile {
    /// The internal identifier for the merchant or seller providing the good or service.
    #[serde(rename = "$merchant_id")]
    pub merchant_id: Option<String>,

    /// The merchant category code follows the 4-digit ISO code.
    ///
    /// Use [ISO-18245] MCC ISO Merchant Category Code.
    ///
    /// [ISO-18245]: https://en.wikipedia.org/wiki/ISO_18245
    #[serde(rename = "$merchant_category_code")]
    pub merchant_category_code: Option<String>,

    /// The name of the merchant or seller providing the good or service.
    #[serde(rename = "$merchant_name")]
    pub merchant_name: String,

    /// The address associated with the merchant of record.
    #[serde(rename = "$merchant_address")]
    pub merchant_address: Option<Address>,

    /// Any extra non-reserved fields to be recorded with the merchant profile.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Information about the specific physical location providing the good or service.
///
/// This can also be used to capture pickup, delivery locations, etc.
///
/// We use the values of the latest version of the `OrderedFrom` struct for each `order_id` for
/// reporting in your console. If you send all other fields except `zipcode` in a `CreateOrder` and
/// then send only `zipcode` in a later, associated `transaction` for the same `order_id`, we will
/// only use `zipcode` for this order in reporting and forget the initial `ordered_from` sent.
/// So, we recommend that you send this struct in either the `CreateOrder`/`UpdateOrder` events OR
/// in `Transaction` events, but not in both. Choose the time where you have the most information to
/// send or where it is easiest to include.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderedFrom {
    /// The customer's internal identifier for the specific physical location providing the good or
    /// service.
    #[serde(rename = "$store_id")]
    pub store_id: Option<String>,

    /// The address of the specific physical location providing the good or service.
    #[serde(rename = "$store_address")]
    pub store_address: Option<Address>,

    /// Any extra non-reserved fields to be recorded with the location.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Represents information about the payment methods provided by the user.
///
/// The value must be a nested object with the appropriate item subfields for the given payment
/// method. Generally used with `Event::CreateOrder` or `Event::Transaction`.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// The general type of payment being used.
    #[serde(rename = "$payment_type")]
    pub payment_type: Option<PaymentType>,

    /// The specific gateway, company, product, etc. being used to process payment.
    #[serde(rename = "$payment_gateway")]
    pub payment_gateway: Option<String>,

    /// The first six digits of the credit card number.
    ///
    /// These numbers contain information about the card issuer, the geography and other card
    /// details.
    #[serde(rename = "$card_bin")]
    pub card_bin: Option<String>,

    /// The last four digits of the credit card number.
    #[serde(rename = "$card_last4")]
    pub card_last4: Option<String>,

    /// Response code from the AVS address verification system.
    ///
    /// Used in payments involving credit cards.
    #[serde(rename = "$avs_result_code")]
    pub avs_result_code: Option<String>,

    /// Response code from the credit card company indicating if the CVV number entered matches the
    /// number on record.
    ///
    /// Used in payments involving credit cards.
    #[serde(rename = "$cvv_result_code")]
    pub cvv_result_code: Option<String>,

    /// Use `verification_status` to indicate the payment method has been verified.
    ///
    /// The value can be `Success`, `Failure` or `Pending`. For instance, if you request payment
    /// method verification from a payment processor and receive a failure set the value to
    /// `Failure`.
    #[serde(rename = "$verification_status")]
    pub verification_status: Option<PaymentMethodVerificationStatus>,

    /// This is the ABA routing number or SWIFT code used.
    #[serde(rename = "$routing_number")]
    pub routing_number: Option<String>,

    /// This is the first 6 characters of the IBAN structure as defined in [ISO 13616-1].
    ///
    /// [ISO 13616-1]: https://en.wikipedia.org/wiki/International_Bank_Account_Number
    #[serde(rename = "$shortened_iban_first6")]
    pub shortened_iban_first6: Option<String>,

    /// This is the last 4 characters of the IBAN structure as defined in [ISO 13616-1].
    ///
    /// [ISO 13616-1]: https://en.wikipedia.org/wiki/International_Bank_Account_Number
    #[serde(rename = "$shortened_iban_last4")]
    pub shortened_iban_last4: Option<String>,

    /// Used to indicate if a end-user/customer has provided authorization to collect future
    /// payments via Sepa Direct Debit.
    #[serde(rename = "$sepa_direct_debit_mandate")]
    pub sepa_direct_debit_mandate: Option<bool>,

    /// In case of a declined payment, response code received from the payment processor indicating
    /// the reason for the decline.
    #[serde(rename = "$decline_reason_code")]
    pub decline_reason_code: Option<String>,

    /// Payer ID returned by Paypal.
    #[serde(rename = "$paypal_payer_id")]
    pub paypal_payer_id: Option<String>,

    /// Payer email returned by Paypal.
    #[serde(rename = "$paypal_payer_email")]
    pub paypal_payer_email: Option<String>,

    /// Payer status returned by Paypal.
    #[serde(rename = "$paypal_payer_status")]
    pub paypal_payer_status: Option<String>,

    /// Payer address status returned by Paypal.
    #[serde(rename = "$paypal_address_status")]
    pub paypal_address_status: Option<String>,

    /// Seller protection eligibility returned by Paypal.
    #[serde(rename = "$paypal_protection_eligibility")]
    pub paypal_protection_eligibility: Option<String>,

    /// Payment status returned by Paypal.
    #[serde(rename = "$paypal_payment_status")]
    pub paypal_payment_status: Option<String>,

    /// CVC verification result returned by Stripe.
    #[serde(rename = "$stripe_cvc_check")]
    pub stripe_cvc_check: Option<String>,

    /// Address line 1 verification result returned by Stripe.
    #[serde(rename = "$stripe_address_line1_check")]
    pub stripe_address_line1_check: Option<String>,

    /// Address line 2 verification result returned by Stripe.
    #[serde(rename = "$stripe_address_line2_check")]
    pub stripe_address_line2_check: Option<String>,

    /// Address zip code verification result returned by Stripe.
    #[serde(rename = "$stripe_address_zip_check")]
    pub stripe_address_zip_check: Option<String>,

    /// Funding source returned by Stripe.
    #[serde(rename = "$stripe_funding")]
    pub stripe_funding: Option<String>,

    /// Card brand returned by Stripe.
    #[serde(rename = "$stripe_brand")]
    pub stripe_brand: Option<String>,

    /// Full name of the user associated with the account.
    #[serde(rename = "$account_holder_name")]
    pub account_holder_name: Option<String>,

    /// The last 5 digits of the account number associated with an ACH or a Wire transaction.
    #[serde(rename = "$account_number_last5")]
    pub account_number_last5: Option<String>,

    /// Name of the financial institution used.
    #[serde(rename = "$bank_name")]
    pub bank_name: Option<String>,

    /// Two-digit [ISO-3166] code for the bank country of origin.
    ///
    /// [ISO-3166]: http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    #[serde(rename = "$bank_country")]
    pub bank_country: Option<String>,

    /// Any extra non-reserved fields to be recorded with the payment method.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Promotions such as referrals, coupons, free trials, etc.
///
/// Populate with the appropriate information to describe the promotion. Not all sub-fields will
/// likely apply to a given promotion. Populate only those that apply.
///
/// A promotion can be added when creating or updating an account, creating or updating an order,
/// or on its own using the `AddPromotion` event. The promotion struct supports both monetary (e.g.
/// $25 coupon on first order) and non-monetary (e.g. "1000 in game points to refer a friend")
/// types.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Promotion {
    /// The ID within your system that you use to represent this promotion. This ID is ideally
    /// unique to the promotion across users (e.g. "BackToSchool2016").
    #[serde(rename = "$promotion_id")]
    pub promotion_id: Option<String>,

    /// The status of the addition of promotion to an account. Best used with the $add_promotion
    /// event. This way you can pass to Sift both successful and failed attempts when using a
    /// promotion. May be useful in spotting potential abuse. Allowed Values $success $failure
    #[serde(rename = "$status")]
    pub status: Option<String>,

    /// When adding a promotion fails, use this to describe why it failed. Allowed Values
    ///     $already_used
    ///     $invalid_code
    ///     $not_applicable
    ///     $expired
    #[serde(rename = "$failure_reason")]
    pub failure_reason: Option<String>,

    /// Freeform text to describe the promotion.
    #[serde(rename = "$description")]
    pub description: Option<String>,

    /// The unique account ID of the user who referred the user to this promotion. Note: User IDs
    /// are case sensitive.
    #[serde(rename = "$referrer_user_id")]
    pub referrer_user_id: Option<String>,

    /// The `discount` field type generically models monetary discounts that are associated with a
    /// promotion (e.g. $25 off an order of $100 or more, 10% off, etc). Most promotions likely
    /// require a discount object or credit_point object to describe them, though both can be set
    /// for a given promotion.
    #[serde(rename = "$discount")]
    pub discount: Option<Discount>,

    /// The credit_point field type generically models monetary and non-monetary rewards (e.g. in-game currency, stored account value, MBs storage, frequent flyer miles, etc.) for a promotion. Most promotions likely require a credit_point object or discount object to describe them, though both can be set for a given promotion.
    #[serde(rename = "$credit_point")]
    pub credit_point: Option<CreditPoint>,

    /// Any extra non-reserved fields to be recorded with the promotion.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// Detailed information about the components of a travel [Booking].
///
/// We recommend sending at least one segment for the following booking_types:
///
/// * [Booking::Flight]
/// * [Booking::Bus]
/// * [Booking::Rideshare]
/// * [Booking::Vehicle]
/// * [Booking::Cruise]
///
/// Even if there's only a single segment associated with the booking, use segment to send valuable
/// information about the trip.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Segment {
    /// The address of the start of the journey.
    ///
    /// This could be the pickup address for rideshare or the train station address for a rail
    /// journey. For flights, you can use the `departure_airport_code` in lieu of this field.
    #[serde(rename = "$departure_address")]
    pub departure_address: Option<Address>,

    /// The address of the end of the journey.
    ///
    /// This could be the drop-off address for rideshare or the train station address for a rail
    /// journey. For flights, you can use the `arrival_airport_code` in lieu of this field.
    #[serde(rename = "$arrival_address")]
    pub arrival_address: Option<Address>,

    /// The start time of this segment of the journey.
    ///
    /// This may be departure time for a flight, the expected pickup time for a rideshare, etc.
    #[serde(
        rename = "$start_time",
        deserialize_with = "deserialize_opt_ms",
        serialize_with = "serialize_opt_ms"
    )]
    pub start_time: Option<SystemTime>,

    /// The finish time of this segment of the journey.
    ///
    /// This may be departure time for a flight, the expected pickup time for a rideshare, etc.
    #[serde(
        rename = "$end_time",
        deserialize_with = "deserialize_opt_ms",
        serialize_with = "serialize_opt_ms"
    )]
    pub end_time: Option<SystemTime>,

    /// An identifier for the journey.
    ///
    /// This could be the flight number ("UA 454"), the car license plate number ("6XYZ123"), etc.
    #[serde(rename = "$vessel_number")]
    pub vessel_number: Option<String>,

    /// The [IATA] code for the departure airport.
    ///
    /// For example: "SFO"
    ///
    /// [IATA]: https://en.wikipedia.org/wiki/IATA_airport_code
    #[serde(rename = "$departure_airport_code")]
    pub departure_airport_code: Option<String>,

    /// The [IATA] code for the arrival airport.
    ///
    /// For example: "SFO"
    ///
    /// [IATA]: https://en.wikipedia.org/wiki/IATA_airport_code
    #[serde(rename = "$arrival_airport_code")]
    pub arrival_airport_code: Option<String>,

    /// A description of the class of travel.
    ///
    /// Eg. "Premium Economy", "Pool", "E3".
    #[serde(rename = "$fare_class")]
    pub fare_class: Option<String>,

    /// Any extra non-reserved fields to be recorded with the segment.
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}
