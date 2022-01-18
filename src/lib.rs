//! A sift science client
#![warn(missing_docs, missing_debug_implementations, unreachable_pub, unused)]

mod client;
mod common;
#[cfg(feature = "decisions")]
pub mod decisions;
mod error;
pub mod events;
#[cfg(feature = "labels")]
pub mod labels;
#[cfg(feature = "score")]
pub mod score;
#[cfg(feature = "verification")]
pub mod verification;
#[cfg(feature = "webhooks")]
pub mod webhooks;

#[cfg(feature = "awc")]
pub use client::AwcClient;
#[cfg(feature = "reqwest")]
pub use client::ReqwestClient;
pub use client::{Client, HttpClient};
pub use common::AbuseType;
pub use error::{Error, Result};
