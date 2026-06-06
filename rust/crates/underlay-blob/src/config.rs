//! Media upload configuration with reusable defaults.

/// Configuration for media handling.
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
