//! Immutable verified staging-to-published blob promotion.
//!
//! A media version may become ready/current only from bytes captured once,
//! within a configured bound, by the server — validated for size, declared
//! MIME, and magic bytes, hashed by the server, and published under a
//! distinct destination key using exclusive create. The client upload key is
//! staging identity only and never becomes the published object identity.
//!
//! This module composes [`BlobAdapter::get_bytes_bounded`] and
//! [`BlobAdapter::put_bytes_create_only`] (or the owned exclusive-create
//! variant). It never calls the unbounded [`BlobAdapter::get_bytes`] and
//! never falls back to an unconditional [`BlobAdapter::put_bytes`].
//! Adapters that have not implemented the primitives refuse via their
//! fail-closed defaults, so promotion refuses too rather than silently
//! degrading to a mutable read/write.

use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::adapter::BlobAdapter;
use crate::config::BlobUploadConfig;
use crate::error::{BlobError, BlobResult};
use crate::owned::{
    unproven_destination, OwnedDestinationAuthority, OwnedPublicationFacts, OwnershipToken,
};
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
        let (bytes, sha256) = capture_validated_bytes(
            self,
            staging_key,
            destination_key,
            declared_content_type,
            config,
        )
        .await?;

        let stored = self
            .put_bytes_create_only(destination_key.as_str(), &bytes, declared_content_type)
            .await?;

        bind_published_result(
            stored,
            destination_key,
            bytes.len() as u64,
            declared_content_type,
            sha256,
        )
    }

    /// Like [`Self::promote_verified`], but exclusively creates the
    /// destination with reserved ownership metadata derived from `token`.
    ///
    /// The raw token is never written, logged, or returned. The stored
    /// verifier binds this adapter's provider, bucket, and `destination_key`
    /// together with the token. A destination collision still returns
    /// [`BlobError::DestinationExists`]; recovery of an owned incumbent is a
    /// separate [`Self::recover_owned_publication`] call. Adapters that have
    /// not implemented owned exclusive create refuse with
    /// [`BlobError::Unsupported`].
    async fn promote_verified_owned(
        &self,
        staging_key: &BlobObjectKey,
        destination_key: &BlobObjectKey,
        declared_content_type: &str,
        config: &BlobUploadConfig,
        token: &OwnershipToken,
    ) -> BlobResult<VerifiedPromotionResult> {
        let (bytes, sha256) = capture_validated_bytes(
            self,
            staging_key,
            destination_key,
            declared_content_type,
            config,
        )
        .await?;
        let authority =
            OwnedDestinationAuthority::new(self.name(), self.bucket(), destination_key.clone())?;
        let facts = OwnedPublicationFacts::from_token_and_bytes(
            token,
            &authority,
            &bytes,
            declared_content_type,
        );
        debug_assert_eq!(facts.sha256(), sha256);

        let stored = self
            .put_bytes_create_only_owned(
                destination_key.as_str(),
                &bytes,
                declared_content_type,
                &facts,
            )
            .await?;

        bind_published_result(
            stored,
            destination_key,
            bytes.len() as u64,
            declared_content_type,
            sha256,
        )
    }

    /// Recover an owned destination from `head` using the durable token and
    /// destination authority persisted before create.
    ///
    /// Never reads staging. Absent, malformed, incomplete, mismatched, or
    /// otherwise unproven ownership is [`BlobError::DestinationExists`] so
    /// the incumbent is preserved and the token is not disclosed. A missing
    /// destination is [`BlobError::NotFound`]. Wrong provider/bucket
    /// authority is [`BlobError::InvalidKey`]. Unsupported adapters refuse
    /// through `head` / owned-create defaults as [`BlobError::Unsupported`]
    /// or unproven collision once an object exists without reserved facts.
    async fn recover_owned_publication(
        &self,
        token: &OwnershipToken,
        authority: &OwnedDestinationAuthority,
    ) -> BlobResult<VerifiedPromotionResult> {
        if self.name() != authority.provider() || self.bucket() != authority.bucket() {
            return Err(BlobError::InvalidKey(
                "owned recovery destination authority does not match this adapter".to_string(),
            ));
        }

        let info = self.head(authority.key().as_str()).await?;
        let Some(facts) = OwnedPublicationFacts::from_object_metadata(&info.metadata) else {
            return Err(unproven_destination(authority.key().as_str()));
        };
        if !facts.matches_token(token, authority) || facts.size() != info.size {
            return Err(unproven_destination(authority.key().as_str()));
        }

        Ok(VerifiedPromotionResult {
            object: StoredObject {
                provider: self.name().to_string(),
                bucket: self.bucket().to_string(),
                key: authority.key().as_str().to_string(),
                size: info.size,
                content_type: facts.mime().to_string(),
                etag: info.etag,
            },
            sha256: facts.sha256().to_string(),
        })
    }
}

async fn capture_validated_bytes<A: BlobAdapter + ?Sized>(
    adapter: &A,
    staging_key: &BlobObjectKey,
    destination_key: &BlobObjectKey,
    declared_content_type: &str,
    config: &BlobUploadConfig,
) -> BlobResult<(Vec<u8>, String)> {
    if staging_key == destination_key {
        return Err(BlobError::InvalidKey(
            "staging and destination keys must be distinct".to_string(),
        ));
    }

    let max_bytes = config.max_file_size_bytes_limit();
    let bytes = adapter
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
    Ok((bytes, sha256))
}

fn bind_published_result(
    stored: StoredObject,
    destination_key: &BlobObjectKey,
    captured_size: u64,
    declared_content_type: &str,
    sha256: String,
) -> BlobResult<VerifiedPromotionResult> {
    // The adapter is trusted to have exclusively written exactly the
    // captured bytes to `destination_key`, but not trusted to *report*
    // that accurately: bind the public result to what was actually
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
    if stored.size != captured_size {
        return Err(BlobError::Internal(format!(
            "adapter reported size {} for destination {:?}, but {} bytes were captured and published",
            stored.size,
            destination_key.as_str(),
            captured_size
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

impl<T> BlobAdapterPromotionExt for T where T: BlobAdapter + ?Sized {}

#[cfg(test)]
#[path = "tests/promotion_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/owned_support.rs"]
mod owned_support;

#[cfg(test)]
#[path = "tests/owned_promotion_tests.rs"]
mod owned_tests;

#[cfg(test)]
#[path = "tests/owned_recovery_refusal_tests.rs"]
mod owned_refusal_tests;
