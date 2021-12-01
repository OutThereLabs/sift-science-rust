//! A sift science client
#![warn(missing_docs, missing_debug_implementations, unreachable_pub, unused)]

mod client;
mod common;
mod error;
pub mod events;
pub mod labels;
pub mod score;
pub mod verification;

#[cfg(feature = "awc")]
pub use client::AwcClient;
#[cfg(feature = "reqwest")]
pub use client::ReqwestClient;
pub use client::{Client, HttpClient};
pub use common::AbuseType;
pub use error::{Error, Result};
