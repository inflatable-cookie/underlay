//! Blob upload configuration with reusable defaults.

use crate::error::{BlobError, BlobResult};
use crate::types::UploadRequest;

/// Default server-side MIME allowlist for uploads.
///
/// Deliberately excludes active content: no `image/svg+xml` (scriptable),
/// no `text/html`, no `application/javascript`. Consumers that genuinely
/// need those must opt in via `with_allowed_content_types` and serve the
/// objects as attachments or from a sandboxed origin.
pub const DEFAULT_ALLOWED_CONTENT_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "image/avif",
    "application/pdf",
];

/// Configuration for blob upload limits.
#[derive(Debug, Clone)]
pub struct BlobUploadConfig {
    /// Maximum allowed file size for blob uploads in bytes.
    ///
    /// Default: 50 MB (52,428,800 bytes)
    max_file_size_bytes: u64,
    /// Server-side MIME allowlist. Empty means nothing is allowed.
    allowed_content_types: Vec<String>,
}

impl Default for BlobUploadConfig {
    fn default() -> Self {
        Self {
            max_file_size_bytes: 50 * 1024 * 1024, // 50 MB
            allowed_content_types: DEFAULT_ALLOWED_CONTENT_TYPES
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl BlobUploadConfig {
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

    /// Return the maximum allowed file size in bytes.
    pub fn max_file_size_bytes_limit(&self) -> u64 {
        self.max_file_size_bytes
    }

    /// Check if a file size is within the allowed limit.
    ///
    /// Returns `true` if the size is acceptable, `false` if it exceeds the limit.
    pub fn is_size_allowed(&self, size_bytes: u64) -> bool {
        size_bytes <= self.max_file_size_bytes
    }

    /// Replace the server-side MIME allowlist.
    pub fn with_allowed_content_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.allowed_content_types = types.into_iter().map(Into::into).collect();
        self
    }

    /// The server-side MIME allowlist.
    pub fn allowed_content_types(&self) -> &[String] {
        &self.allowed_content_types
    }

    /// Check whether a content type is on the allowlist (case-insensitive,
    /// parameters ignored).
    pub fn is_content_type_allowed(&self, content_type: &str) -> bool {
        let essence = content_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        self.allowed_content_types
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(&essence))
    }

    /// Validate an upload request against the configured size cap and MIME
    /// allowlist. This is the server-side enforcement point; client-side
    /// checks are hints only.
    pub fn validate_upload_request(&self, request: &UploadRequest) -> BlobResult<()> {
        if !self.is_size_allowed(request.content_length) {
            return Err(BlobError::TooLarge(
                request.content_length,
                self.max_file_size_bytes,
            ));
        }

        if !self.is_content_type_allowed(&request.content_type) {
            return Err(BlobError::InvalidContentType(request.content_type.clone()));
        }

        Ok(())
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
