use underlay_auth::AuthResult;
use webauthn_rs::prelude::*;

use super::WebAuthnService;
use crate::WebAuthnError;

impl WebAuthnService {
    /// Serialize a `Passkey` to JSON for storage.
    pub fn encode_passkey(&self, passkey: &Passkey) -> AuthResult<String> {
        serde_json::to_string(passkey).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    /// Deserialize a stored `Passkey` JSON string.
    pub fn decode_passkey(&self, encoded: &str) -> AuthResult<Passkey> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    /// Serialisation helpers for registration/authentication state.
    ///
    /// These are gated behind a feature because `webauthn-rs` treats state serialisation as a
    /// footgun unless you understand the replay/security implications.
    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn encode_registration_state(state: &PasskeyRegistration) -> AuthResult<String> {
        serde_json::to_string(state).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn decode_registration_state(encoded: &str) -> AuthResult<PasskeyRegistration> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn encode_authentication_state(state: &PasskeyAuthentication) -> AuthResult<String> {
        serde_json::to_string(state).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn decode_authentication_state(encoded: &str) -> AuthResult<PasskeyAuthentication> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn encode_discoverable_authentication_state(
        state: &DiscoverableAuthentication,
    ) -> AuthResult<String> {
        serde_json::to_string(state).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "danger-allow-state-serialisation")]
    pub fn decode_discoverable_authentication_state(
        encoded: &str,
    ) -> AuthResult<DiscoverableAuthentication> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }
}
