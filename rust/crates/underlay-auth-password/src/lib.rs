//! Password authentication for Underlay.
//!
//! This module provides secure password hashing using Argon2id and
//! password strength validation.

mod errors;
#[cfg(feature = "hibp")]
pub(crate) mod hibp;
mod service;
mod strength;

pub use crate::errors::{PasswordAuthError, PasswordAuthResult};
pub use crate::service::{
    CompromisedPasswordStrategy, FailedLoginAttempt, PasswordAuthRepository, PasswordAuthService,
    PasswordConfig,
};
pub use crate::strength::{
    PasswordAnalysis, PasswordRequirements, PasswordStrength, PasswordStrengthAnalyzer,
};
pub use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};
