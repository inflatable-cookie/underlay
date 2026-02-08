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
pub mod image;
pub mod repository;
pub mod storage;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "postgres")]
mod postgres_rows;

#[cfg(feature = "renditions")]
pub mod renditions;

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
    // Identifiers
    MediaId,
    // Enums (from underlay-db)
    MediaKind,
    MediaRendition,
    MediaRenditionId,
    MediaSummary,
    MediaTypeParseError,
    MediaUsage,
    MediaVersion,
    MediaVersionId,
    MediaVersionState,
    MediaVisibility,
    RenditionType,
    UpdateMediaInput,
};

pub use error::{MediaError, MediaResult};
pub use image::{
    calculate_thumbnail_dimensions, format_from_mime, generate_square_thumbnail,
    generate_thumbnail, is_supported_image, mime_from_format, ImageError, ThumbnailConfig,
    ThumbnailResult,
};
pub use repository::{MediaRepository, MediaRepositoryExt};
pub use storage::{
    mime_to_extension, rendition_key, version_filename, version_key, StorageKeyConfig,
    StorageKeyGenerator,
};

#[cfg(feature = "postgres")]
pub use postgres::{PostgresMediaConfig, PostgresMediaRepository};

#[cfg(feature = "renditions")]
pub use renditions::{RenditionConfig, RenditionResult, RenditionService};

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

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

    #[test]
    fn test_storage_key_re_exports() {
        let media_id = Uuid::nil();
        let version_id = Uuid::nil();

        // Test convenience functions
        let key = version_key(media_id, version_id, "photo.jpg");
        assert!(key.contains("/versions/"));
        assert!(key.ends_with("/photo.jpg"));

        let rend_key = rendition_key(media_id, version_id, "thumb");
        assert!(rend_key.contains("/renditions/"));
        assert!(rend_key.ends_with("/thumb.jpg"));

        // Test filename generation
        assert_eq!(version_filename("image/jpeg", None), "file.jpg");
        assert_eq!(
            version_filename("image/png", Some("custom.png")),
            "custom.png"
        );

        // Test extension mapping
        assert_eq!(mime_to_extension("image/jpeg"), "jpg");
        assert_eq!(mime_to_extension("application/pdf"), "pdf");
    }

    #[test]
    fn test_storage_key_generator_re_export() {
        let config = StorageKeyConfig::with_prefix("uploads");
        let generator = StorageKeyGenerator::new(config);
        assert_eq!(generator.config().base_prefix, "uploads");
    }
}
