//! JWT-based token issuance and verification for Underlay.
//!
//! This crate intentionally uses `jsonwebtoken` for JWT encoding/decoding.
//! It provides Underlay-specific claims, key loading, and token fingerprinting.
//! For full session lifecycle management, use `underlay-auth-session`.

mod claims;
mod config;
mod error;
mod fingerprint;
mod keys;
mod service;

pub use crate::claims::{AccessTokenClaims, RefreshTokenClaims, TokenUse};
pub use crate::config::{JwtBehaviorDefaults, JwtConfig};
pub use crate::error::{JwtError, JwtResult};
pub use crate::fingerprint::token_fingerprint;
pub use crate::keys::KeyPair;
pub use crate::service::JwtService;
