//! Local filesystem blob storage adapter for development.
//!
//! **WARNING**: This adapter is for development only and should NOT be used in production.
//! The associated file-serving endpoint must be completely removed from production builds.

mod adapter;
mod bounded;
mod config;
mod mime;
mod path;

pub use adapter::LocalAdapter;
pub use config::LocalConfig;

#[cfg(test)]
use crate::{BlobAdapter, BlobError};
#[cfg(test)]
use mime::guess_content_type;
#[cfg(test)]
use tokio::fs;

#[cfg(test)]
#[path = "../tests/adapters/local_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/adapters/local_bounded_promotion_tests.rs"]
mod bounded_promotion_tests;

#[cfg(test)]
#[path = "../tests/adapters/local_containment_race_tests.rs"]
mod containment_race_tests;
