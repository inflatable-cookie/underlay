//! Local filesystem blob storage adapter for development.
//!
//! **WARNING**: This adapter is for development only and should NOT be used in production.
//! The associated file-serving endpoint must be completely removed from production builds.

mod adapter;
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
