//! WebAuthn / Passkey primitives for Underlay-based apps.
//!
//! This crate wraps `webauthn-rs` into an app-agnostic boundary that:
//! - generates registration/authentication challenges
//! - verifies registration/authentication responses
//! - provides helpers for storing passkeys
//!
//! Apps remain responsible for:
//! - persisting state between start/finish steps (server-side)
//! - persisting passkeys (typically in the credential store)
//! - routing, cookies, sessions, and UX

#[cfg(feature = "attestation")]
mod attested;
mod error;
mod service;
mod types;

use base64urlsafedata::HumanBinaryData;

#[cfg(test)]
use underlay_auth::{AuthError, CredentialMetadata};
#[cfg(test)]
use underlay_core::Uuid;
#[cfg(test)]
use webauthn_rs::prelude::RegisterPublicKeyCredential;

#[cfg(feature = "attestation")]
pub use attested::*;
pub use error::WebAuthnError;
pub use service::WebAuthnService;
pub use types::*;

/// Underlay boundary type for WebAuthn credential IDs.
///
/// This intentionally avoids exposing `webauthn-rs-core` internals.
pub type CredentialId = HumanBinaryData;

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
