//! Password authentication service.

mod config;
mod core;
mod login;
mod passwords;
mod policy;
mod repository;

pub use config::{CompromisedPasswordStrategy, PasswordConfig};
pub use core::PasswordAuthService;
pub use repository::{FailedLoginAttempt, PasswordAuthRepository};

#[cfg(test)]
pub(crate) use crate::errors::{PasswordAuthError, PasswordAuthResult};

#[cfg(test)]
#[path = "tests/service_tests.rs"]
mod tests;
