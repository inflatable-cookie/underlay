//! Blob storage infrastructure for Underlay applications.
//!
//! This crate provides a flexible, adapter-based blob storage system that supports
//! multiple backends (AWS S3, MinIO, local filesystem, etc.) for storing and retrieving
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
//! - `S3Adapter` - AWS S3 or S3-compatible storage such as MinIO (requires `s3` feature)
//! - `LocalAdapter` - Local filesystem (legacy/utility seam, requires `local` feature)
//!
//! All adapters implement the `BlobAdapter` trait.
//!
//! # Features
//!
//! - `s3` - Enable the AWS S3 adapter (adds `aws-sdk-s3` dependency)
//! - `local` - Enable the local filesystem adapter
//!
//! # Security Note
//!
//! Underlay's preferred local-development blob path is the S3 adapter pointed at
//! a bundled MinIO instance. The `LocalAdapter` remains available as a narrow
//! filesystem seam, but browser-facing `dev-uploads` style flows should not be
//! the default consumer pattern anymore.

mod adapter;
pub mod adapters;
mod config;
mod error;
mod types;

// Re-export main types
pub use adapter::{BlobAdapter, NoopAdapter};
pub use config::MediaConfig;
pub use error::{BlobError, BlobResult};
pub use types::{DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest};

// Re-export adapters based on features
#[cfg(feature = "s3")]
pub use adapters::{S3Adapter, S3Config};

#[cfg(feature = "local")]
pub use adapters::{LocalAdapter, LocalConfig};
