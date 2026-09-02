//! Immutable verified staging-to-published blob promotion.
//!
//! A media version may become ready/current only from bytes captured once,
//! within a configured bound, by the server — validated for size, declared
//! MIME, and magic bytes, hashed by the server, and published under a
//! distinct destination key using exclusive create. The client upload key is
//! staging identity only and never becomes the published object identity.
//!
//! This module composes [`BlobAdapter::get_bytes_bounded`] and
//! [`BlobAdapter::put_bytes_create_only`]; it never calls the unbounded
//! [`BlobAdapter::get_bytes`] and never falls back to an unconditional
//! [`BlobAdapter::put_bytes`]. Adapters that have not implemented the two
//! bounded/exclusive primitives refuse via their fail-closed defaults, so
//! `promote_verified` refuses too rather than silently degrading to a
//! mutable read/write.

use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::adapter::BlobAdapter;
use crate::config::BlobUploadConfig;
use crate::error::{BlobError, BlobResult};
use crate::types::{BlobObjectKey, StoredObject};

/// Result of a verified staging-to-published blob promotion.
///
/// `object` describes the distinct destination the captured bytes were
/// published to. Its `content_type` is the server-validated declared MIME,
/// not whatever the storage backend happens to echo back. `object.etag` is
/// supplemental backend metadata only, never the cross-adapter identity for
/// the published bytes; use `sha256` for that.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerifiedPromotionResult {
    /// The published destination object: provider, bucket, key, size, and
    /// validated content type.
    pub object: StoredObject,

    /// Lowercase 64-character hex SHA-256 of the exact bytes captured from
    /// staging and published to `object`.
    pub sha256: String,
}

/// Verified staging-to-published promotion over any [`BlobAdapter`].
///
/// Additive over the raw trait: an adapter only needs to implement
/// [`BlobAdapter::get_bytes_bounded`] and [`BlobAdapter::put_bytes_create_only`]
/// (or accept their fail-closed defaults) to participate.
#[async_trait]
pub trait BlobAdapterPromotionExt: BlobAdapter {
    /// Capture `staging_key` once under `config`'s size bound, validate the
    /// captured bytes against `declared_content_type` (allowlist and magic
    /// bytes), derive their SHA-256, and publish that exact vector to
    /// `destination_key` through exclusive create.
    ///
    /// `staging_key` is preserved; the caller owns its cleanup/recovery
    /// policy. No client-supplied digest ever enters this path — the
    /// returned SHA-256 is always server-derived from the captured bytes.
    ///
    /// Fails with:
    /// - [`BlobError::InvalidKey`] if `staging_key` and `destination_key` are
    ///   equal;
    /// - [`BlobError::TooLarge`] if staging exceeds the configured maximum;
    /// - [`BlobError::InvalidContentType`] if `declared_content_type` is not
    ///   allowed, or the captured bytes' magic-byte signature does not match
    ///   it;
    /// - [`BlobError::DestinationExists`] if `destination_key` already holds
    ///   an object (a collision, never overwritten);
    /// - [`BlobError::Unsupported`] if the adapter has not implemented real
    ///   bounded capture or exclusive create;
    /// - [`BlobError::Internal`] if the adapter's `put_bytes_create_only`
    ///   reports a different key or size than what was actually written —
    ///   an adapter-contract violation, not a caller error. This keeps the
    ///   returned `VerifiedPromotionResult` bound to the captured vector
    ///   even against a non-conforming custom adapter.
    async fn promote_verified(
        &self,
        staging_key: &BlobObjectKey,
        destination_key: &BlobObjectKey,
        declared_content_type: &str,
        config: &BlobUploadConfig,
    ) -> BlobResult<VerifiedPromotionResult> {
        if staging_key == destination_key {
            return Err(BlobError::InvalidKey(
                "staging and destination keys must be distinct".to_string(),
            ));
        }

        let max_bytes = config.max_file_size_bytes_limit();
        let bytes = self
            .get_bytes_bounded(staging_key.as_str(), max_bytes)
            .await?;
        debug_assert!(
            bytes.len() as u64 <= max_bytes,
            "get_bytes_bounded must not return more than max_bytes"
        );

        if !config.is_content_type_allowed(declared_content_type) {
            return Err(BlobError::InvalidContentType(
                declared_content_type.to_string(),
            ));
        }
        if !crate::sniff::content_matches_declared(&bytes, declared_content_type) {
            return Err(BlobError::InvalidContentType(format!(
                "captured bytes do not match declared content type {}",
                declared_content_type
            )));
        }

        let sha256 = hex::encode(Sha256::digest(&bytes));

        let stored = self
            .put_bytes_create_only(destination_key.as_str(), &bytes, declared_content_type)
            .await?;

        // The adapter is trusted to have exclusively written exactly
        // `bytes` to `destination_key`, but not trusted to *report* that
        // accurately: bind the public result to what was actually
        // requested/captured rather than whatever identity the adapter
        // echoes back, so a non-conforming adapter cannot desynchronize
        // `VerifiedPromotionResult` from the captured vector.
        if stored.key != destination_key.as_str() {
            return Err(BlobError::Internal(format!(
                "adapter returned destination key {:?}, expected {:?}",
                stored.key,
                destination_key.as_str()
            )));
        }
        if stored.size != bytes.len() as u64 {
            return Err(BlobError::Internal(format!(
                "adapter reported size {} for destination {:?}, but {} bytes were captured and published",
                stored.size,
                destination_key.as_str(),
                bytes.len()
            )));
        }

        Ok(VerifiedPromotionResult {
            object: StoredObject {
                content_type: declared_content_type.to_string(),
                ..stored
            },
            sha256,
        })
    }
}

impl<T> BlobAdapterPromotionExt for T where T: BlobAdapter + ?Sized {}

#[cfg(test)]
#[path = "tests/promotion_tests.rs"]
mod tests;
