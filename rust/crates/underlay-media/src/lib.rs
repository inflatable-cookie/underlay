//! Media library for Underlay applications.
//!
//! This crate provides a complete media management solution including:
//!
//! - **Domain types**: Media items, versions, renditions, and usage tracking
//! - **Repository trait**: Abstract interface for media storage operations
//! - **PostgreSQL implementation**: Production-ready database backend via `underlay-media-postgres`
//! - **Rendition service**: Automatic thumbnail/preview generation (with `renditions` feature)
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use underlay_media::{
//!     MediaRepository, MediaId, CreateMediaInput, MediaKind, MediaVisibility,
//! };
//! use underlay_media_postgres::PostgresMediaRepository;
//!
//! // Create a repository
//! let repo = PostgresMediaRepository::new(pool);
//!
//! // Create a media item
//! let input = CreateMediaInput {
//!     kind: MediaKind::Image,
//!     visibility: MediaVisibility::Public,
//!     title: "My Photo".to_string(),
//!     original_filename: Some("photo.jpg".to_string()),
//!     alt_text: None,
//! };
//! let media = repo.create_media(input, Some(user_id)).await?;
//!
//! // Create a version and upload
//! let version = repo.create_version(media.id, Some(user_id)).await?;
//! // ... upload to blob storage ...
//! let version = repo.finalize_version(version.id, finalize_input).await?;
//! ```
//!
//! # Features
//!
//! - `renditions` - Automatic thumbnail and preview generation
//! - `full` - All features enabled
//!
//! # Architecture
//!
//! The media library follows a versioned content model:
//!
//! ```text
//! Media (stable reference)
//!   └── MediaVersion (immutable content snapshot)
//!         └── MediaRendition (derived images)
//!   └── MediaUsage (reference tracking)
//! ```
//!
//! - **Media** items are stable references that content can link to
//! - **Versions** are immutable - to update content, create a new version
//! - **Renditions** are automatically generated thumbnails/previews
//! - **Usages** track where media is referenced for safe deletion

pub mod domain;
pub mod error;
pub mod image;
#[cfg(feature = "nightfire")]
pub mod nightfire;
pub mod repository;
pub mod storage;
pub mod sync;

#[cfg(feature = "renditions")]
pub mod renditions;

pub use underlay_blob::{BlobObjectKey, BlobObjectKeyError};

// Re-export main types for convenience
pub use domain::{
    // Utility
    detect_media_kind_from_mime_type,
    // Input types
    CreateMediaInput,
    CreateRenditionInput,
    FinalizeUploadInput,
    ListMediaParams,
    // Entities
    Media,
    MediaContentKind,
    // Identifiers
    MediaId,
    // Enums (from underlay-db)
    MediaKind,
    MediaLocatorKind,
    MediaRendition,
    MediaRenditionId,
    MediaSummary,
    MediaTypeParseError,
    MediaUsage,
    MediaUsageEdge,
    MediaUsageEdgeInput,
    MediaUsageEdgeKey,
    MediaUsageFieldPayload,
    MediaUsageProvenanceKind,
    MediaUsageRole,
    MediaVersion,
    MediaVersionId,
    MediaVersionState,
    MediaVisibility,
    MigratedAttachmentBinding,
    MigratedAttachmentBindingInput,
    MigratedAttachmentIdentity,
    RenditionType,
    UpdateMediaInput,
};

pub use error::{MediaError, MediaResult};
pub use repository::{MediaRepository, MediaRepositoryExt, MediaUsageRepository};

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
