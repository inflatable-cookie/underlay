//! Blob storage infrastructure for Underlay applications.
//!
//! This crate provides a flexible, adapter-based blob storage system that supports
//! multiple backends (AWS S3, local filesystem, etc.) for storing and retrieving
//! binary objects like images and PDFs.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use underlay_blob::{BlobAdapter, NoopAdapter, UploadRequest};
//!
//! // Create an adapter (using noop for testing)
//! let adapter = Arc::new(NoopAdapter::new());
//!
//! // Initiate an upload
//! let request = UploadRequest::new("media/123/photo.jpg", "image/jpeg", 1024);
//! let plan = adapter.initiate_upload(request).await?;
//!
//! // Client uploads to plan.upload_url...
//!
//! // Finalise the upload
//! let stored = adapter.finalise_upload("media/123/photo.jpg").await?;
//!
//! // Get URLs
//! let public = adapter.public_url("media/123/photo.jpg");
//! let signed = adapter.signed_download_url(DownloadRequest::new("media/123/photo.jpg")).await?;
//! ```
//!
//! # Adapter Pattern
//!
//! The crate uses the adapter pattern to support different storage backends:
//!
//! - `NoopAdapter` - Does nothing (for testing)
//! - `S3Adapter` - AWS S3 or S3-compatible storage (requires `s3` feature)
//! - `LocalAdapter` - Local filesystem (for development, requires `local` feature)
//!
//! All adapters implement the `BlobAdapter` trait.
//!
//! # Features
//!
//! - `s3` - Enable the AWS S3 adapter (adds `aws-sdk-s3` dependency)
//! - `local` - Enable the local filesystem adapter (for development only)
//!
//! # Security Note
//!
//! The `LocalAdapter` is intended for development only. Any HTTP endpoints that
//! serve files from the local adapter **must be completely removed** in production
//! builds. Use compile-time feature flags to ensure this.

mod adapter;
pub mod adapters;
mod error;
mod types;

// Re-export main types
pub use adapter::{BlobAdapter, NoopAdapter};
pub use error::{BlobError, BlobResult};
pub use types::{
    DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

// Re-export adapters based on features
#[cfg(feature = "s3")]
pub use adapters::{S3Adapter, S3Config};

#[cfg(feature = "local")]
pub use adapters::{LocalAdapter, LocalConfig};
