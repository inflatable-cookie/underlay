//! Key loading and generation for Ed25519/EdDSA JWT signing.

pub use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::SigningKey;

use crate::error::{JwtError, JwtResult};

/// An Ed25519 key pair for JWT signing.
///
/// - Private key: base64-encoded PKCS#8 DER
/// - Public key: base64-encoded raw Ed25519 public key bytes (32 bytes)
#[derive(Clone)]
pub struct KeyPair {
    pub private_key_pkcs8_der_b64: String,
    pub public_key_raw_b64: String,
}

impl std::fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPair")
            .field("private_key_pkcs8_der_b64", &"[REDACTED]")
            .field("public_key_raw_b64", &self.public_key_raw_b64)
            .finish()
    }
}

impl KeyPair {
    /// Generate a new Ed25519 key pair.
    pub fn generate() -> JwtResult<Self> {
        let signing_key = SigningKey::generate(&mut rand::rng());
        let verifying_key = signing_key.verifying_key();

        let private_der = signing_key
            .to_pkcs8_der()
            .map_err(|e| JwtError::Key(e.to_string()))?;

        Ok(Self {
            private_key_pkcs8_der_b64: STANDARD.encode(private_der.as_bytes()),
            public_key_raw_b64: URL_SAFE_NO_PAD.encode(verifying_key.as_bytes()),
        })
    }

    pub fn decode_private_key_der(&self) -> JwtResult<Vec<u8>> {
        STANDARD
            .decode(&self.private_key_pkcs8_der_b64)
            .map_err(|_| JwtError::Key("Invalid base64 private key".to_string()))
    }

    pub fn public_key_raw_b64(&self) -> &str {
        &self.public_key_raw_b64
    }
}
