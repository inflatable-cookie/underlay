//! JWT-based session management for Underlay.
//!
//! This crate intentionally uses `jsonwebtoken` for JWT encoding/decoding.
//! It provides Underlay-specific claims, key loading, and a session manager
//! abstraction on top.

mod claims;
mod config;
mod error;
mod fingerprint;
mod keys;
mod service;

pub use crate::claims::{AccessTokenClaims, RefreshTokenClaims, TokenUse};
pub use crate::config::JwtConfig;
pub use crate::error::{JwtError, JwtResult};
pub use crate::fingerprint::token_fingerprint;
pub use crate::keys::KeyPair;
pub use crate::service::{JwtService, SessionManager, SessionState, SessionStore, SessionTokens};
