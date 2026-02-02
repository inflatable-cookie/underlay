//! Blob storage adapter implementations.

#[cfg(feature = "s3")]
mod s3;

#[cfg(feature = "local")]
mod local;

#[cfg(feature = "s3")]
pub use s3::{S3Adapter, S3Config};

#[cfg(feature = "local")]
pub use local::{LocalAdapter, LocalConfig};
