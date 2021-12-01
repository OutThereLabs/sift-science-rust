use serde::{de, ser, Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Type of abuse tracked by a sift science.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbuseType {
    /// Account takeover
    AccountTakeover,

    /// Account abuse
    AccountAbuse,

    /// Content abuse
    ContentAbuse,

    /// Payment abuse
    PaymentAbuse,

    /// Promo abuse
    PromoAbuse,
}

impl fmt::Display for AbuseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbuseType::AccountAbuse => write!(f, "account_abuse"),
            AbuseType::AccountTakeover => write!(f, "account_takeover"),
            AbuseType::ContentAbuse => write!(f, "content_abuse"),
            AbuseType::PaymentAbuse => write!(f, "payment_abuse"),
            AbuseType::PromoAbuse => write!(f, "promo_abuse"),
        }
    }
}

// Serialize to optional comma separated list for query params
//
// Required as array support in query params is **explicitly** not supported:
// https://github.com/nox/serde_urlencoded/issues/75
pub(crate) fn abuse_type_serialize<S>(
    types: &Option<Vec<AbuseType>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match types {
        Some(abuses) => {
            let joined = abuses
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            serializer.serialize_some(&joined)
        }
        None => serializer.serialize_none(),
    }
}

// Deserialize optional system time as timestamp in ms
pub(crate) fn deserialize_opt_ms<'de, D>(d: D) -> Result<Option<SystemTime>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let maybe_ms = Option::<u64>::deserialize(d)?;
    let time = maybe_ms.and_then(|ms| UNIX_EPOCH.checked_add(Duration::from_millis(ms)));

    Ok(time)
}

// Deserialize optional system time as timestamp in ms
pub(crate) fn deserialize_ms<'de, D>(d: D) -> Result<SystemTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(deserialize_opt_ms(d)?.unwrap_or(UNIX_EPOCH))
}

// Serialize system time as timestamp in ms
pub(crate) fn serialize_opt_ms<S>(time: &Option<SystemTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    if let Some(time) = time {
        serialize_ms(time, s)
    } else {
        s.serialize_none()
    }
}

// Serialize system time as timestamp in ms
pub(crate) fn serialize_ms<S>(time: &SystemTime, s: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    s.serialize_u64(
        time.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
    )
}
