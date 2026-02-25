//! Media configuration with sensible defaults.
//!
//! This module provides a reusable configuration structure for media handling
//! that consuming applications can use with default values or customize as needed.
//!
//! # Example
//!
//! ```rust
//! use underlay_blob::MediaConfig;
//!
//! // Use all defaults (50 MB max, 300px thumbnails)
//! let config = MediaConfig::default();
//!
//! // Override specific values using builder methods
//! let config = MediaConfig::default()
//!     .max_file_size_mb(100)
//!     .thumbnail_dimension(400);
//!
//! // Or use struct syntax
//! let config = MediaConfig {
//!     max_file_size_bytes: 100 * 1024 * 1024,
//!     thumbnail_max_dimension: 400,
//! };
//! ```

/// Configuration for media handling with sensible defaults.
///
/// Provides default values that work well for most applications:
/// - 50 MB maximum file size
/// - 300 pixel maximum thumbnail dimension
///
/// Applications can override any value using the builder methods or
/// struct field assignment.
#[derive(Debug, Clone)]
pub struct MediaConfig {
    /// Maximum allowed file size for media uploads in bytes.
    ///
    /// Default: 50 MB (52,428,800 bytes)
    pub max_file_size_bytes: u64,

    /// Maximum thumbnail dimension (width or height) in pixels.
    ///
    /// When generating thumbnails, the image is scaled to fit within
    /// a square of this size while preserving aspect ratio.
    ///
    /// Default: 300 px
    pub thumbnail_max_dimension: u32,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 50 * 1024 * 1024, // 50 MB
            thumbnail_max_dimension: 300,
        }
    }
}

impl MediaConfig {
    /// Create a new config with all defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum file size for uploads in megabytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_blob::MediaConfig;
    ///
    /// let config = MediaConfig::default().max_file_size_mb(100); // 100 MB
    /// assert_eq!(config.max_file_size_bytes, 100 * 1024 * 1024);
    /// ```
    pub fn max_file_size_mb(mut self, mb: u64) -> Self {
        self.max_file_size_bytes = mb * 1024 * 1024;
        self
    }

    /// Set the maximum file size for uploads in bytes.
    ///
    /// Use this for precise control; otherwise prefer `max_file_size_mb`.
    pub fn max_file_size_bytes(mut self, bytes: u64) -> Self {
        self.max_file_size_bytes = bytes;
        self
    }

    /// Set the maximum thumbnail dimension in pixels.
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_blob::MediaConfig;
    ///
    /// let config = MediaConfig::default().thumbnail_dimension(400);
    /// assert_eq!(config.thumbnail_max_dimension, 400);
    /// ```
    pub fn thumbnail_dimension(mut self, pixels: u32) -> Self {
        self.thumbnail_max_dimension = pixels;
        self
    }

    /// Check if a file size is within the allowed limit.
    ///
    /// Returns `true` if the size is acceptable, `false` if it exceeds the limit.
    pub fn is_size_allowed(&self, size_bytes: u64) -> bool {
        size_bytes <= self.max_file_size_bytes
    }

    /// Get the maximum file size formatted as a human-readable string.
    ///
    /// Returns values like "50 MB" or "100 MB".
    pub fn max_file_size_display(&self) -> String {
        let mb = self.max_file_size_bytes / (1024 * 1024);
        if mb * 1024 * 1024 == self.max_file_size_bytes {
            format!("{} MB", mb)
        } else {
            format!("{} bytes", self.max_file_size_bytes)
        }
    }
}

#[cfg(test)]
#[path = "tests/config_tests.rs"]
mod tests;
