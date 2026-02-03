//! Media Library types for Underlay.
//!
//! Generic types for media items that can be shared across consuming applications.
//! These types match the TypeScript definitions in `@decodelabs/underlay/patterns`.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// ============================================================================
// Enums
// ============================================================================

/// Type of media item.
///
/// Matches TypeScript `MediaKind` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    /// Image file (JPEG, PNG, WebP, etc.)
    Image,
    /// PDF document
    Pdf,
}

impl MediaKind {
    /// Get the string representation of the kind.
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaKind::Image => "image",
            MediaKind::Pdf => "pdf",
        }
    }

    /// Get a human-readable label for the kind.
    pub fn label(&self) -> &'static str {
        match self {
            MediaKind::Image => "Image",
            MediaKind::Pdf => "PDF",
        }
    }

    /// All available media kinds.
    pub const fn all() -> &'static [MediaKind] {
        &[MediaKind::Image, MediaKind::Pdf]
    }
}

impl fmt::Display for MediaKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for MediaKind {
    type Err = MediaTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "image" => Ok(MediaKind::Image),
            "pdf" => Ok(MediaKind::Pdf),
            _ => Err(MediaTypeParseError::InvalidMediaKind(s.to_string())),
        }
    }
}

/// Visibility/access level of media.
///
/// Matches TypeScript `MediaVisibility` enum.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaVisibility {
    /// Publicly accessible
    #[default]
    Public,
    /// Restricted access (requires authentication/authorization)
    Restricted,
}

impl MediaVisibility {
    /// Get the string representation of the visibility.
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaVisibility::Public => "public",
            MediaVisibility::Restricted => "restricted",
        }
    }

    /// Get a human-readable label for the visibility.
    pub fn label(&self) -> &'static str {
        match self {
            MediaVisibility::Public => "Public",
            MediaVisibility::Restricted => "Restricted",
        }
    }

    /// All available visibility levels.
    pub const fn all() -> &'static [MediaVisibility] {
        &[MediaVisibility::Public, MediaVisibility::Restricted]
    }
}

impl fmt::Display for MediaVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for MediaVisibility {
    type Err = MediaTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(MediaVisibility::Public),
            "restricted" => Ok(MediaVisibility::Restricted),
            _ => Err(MediaTypeParseError::InvalidMediaVisibility(s.to_string())),
        }
    }
}

/// State of a media version in the upload lifecycle.
///
/// Matches TypeScript `MediaVersionState` enum.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaVersionState {
    /// Upload in progress
    #[default]
    Uploading,
    /// Upload complete, version is ready for use
    Ready,
    /// Upload failed
    Failed,
    /// Version is being permanently deleted
    Purging,
}

impl MediaVersionState {
    /// Get the string representation of the state.
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaVersionState::Uploading => "uploading",
            MediaVersionState::Ready => "ready",
            MediaVersionState::Failed => "failed",
            MediaVersionState::Purging => "purging",
        }
    }

    /// Get a human-readable label for the state.
    pub fn label(&self) -> &'static str {
        match self {
            MediaVersionState::Uploading => "Uploading",
            MediaVersionState::Ready => "Ready",
            MediaVersionState::Failed => "Failed",
            MediaVersionState::Purging => "Purging",
        }
    }

    /// Check if this state represents a terminal (final) state.
    pub fn is_terminal(&self) -> bool {
        matches!(self, MediaVersionState::Ready | MediaVersionState::Failed)
    }

    /// Check if this state represents a ready/usable version.
    pub fn is_ready(&self) -> bool {
        matches!(self, MediaVersionState::Ready)
    }

    /// All available version states.
    pub const fn all() -> &'static [MediaVersionState] {
        &[
            MediaVersionState::Uploading,
            MediaVersionState::Ready,
            MediaVersionState::Failed,
            MediaVersionState::Purging,
        ]
    }
}

impl fmt::Display for MediaVersionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for MediaVersionState {
    type Err = MediaTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "uploading" => Ok(MediaVersionState::Uploading),
            "ready" => Ok(MediaVersionState::Ready),
            "failed" => Ok(MediaVersionState::Failed),
            "purging" => Ok(MediaVersionState::Purging),
            _ => Err(MediaTypeParseError::InvalidMediaVersionState(s.to_string())),
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Detect media kind from a MIME type string.
///
/// Returns `None` if the MIME type doesn't map to a known media kind.
pub fn detect_media_kind_from_mime_type(mime_type: &str) -> Option<MediaKind> {
    if mime_type.starts_with("image/") {
        Some(MediaKind::Image)
    } else if mime_type == "application/pdf" {
        Some(MediaKind::Pdf)
    } else {
        None
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Error type for parsing media type strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaTypeParseError {
    /// Invalid media kind value
    InvalidMediaKind(String),
    /// Invalid media visibility value
    InvalidMediaVisibility(String),
    /// Invalid media version state value
    InvalidMediaVersionState(String),
}

impl fmt::Display for MediaTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaTypeParseError::InvalidMediaKind(s) => {
                write!(f, "invalid media kind: '{}'", s)
            }
            MediaTypeParseError::InvalidMediaVisibility(s) => {
                write!(f, "invalid media visibility: '{}'", s)
            }
            MediaTypeParseError::InvalidMediaVersionState(s) => {
                write!(f, "invalid media version state: '{}'", s)
            }
        }
    }
}

impl std::error::Error for MediaTypeParseError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_kind_serialization() {
        assert_eq!(serde_json::to_string(&MediaKind::Image).unwrap(), "\"image\"");
        assert_eq!(serde_json::to_string(&MediaKind::Pdf).unwrap(), "\"pdf\"");
    }

    #[test]
    fn test_media_kind_deserialization() {
        assert_eq!(
            serde_json::from_str::<MediaKind>("\"image\"").unwrap(),
            MediaKind::Image
        );
        assert_eq!(
            serde_json::from_str::<MediaKind>("\"pdf\"").unwrap(),
            MediaKind::Pdf
        );
    }

    #[test]
    fn test_media_kind_from_str() {
        assert_eq!("image".parse::<MediaKind>().unwrap(), MediaKind::Image);
        assert_eq!("IMAGE".parse::<MediaKind>().unwrap(), MediaKind::Image);
        assert_eq!("pdf".parse::<MediaKind>().unwrap(), MediaKind::Pdf);
        assert!("invalid".parse::<MediaKind>().is_err());
    }

    #[test]
    fn test_media_visibility_serialization() {
        assert_eq!(
            serde_json::to_string(&MediaVisibility::Public).unwrap(),
            "\"public\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVisibility::Restricted).unwrap(),
            "\"restricted\""
        );
    }

    #[test]
    fn test_media_visibility_from_str() {
        assert_eq!(
            "public".parse::<MediaVisibility>().unwrap(),
            MediaVisibility::Public
        );
        assert_eq!(
            "restricted".parse::<MediaVisibility>().unwrap(),
            MediaVisibility::Restricted
        );
        assert!("invalid".parse::<MediaVisibility>().is_err());
    }

    #[test]
    fn test_media_version_state_serialization() {
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Uploading).unwrap(),
            "\"uploading\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Ready).unwrap(),
            "\"ready\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Failed).unwrap(),
            "\"failed\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Purging).unwrap(),
            "\"purging\""
        );
    }

    #[test]
    fn test_media_version_state_from_str() {
        assert_eq!(
            "uploading".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Uploading
        );
        assert_eq!(
            "ready".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Ready
        );
        assert_eq!(
            "failed".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Failed
        );
        assert_eq!(
            "purging".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Purging
        );
        assert!("invalid".parse::<MediaVersionState>().is_err());
    }

    #[test]
    fn test_media_version_state_is_terminal() {
        assert!(!MediaVersionState::Uploading.is_terminal());
        assert!(MediaVersionState::Ready.is_terminal());
        assert!(MediaVersionState::Failed.is_terminal());
        assert!(!MediaVersionState::Purging.is_terminal());
    }

    #[test]
    fn test_detect_media_kind_from_mime_type() {
        assert_eq!(
            detect_media_kind_from_mime_type("image/jpeg"),
            Some(MediaKind::Image)
        );
        assert_eq!(
            detect_media_kind_from_mime_type("image/png"),
            Some(MediaKind::Image)
        );
        assert_eq!(
            detect_media_kind_from_mime_type("application/pdf"),
            Some(MediaKind::Pdf)
        );
        assert_eq!(detect_media_kind_from_mime_type("text/plain"), None);
        assert_eq!(detect_media_kind_from_mime_type("video/mp4"), None);
    }
}
