//! Token fingerprinting helpers.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha2::{Digest, Sha256};

/// Compute a stable fingerprint for looking up tokens without storing them.
///
/// This is intended for DB lookup and revocation checks.
pub fn token_fingerprint(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let digest = hasher.finalize();
    URL_SAFE_NO_PAD.encode(digest)
}
