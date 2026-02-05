//! Media library for Underlay applications.
//!
//! This crate provides a complete media management solution including:
//!
//! - **Domain types**: Media items, versions, renditions, and usage tracking
//! - **Repository trait**: Abstract interface for media storage operations
//! - **PostgreSQL implementation**: Production-ready database backend (with `postgres` feature)
//! - **Rendition service**: Automatic thumbnail/preview generation (with `renditions` feature)
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use underlay_media::{
//!     MediaRepository, MediaId, CreateMediaInput, MediaKind, MediaVisibility,
//!     PostgresMediaRepository,
//! };
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
//! - `postgres` - PostgreSQL repository implementation using sqlx
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
pub mod repository;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "renditions")]
pub mod renditions;

// Re-export main types for convenience
pub use domain::{
    // Identifiers
    MediaId,
    MediaRenditionId,
    MediaVersionId,
    // Entities
    Media,
    MediaRendition,
    MediaSummary,
    MediaUsage,
    MediaVersion,
    // Enums (from underlay-db)
    MediaKind,
    MediaTypeParseError,
    MediaVersionState,
    MediaVisibility,
    RenditionType,
    // Input types
    CreateMediaInput,
    CreateRenditionInput,
    FinalizeUploadInput,
    ListMediaParams,
    UpdateMediaInput,
    // Utility
    detect_media_kind_from_mime_type,
};

pub use error::{MediaError, MediaResult};
pub use repository::{MediaRepository, MediaRepositoryExt};

#[cfg(feature = "postgres")]
pub use postgres::{PostgresMediaConfig, PostgresMediaRepository};

#[cfg(feature = "renditions")]
pub use renditions::{RenditionConfig, RenditionResult, RenditionService};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re_exports() {
        // Ensure core types are accessible
        let _id = MediaId::new();
        let _version_id = MediaVersionId::new();
        let _rendition_id = MediaRenditionId::new();
    }

    #[test]
    fn test_media_kind_re_export() {
        assert_eq!(MediaKind::Image.as_str(), "image");
        assert_eq!(MediaKind::Pdf.as_str(), "pdf");
    }

    #[test]
    fn test_detect_media_kind() {
        assert_eq!(
            detect_media_kind_from_mime_type("image/jpeg"),
            Some(MediaKind::Image)
        );
        assert_eq!(
            detect_media_kind_from_mime_type("application/pdf"),
            Some(MediaKind::Pdf)
        );
        assert_eq!(detect_media_kind_from_mime_type("text/plain"), None);
    }
}
