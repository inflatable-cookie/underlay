//! Token-bound ownership proof for immutable verified publication.
//!
//! A consumer persists an opaque token and immutable destination authority
//! before exclusive create. Underlay stores only a one-way verifier plus
//! server-derived SHA-256, size, and validated MIME as reserved metadata in
//! the same backend commit as the bytes. Restart recovery accepts a matching
//! verifier and complete facts from `head`; it never rereads staging and
//! never treats byte equality as ownership.
//!
//! # v0.9.7 reserved metadata wire format
//!
//! Exclusive owned create writes these object-metadata keys (S3 user
//! metadata / local `user.underlay.owned.v1.*` xattrs mapped to the same
//! names):
//!
//! - `underlay-owned-v1-verifier`: 64-char lowercase hex SHA-256
//! - `underlay-owned-v1-sha256`: 64-char lowercase hex of the published bytes
//! - `underlay-owned-v1-size`: decimal byte length
//! - `underlay-owned-v1-mime`: validated declared MIME
//!
//! The verifier is SHA-256 over the domain-separated, length-prefixed
//! encoding:
//!
//! ```text
//! b"underlay.blob.owned-publication.v1\0"
//! || u32be(len(provider)) || provider_utf8
//! || u32be(len(bucket))   || bucket_utf8
//! || u32be(len(key))      || key_utf8
//! || u32be(len(token))    || token_bytes
//! ```
//!
//! Lengths are big-endian `u32` byte counts of the following field. This
//! encoding is unambiguous under concatenation: `("ab","c")` and `("a","bc")`
//! cannot produce the same verifier. The raw token is never stored. Copying
//! an object plus its reserved metadata to another provider, bucket, or key
//! cannot recover under that new authority.

use std::collections::HashMap;
use std::fmt;

use sha2::{Digest, Sha256};

use crate::error::{BlobError, BlobResult};
use crate::types::BlobObjectKey;

/// Domain-separated one-way verifier prefix. The raw token is never stored.
const VERIFIER_DOMAIN: &[u8] = b"underlay.blob.owned-publication.v1\0";

/// Reserved object-metadata key for the token verifier (lowercase hex SHA-256).
pub const OWNED_META_VERIFIER: &str = "underlay-owned-v1-verifier";
/// Reserved object-metadata key for the server-derived content SHA-256.
pub const OWNED_META_SHA256: &str = "underlay-owned-v1-sha256";
/// Reserved object-metadata key for the published byte size (decimal).
pub const OWNED_META_SIZE: &str = "underlay-owned-v1-size";
/// Reserved object-metadata key for the validated declared MIME.
pub const OWNED_META_MIME: &str = "underlay-owned-v1-mime";

/// Opaque high-entropy ownership token.
///
/// Generate a distinct token for each publication. The stored verifier also
/// binds provider, bucket/namespace, and destination key, so reusing a token
/// cannot authorize a different destination. Per-publication uniqueness is
/// an operational requirement (lost-token blast radius), not the ownership
/// proof itself — uniqueness alone is not sufficient.
///
/// The raw bytes never appear in `Debug`, public errors, logs, URLs, object
/// metadata, or returned DTOs. Compare only through the one-way verifier
/// attached at exclusive create.
pub struct OwnershipToken {
    bytes: Vec<u8>,
}

impl OwnershipToken {
    /// Minimum accepted token length. Shorter values are not high-entropy
    /// enough for this proof. Each publication should use a freshly generated
    /// token at least this long.
    pub const MIN_LEN: usize = 32;

    /// Construct from caller-held secret bytes.
    ///
    /// Fails with [`BlobError::ConfigError`] when `bytes` is shorter than
    /// [`Self::MIN_LEN`]. The error does not echo the input.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> BlobResult<Self> {
        let bytes = bytes.into();
        if bytes.len() < Self::MIN_LEN {
            return Err(BlobError::ConfigError(
                "ownership token must be at least 32 bytes".to_string(),
            ));
        }
        Ok(Self { bytes })
    }
}

impl fmt::Debug for OwnershipToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OwnershipToken([redacted])")
    }
}

/// Immutable destination the consumer persisted before publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedDestinationAuthority {
    provider: String,
    bucket: String,
    key: BlobObjectKey,
}

impl OwnedDestinationAuthority {
    /// Bind recovery to this adapter identity and destination key.
    ///
    /// `provider` must equal [`crate::BlobAdapter::name`] and `bucket` must
    /// equal [`crate::BlobAdapter::bucket`] of the adapter used to recover.
    pub fn new(
        provider: impl Into<String>,
        bucket: impl Into<String>,
        key: BlobObjectKey,
    ) -> BlobResult<Self> {
        let provider = provider.into();
        let bucket = bucket.into();
        if provider.is_empty() || bucket.is_empty() {
            return Err(BlobError::InvalidKey(
                "owned recovery destination authority requires provider and bucket".to_string(),
            ));
        }
        Ok(Self {
            provider,
            bucket,
            key,
        })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &BlobObjectKey {
        &self.key
    }
}

/// Reserved publication facts written atomically with exclusive create.
///
/// Construct only from a caller token, the destination authority, the
/// captured bytes, and the validated MIME. The raw token is not retained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedPublicationFacts {
    verifier_hex: String,
    sha256: String,
    size: u64,
    mime: String,
}

impl OwnedPublicationFacts {
    /// Derive reserved facts from the token, destination authority, and the
    /// exact bytes that will be published.
    ///
    /// The verifier binds `authority` (provider, bucket/namespace, key) and
    /// the token. Copied metadata cannot recover under a different
    /// destination.
    pub fn from_token_and_bytes(
        token: &OwnershipToken,
        authority: &OwnedDestinationAuthority,
        data: &[u8],
        content_type: &str,
    ) -> Self {
        Self {
            verifier_hex: hex::encode(owned_verifier(token, authority)),
            sha256: hex::encode(Sha256::digest(data)),
            size: data.len() as u64,
            mime: content_type.to_string(),
        }
    }

    /// Parse reserved keys from object head metadata.
    ///
    /// Missing, empty, or malformed fields yield `None` so callers can
    /// refuse as an unproven collision without distinguishing why.
    pub fn from_object_metadata(metadata: &HashMap<String, String>) -> Option<Self> {
        let verifier_hex = metadata.get(OWNED_META_VERIFIER)?.clone();
        let sha256 = metadata.get(OWNED_META_SHA256)?.clone();
        let size_raw = metadata.get(OWNED_META_SIZE)?;
        let mime = metadata.get(OWNED_META_MIME)?.clone();

        if !is_lowercase_hex_sha256(&verifier_hex) || !is_lowercase_hex_sha256(&sha256) {
            return None;
        }
        let size: u64 = size_raw.parse().ok()?;
        if !is_reserved_mime(&mime) {
            return None;
        }

        Some(Self {
            verifier_hex,
            sha256,
            size,
            mime,
        })
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn mime(&self) -> &str {
        &self.mime
    }

    /// Compare the stored verifier to `token` bound to `authority` without a
    /// data-dependent early exit on the digest bytes.
    pub fn matches_token(
        &self,
        token: &OwnershipToken,
        authority: &OwnedDestinationAuthority,
    ) -> bool {
        let computed = owned_verifier(token, authority);
        match decode_sha256_bytes(&self.verifier_hex) {
            Some(stored) => constant_time_eq(&computed, &stored),
            None => false,
        }
    }

    /// Reserved S3 / `ObjectInfo` metadata pairs. Values are ASCII and never
    /// include the raw token.
    pub fn metadata_pairs(&self) -> [(&'static str, String); 4] {
        [
            (OWNED_META_VERIFIER, self.verifier_hex.clone()),
            (OWNED_META_SHA256, self.sha256.clone()),
            (OWNED_META_SIZE, self.size.to_string()),
            (OWNED_META_MIME, self.mime.clone()),
        ]
    }
}

pub(crate) fn unproven_destination(key: &str) -> BlobError {
    BlobError::DestinationExists(key.to_string())
}

fn owned_verifier(token: &OwnershipToken, authority: &OwnedDestinationAuthority) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(VERIFIER_DOMAIN);
    append_len_prefixed(&mut hasher, authority.provider().as_bytes());
    append_len_prefixed(&mut hasher, authority.bucket().as_bytes());
    append_len_prefixed(&mut hasher, authority.key().as_str().as_bytes());
    append_len_prefixed(&mut hasher, &token.bytes);
    hasher.finalize().into()
}

fn append_len_prefixed(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u32).to_be_bytes());
    hasher.update(bytes);
}

fn is_lowercase_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_reserved_mime(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|b| b.is_ascii() && !b.is_ascii_control())
        && value.contains('/')
}

fn decode_sha256_bytes(hex_str: &str) -> Option<[u8; 32]> {
    if !is_lowercase_hex_sha256(hex_str) {
        return None;
    }
    let mut out = [0u8; 32];
    hex::decode_to_slice(hex_str.as_bytes(), &mut out).ok()?;
    Some(out)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[cfg(test)]
#[path = "tests/owned_tests.rs"]
mod tests;
