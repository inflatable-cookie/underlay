//! Password authentication for Underlay.
//!
//! This module provides secure password hashing using Argon2id and
//! password strength validation.

mod errors;
mod hasher;
mod service;
mod strength;

pub use crate::errors::{PasswordAuthError, PasswordAuthResult};
pub use crate::hasher::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};
pub use crate::service::{
    CompromisedPasswordStrategy, FailedLoginAttempt, PasswordAuthRepository, PasswordAuthService,
    PasswordConfig,
};
pub use crate::strength::{
    PasswordAnalysis, PasswordRequirements, PasswordStrength, PasswordStrengthAnalyzer,
};
