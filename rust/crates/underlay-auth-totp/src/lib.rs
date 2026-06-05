//! Time-based One-Time Password (TOTP) primitives.
//!
//! This crate provides app-agnostic helpers for:
//! - generating secrets
//! - generating otpauth provisioning URIs + QR SVG
//! - verifying TOTP codes with a time window
//! - generating and verifying backup codes (hashed)
//!
//! Storage, encryption, and credential association are owned by the app.

mod algorithm;
mod backup_codes;
mod config;
mod error;
mod provisioning;
mod service;
mod totp;
mod types;

pub use algorithm::TotpAlgorithm;
pub use config::TotpConfig;
pub use error::TotpError;
pub use service::TotpService;
pub use types::{TotpSecret, TotpSetup, TwoFactorCode, TwoFactorVerified, VerifiedTotp};

#[cfg(test)]
pub(crate) use totp::totp_code;

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
