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

use base64urlsafedata::HumanBinaryData;
use serde::{Deserialize, Serialize};
use underlay_auth::{AuthError, AuthResult, CredentialMetadata};
use underlay_core::Uuid;
use url::Url;
use webauthn_rs::prelude::*;

/// Underlay boundary type for WebAuthn credential IDs.
///
/// This intentionally avoids exposing `webauthn-rs-core` internals.
pub type CredentialId = HumanBinaryData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPasskey {
    /// Base64url-encoded credential ID (safe for indexing).
    pub credential_id: String,

    /// JSON-encoded `webauthn_rs::prelude::Passkey`.
    pub passkey_json: String,

    /// The last known signature counter, if available.
    pub counter: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeySyncInfo {
    pub transports: Vec<String>,
    pub backup_eligible: bool,
    pub backup_state: bool,
    pub user_verified: bool,
}

#[derive(Debug, Clone)]
pub struct StoredPasskeyUpdate {
    pub stored_passkey: StoredPasskey,
    pub changed: bool,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAttestedPasskey {
    /// Base64url-encoded credential ID (safe for indexing).
    pub credential_id: String,

    /// JSON-encoded `webauthn_rs::prelude::AttestedPasskey`.
    pub attested_passkey_json: String,

    /// The last known signature counter, if available.
    pub counter: Option<u32>,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyRegistrationRequest {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
    pub exclude_credential_ids: Option<Vec<CredentialId>>,
    pub ui_hint_authenticator_attachment: Option<AuthenticatorAttachment>,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyRegistrationResponse {
    pub options: CreationChallengeResponse,
    pub state_id: String,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyRegistrationRequest {
    pub state_id: String,
    pub credential: RegisterPublicKeyCredential,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyRegistrationResponse {
    pub stored_attested_passkey: StoredAttestedPasskey,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyAuthenticationRequest {
    /// JSON-encoded `AttestedPasskey` values that are allowed for this authentication.
    pub allowed_credentials: Vec<String>,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyAuthenticationResponse {
    pub options: RequestChallengeResponse,
    pub state_id: String,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyAuthenticationRequest {
    pub state_id: String,
    pub credential: PublicKeyCredential,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyAuthenticationResponse {
    pub result: AuthenticationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyRegistrationRequest {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
    pub exclude_credential_ids: Option<Vec<CredentialId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyRegistrationResponse {
    pub options: CreationChallengeResponse,

    /// Opaque identifier for server-side persistence between start/finish.
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyRegistrationRequest {
    pub state_id: String,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyRegistrationResponse {
    pub stored_passkey: StoredPasskey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyAuthenticationRequest {
    /// JSON-encoded `Passkey` values that are allowed for this authentication.
    ///
    /// For discoverable credentials, pass an empty list.
    pub allowed_credentials: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyAuthenticationResponse {
    pub options: RequestChallengeResponse,

    /// Opaque identifier for server-side persistence between start/finish.
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyAuthenticationRequest {
    pub state_id: String,
    pub credential: PublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyAuthenticationResponse {
    pub result: AuthenticationResult,
}

#[derive(Debug, Clone)]
pub struct WebAuthnConfig {
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum WebAuthnError {
    #[error("invalid webauthn configuration")]
    InvalidConfig,
    #[error("passkey registration failed")]
    RegistrationFailed,
    #[error("passkey authentication failed")]
    AuthenticationFailed,
    #[error("passkey counter regression")]
    CounterRegression,
    #[error("invalid passkey encoding")]
    InvalidPasskeyEncoding,
}

impl From<WebAuthnError> for AuthError {
    fn from(value: WebAuthnError) -> Self {
        match value {
            WebAuthnError::InvalidConfig => {
                AuthError::Internal("invalid webauthn configuration".into())
            }
            WebAuthnError::RegistrationFailed => AuthError::PassKeyRegistrationFailed,
            WebAuthnError::AuthenticationFailed => AuthError::PassKeyAuthenticationFailed,
            WebAuthnError::CounterRegression => AuthError::PassKeyCounterRegression,
            WebAuthnError::InvalidPasskeyEncoding => {
                AuthError::BadRequest("invalid passkey encoding".into())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebAuthnService {
    inner: Webauthn,
}

impl WebAuthnService {
    pub fn new(config: WebAuthnConfig) -> AuthResult<Self> {
        let origin = Url::parse(&config.rp_origin).map_err(|_| WebAuthnError::InvalidConfig)?;

        let builder = WebauthnBuilder::new(&config.rp_id, &origin)
            .map_err(|_| WebAuthnError::InvalidConfig)?
            .rp_name(&config.rp_name);

        let inner = builder.build().map_err(|_| WebAuthnError::InvalidConfig)?;

        Ok(Self { inner })
    }

    pub fn start_passkey_registration(
        &self,
        user_id: Uuid,
        user_name: &str,
        display_name: &str,
        exclude_credential_ids: Option<Vec<CredentialId>>,
    ) -> AuthResult<(CreationChallengeResponse, PasskeyRegistration)> {
        self.inner
            .start_passkey_registration(
                user_id.into_inner(),
                user_name,
                display_name,
                exclude_credential_ids,
            )
            .map_err(|_| WebAuthnError::RegistrationFailed.into())
    }

    pub fn credential_id_from_passkey(passkey: &Passkey) -> CredentialId {
        passkey.cred_id().clone()
    }

    pub fn finish_passkey_registration(
        &self,
        state: &PasskeyRegistration,
        response: &RegisterPublicKeyCredential,
    ) -> AuthResult<Passkey> {
        self.inner
            .finish_passkey_registration(response, state)
            .map_err(|_| WebAuthnError::RegistrationFailed.into())
    }

    pub fn start_passkey_authentication(
        &self,
        allowed_credentials: Vec<Passkey>,
    ) -> AuthResult<(RequestChallengeResponse, PasskeyAuthentication)> {
        self.inner
            .start_passkey_authentication(allowed_credentials.as_slice())
            .map_err(|_| WebAuthnError::AuthenticationFailed.into())
    }

    pub fn finish_passkey_authentication(
        &self,
        response: &PublicKeyCredential,
        state: &PasskeyAuthentication,
    ) -> AuthResult<AuthenticationResult> {
        self.inner
            .finish_passkey_authentication(response, state)
            .map_err(|e| match e {
                WebauthnError::CredentialCounterUpdateFailure => {
                    WebAuthnError::CounterRegression.into()
                }
                _ => WebAuthnError::AuthenticationFailed.into(),
            })
    }

    /// Serialize a `Passkey` to JSON for storage.
    pub fn encode_passkey(&self, passkey: &Passkey) -> AuthResult<String> {
        serde_json::to_string(passkey).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    /// Deserialize a stored `Passkey` JSON string.
    pub fn decode_passkey(&self, encoded: &str) -> AuthResult<Passkey> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    pub fn credential_id_to_base64url(credential_id: &CredentialId) -> AuthResult<String> {
        let value = serde_json::to_value(credential_id)
            .map_err(|_| WebAuthnError::InvalidPasskeyEncoding)?;

        match value {
            serde_json::Value::String(s) => Ok(s),
            _ => Err(WebAuthnError::InvalidPasskeyEncoding.into()),
        }
    }

    /// Attempts to read the signature counter from an encoded `Passkey` JSON string.
    ///
    /// This is intended for indexing/diagnostics only; rely on `AuthenticationResult` and
    /// `Passkey::update_credential` for counter updates.
    pub fn passkey_counter_from_json(passkey_json: &str) -> Option<u32> {
        let value: serde_json::Value = serde_json::from_str(passkey_json).ok()?;
        let counter = value.get("cred")?.get("counter")?.as_u64()?;
        u32::try_from(counter).ok()
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

    pub fn stored_passkey_from_passkey(&self, passkey: &Passkey) -> AuthResult<StoredPasskey> {
        let passkey_json = self.encode_passkey(passkey)?;
        let credential_id =
            Self::credential_id_to_base64url(&Self::credential_id_from_passkey(passkey))?;
        let counter = Self::passkey_counter_from_json(&passkey_json);

        Ok(StoredPasskey {
            credential_id,
            passkey_json,
            counter,
        })
    }

    pub fn passkey_from_stored_passkey(&self, stored: &StoredPasskey) -> AuthResult<Passkey> {
        self.decode_passkey(&stored.passkey_json)
    }

    pub fn passkey_transports_from_json(passkey_json: &str) -> Vec<String> {
        let value: serde_json::Value = match serde_json::from_str(passkey_json) {
            Ok(v) => v,
            Err(_) => return vec![],
        };

        let Some(transports) = value.get("cred").and_then(|v| v.get("transports")) else {
            return vec![];
        };

        let Some(transports) = transports.as_array() else {
            return vec![];
        };

        transports
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    }

    pub fn passkey_sync_info_from_json(passkey_json: &str) -> PasskeySyncInfo {
        let value: serde_json::Value = match serde_json::from_str(passkey_json) {
            Ok(v) => v,
            Err(_) => {
                return PasskeySyncInfo {
                    transports: vec![],
                    backup_eligible: false,
                    backup_state: false,
                    user_verified: false,
                };
            }
        };

        let transports = Self::passkey_transports_from_json(passkey_json);

        let backup_eligible = value
            .get("cred")
            .and_then(|v| v.get("backup_eligible"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let backup_state = value
            .get("cred")
            .and_then(|v| v.get("backup_state"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let user_verified = value
            .get("cred")
            .and_then(|v| v.get("user_verified"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        PasskeySyncInfo {
            transports,
            backup_eligible,
            backup_state,
            user_verified,
        }
    }

    pub fn credential_metadata_from_stored_passkey(stored: &StoredPasskey) -> CredentialMetadata {
        let transports = Self::passkey_transports_from_json(&stored.passkey_json);
        let last_counter = stored
            .counter
            .or_else(|| Self::passkey_counter_from_json(&stored.passkey_json))
            .unwrap_or(0);

        CredentialMetadata::Passkey {
            credential_id: stored.credential_id.clone(),
            transports,
            last_counter,
        }
    }

    pub fn authentication_result_credential_id_base64url(
        result: &AuthenticationResult,
    ) -> AuthResult<String> {
        let value = serde_json::to_value(result.cred_id())
            .map_err(|_| WebAuthnError::InvalidPasskeyEncoding)?;

        match value {
            serde_json::Value::String(s) => Ok(s),
            _ => Err(WebAuthnError::InvalidPasskeyEncoding.into()),
        }
    }

    pub fn find_stored_passkey_by_credential_id<'a>(
        stored: &'a [StoredPasskey],
        credential_id: &str,
    ) -> Option<&'a StoredPasskey> {
        stored.iter().find(|pk| pk.credential_id == credential_id)
    }

    /// Applies an [`AuthenticationResult`] to a stored passkey, performing counter regression checks
    /// and returning an updated stored passkey when the credential properties changed.
    pub fn update_stored_passkey_after_authentication(
        &self,
        stored: &StoredPasskey,
        result: &AuthenticationResult,
    ) -> AuthResult<StoredPasskeyUpdate> {
        let result_credential_id = Self::authentication_result_credential_id_base64url(result)?;
        if result_credential_id != stored.credential_id {
            return Err(AuthError::PassKeyCredentialNotFound);
        }

        let stored_counter = stored
            .counter
            .or_else(|| Self::passkey_counter_from_json(&stored.passkey_json));

        let new_counter = result.counter();
        if new_counter > 0 {
            if let Some(old_counter) = stored_counter {
                if new_counter <= old_counter {
                    return Err(AuthError::PassKeyCounterRegression);
                }
            }
        }

        let mut passkey = self.passkey_from_stored_passkey(stored)?;
        let changed = passkey
            .update_credential(result)
            .ok_or(AuthError::PassKeyCredentialNotFound)?;

        if !changed {
            return Ok(StoredPasskeyUpdate {
                stored_passkey: stored.clone(),
                changed: false,
            });
        }

        let passkey_json = self.encode_passkey(&passkey)?;
        let counter = Self::passkey_counter_from_json(&passkey_json).or(stored.counter);

        Ok(StoredPasskeyUpdate {
            stored_passkey: StoredPasskey {
                credential_id: stored.credential_id.clone(),
                passkey_json,
                counter,
            },
            changed: true,
        })
    }

    pub fn start_passkey_registration_http<P>(
        &self,
        req: StartPasskeyRegistrationRequest,
        persist_state: P,
    ) -> AuthResult<StartPasskeyRegistrationResponse>
    where
        P: FnOnce(PasskeyRegistration) -> AuthResult<String>,
    {
        let (options, state) = self.start_passkey_registration(
            req.user_id,
            &req.user_name,
            &req.display_name,
            req.exclude_credential_ids,
        )?;

        Ok(StartPasskeyRegistrationResponse {
            options,
            state_id: persist_state(state)?,
        })
    }

    pub fn finish_passkey_registration_http<L>(
        &self,
        req: FinishPasskeyRegistrationRequest,
        load_state: L,
    ) -> AuthResult<FinishPasskeyRegistrationResponse>
    where
        L: FnOnce(&str) -> AuthResult<PasskeyRegistration>,
    {
        let state = load_state(&req.state_id)?;
        let passkey = self.finish_passkey_registration(&state, &req.credential)?;

        Ok(FinishPasskeyRegistrationResponse {
            stored_passkey: self.stored_passkey_from_passkey(&passkey)?,
        })
    }

    pub fn start_passkey_authentication_http<P>(
        &self,
        req: StartPasskeyAuthenticationRequest,
        persist_state: P,
    ) -> AuthResult<StartPasskeyAuthenticationResponse>
    where
        P: FnOnce(PasskeyAuthentication) -> AuthResult<String>,
    {
        let mut allowed_credentials = Vec::with_capacity(req.allowed_credentials.len());
        for encoded in req.allowed_credentials {
            allowed_credentials.push(self.decode_passkey(&encoded)?);
        }

        let (options, state) = self.start_passkey_authentication(allowed_credentials)?;

        Ok(StartPasskeyAuthenticationResponse {
            options,
            state_id: persist_state(state)?,
        })
    }

    pub fn finish_passkey_authentication_http<L>(
        &self,
        req: FinishPasskeyAuthenticationRequest,
        load_state: L,
    ) -> AuthResult<FinishPasskeyAuthenticationResponse>
    where
        L: FnOnce(&str) -> AuthResult<PasskeyAuthentication>,
    {
        let state = load_state(&req.state_id)?;
        let result = self.finish_passkey_authentication(&req.credential, &state)?;

        Ok(FinishPasskeyAuthenticationResponse { result })
    }

    #[cfg(feature = "attestation")]
    pub fn encode_attested_passkey(&self, passkey: &AttestedPasskey) -> AuthResult<String> {
        serde_json::to_string(passkey).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "attestation")]
    pub fn decode_attested_passkey(&self, encoded: &str) -> AuthResult<AttestedPasskey> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    #[cfg(feature = "attestation")]
    pub fn stored_attested_passkey_from_attested(
        &self,
        passkey: &AttestedPasskey,
    ) -> AuthResult<StoredAttestedPasskey> {
        let attested_passkey_json = self.encode_attested_passkey(passkey)?;
        let credential_id = Self::credential_id_to_base64url(&passkey.cred_id().clone())?;
        let counter = Self::passkey_counter_from_json(&attested_passkey_json);

        Ok(StoredAttestedPasskey {
            credential_id,
            attested_passkey_json,
            counter,
        })
    }

    #[cfg(feature = "attestation")]
    pub fn attested_passkey_from_stored_attested_passkey(
        &self,
        stored: &StoredAttestedPasskey,
    ) -> AuthResult<AttestedPasskey> {
        self.decode_attested_passkey(&stored.attested_passkey_json)
    }

    #[cfg(feature = "attestation")]
    pub fn start_attested_passkey_registration(
        &self,
        user_id: Uuid,
        user_name: &str,
        display_name: &str,
        exclude_credential_ids: Option<Vec<CredentialId>>,
        attestation_ca_list: AttestationCaList,
        ui_hint_authenticator_attachment: Option<AuthenticatorAttachment>,
    ) -> AuthResult<(CreationChallengeResponse, AttestedPasskeyRegistration)> {
        self.inner
            .start_attested_passkey_registration(
                user_id.into_inner(),
                user_name,
                display_name,
                exclude_credential_ids,
                attestation_ca_list,
                ui_hint_authenticator_attachment,
            )
            .map_err(|_| WebAuthnError::RegistrationFailed.into())
    }

    #[cfg(feature = "attestation")]
    pub fn finish_attested_passkey_registration(
        &self,
        state: &AttestedPasskeyRegistration,
        response: &RegisterPublicKeyCredential,
    ) -> AuthResult<AttestedPasskey> {
        self.inner
            .finish_attested_passkey_registration(response, state)
            .map_err(|_| WebAuthnError::RegistrationFailed.into())
    }

    #[cfg(feature = "attestation")]
    pub fn start_attested_passkey_authentication(
        &self,
        allowed_credentials: Vec<AttestedPasskey>,
    ) -> AuthResult<(RequestChallengeResponse, AttestedPasskeyAuthentication)> {
        self.inner
            .start_attested_passkey_authentication(allowed_credentials.as_slice())
            .map_err(|_| WebAuthnError::AuthenticationFailed.into())
    }

    #[cfg(feature = "attestation")]
    pub fn finish_attested_passkey_authentication(
        &self,
        response: &PublicKeyCredential,
        state: &AttestedPasskeyAuthentication,
    ) -> AuthResult<AuthenticationResult> {
        self.inner
            .finish_attested_passkey_authentication(response, state)
            .map_err(|e| match e {
                WebauthnError::CredentialCounterUpdateFailure => {
                    WebAuthnError::CounterRegression.into()
                }
                _ => WebAuthnError::AuthenticationFailed.into(),
            })
    }

    #[cfg(feature = "attestation")]
    pub fn start_attested_passkey_registration_http<P>(
        &self,
        req: StartAttestedPasskeyRegistrationRequest,
        attestation_ca_list: AttestationCaList,
        persist_state: P,
    ) -> AuthResult<StartAttestedPasskeyRegistrationResponse>
    where
        P: FnOnce(AttestedPasskeyRegistration) -> AuthResult<String>,
    {
        let (options, state) = self.start_attested_passkey_registration(
            req.user_id,
            &req.user_name,
            &req.display_name,
            req.exclude_credential_ids,
            attestation_ca_list,
            req.ui_hint_authenticator_attachment,
        )?;

        Ok(StartAttestedPasskeyRegistrationResponse {
            options,
            state_id: persist_state(state)?,
        })
    }

    #[cfg(feature = "attestation")]
    pub fn finish_attested_passkey_registration_http<L>(
        &self,
        req: FinishAttestedPasskeyRegistrationRequest,
        load_state: L,
    ) -> AuthResult<FinishAttestedPasskeyRegistrationResponse>
    where
        L: FnOnce(&str) -> AuthResult<AttestedPasskeyRegistration>,
    {
        let state = load_state(&req.state_id)?;
        let passkey = self.finish_attested_passkey_registration(&state, &req.credential)?;

        Ok(FinishAttestedPasskeyRegistrationResponse {
            stored_attested_passkey: self.stored_attested_passkey_from_attested(&passkey)?,
        })
    }

    #[cfg(feature = "attestation")]
    pub fn start_attested_passkey_authentication_http<P>(
        &self,
        req: StartAttestedPasskeyAuthenticationRequest,
        persist_state: P,
    ) -> AuthResult<StartAttestedPasskeyAuthenticationResponse>
    where
        P: FnOnce(AttestedPasskeyAuthentication) -> AuthResult<String>,
    {
        let mut allowed_credentials = Vec::with_capacity(req.allowed_credentials.len());
        for encoded in req.allowed_credentials {
            allowed_credentials.push(self.decode_attested_passkey(&encoded)?);
        }

        let (options, state) = self.start_attested_passkey_authentication(allowed_credentials)?;

        Ok(StartAttestedPasskeyAuthenticationResponse {
            options,
            state_id: persist_state(state)?,
        })
    }

    #[cfg(feature = "attestation")]
    pub fn finish_attested_passkey_authentication_http<L>(
        &self,
        req: FinishAttestedPasskeyAuthenticationRequest,
        load_state: L,
    ) -> AuthResult<FinishAttestedPasskeyAuthenticationResponse>
    where
        L: FnOnce(&str) -> AuthResult<AttestedPasskeyAuthentication>,
    {
        let state = load_state(&req.state_id)?;
        let result = self.finish_attested_passkey_authentication(&req.credential, &state)?;

        Ok(FinishAttestedPasskeyAuthenticationResponse { result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> WebAuthnService {
        WebAuthnService::new(WebAuthnConfig {
            rp_id: "example.com".to_string(),
            rp_origin: "https://example.com".to_string(),
            rp_name: "Example".to_string(),
        })
        .unwrap()
    }

    #[test]
    fn start_registration_produces_challenge_and_state() {
        let svc = service();
        let user_id = Uuid::new_v7();

        let (ccr, state) = svc
            .start_passkey_registration(user_id, "claire", "Claire", None)
            .unwrap();

        // Basic sanity: challenge options must contain rp + user.
        let json = serde_json::to_value(&ccr).unwrap();
        assert!(json.get("publicKey").is_some());
        assert!(format!("{state:?}").contains("PasskeyRegistration"));
    }

    #[test]
    fn start_authentication_allows_discoverable_credentials() {
        let svc = service();
        let (rcr, _state) = svc.start_passkey_authentication(vec![]).unwrap();

        let json = serde_json::to_value(&rcr).unwrap();
        assert!(json.get("publicKey").is_some());

        let allow_credentials = json
            .get("publicKey")
            .and_then(|v| v.get("allowCredentials"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(allow_credentials.is_empty());
    }

    #[test]
    fn http_start_registration_returns_state_id_and_options() {
        let svc = service();

        let stored = std::cell::RefCell::new(None);

        let resp = svc
            .start_passkey_registration_http(
                StartPasskeyRegistrationRequest {
                    user_id: Uuid::new_v7(),
                    user_name: "claire".to_string(),
                    display_name: "Claire".to_string(),
                    exclude_credential_ids: None,
                },
                |state| {
                    *stored.borrow_mut() = Some(state);
                    Ok("state-1".to_string())
                },
            )
            .unwrap();

        assert_eq!(resp.state_id, "state-1");
        assert!(stored.borrow().is_some());

        let json = serde_json::to_value(&resp.options).unwrap();
        assert!(json.get("publicKey").is_some());
    }

    #[test]
    fn credential_id_to_base64url_round_trips() {
        let credential_id: CredentialId =
            serde_json::from_value(serde_json::json!("AQID")).unwrap();
        let encoded = WebAuthnService::credential_id_to_base64url(&credential_id).unwrap();
        assert_eq!(encoded, "AQID");
    }

    #[test]
    fn passkey_counter_from_json_extracts_counter() {
        assert_eq!(WebAuthnService::passkey_counter_from_json("not json"), None);
        assert_eq!(
            WebAuthnService::passkey_counter_from_json(r#"{"cred":{"counter":5}}"#),
            Some(5)
        );
    }

    #[test]
    fn passkey_transports_from_json_extracts_transports() {
        assert_eq!(
            WebAuthnService::passkey_transports_from_json(
                r#"{"cred":{"transports":["internal","hybrid"]}}"#
            ),
            vec!["internal".to_string(), "hybrid".to_string()]
        );
    }

    #[test]
    fn passkey_sync_info_from_json_extracts_backup_flags() {
        let info = WebAuthnService::passkey_sync_info_from_json(
            r#"{"cred":{"transports":["internal"],"backup_eligible":true,"backup_state":false,"user_verified":true}}"#,
        );

        assert_eq!(info.transports, vec!["internal".to_string()]);
        assert!(info.backup_eligible);
        assert!(!info.backup_state);
        assert!(info.user_verified);
    }

    #[test]
    fn credential_metadata_from_stored_passkey_uses_transports_and_counter() {
        let stored = StoredPasskey {
            credential_id: "AQID".to_string(),
            passkey_json: r#"{"cred":{"transports":["internal"],"counter":9}}"#.to_string(),
            counter: Some(7),
        };

        let meta = WebAuthnService::credential_metadata_from_stored_passkey(&stored);
        match meta {
            CredentialMetadata::Passkey {
                credential_id,
                transports,
                last_counter,
            } => {
                assert_eq!(credential_id, "AQID");
                assert_eq!(transports, vec!["internal".to_string()]);
                assert_eq!(last_counter, 7);
            }
            _ => panic!("expected passkey metadata"),
        }
    }

    #[test]
    fn invalid_stored_passkey_rejects_decode() {
        let svc = service();

        let stored = StoredPasskey {
            credential_id: "AQID".to_string(),
            passkey_json: "not json".to_string(),
            counter: None,
        };

        let err = svc.passkey_from_stored_passkey(&stored).unwrap_err();
        assert!(matches!(err, AuthError::BadRequest(_)));
    }

    #[test]
    fn http_start_authentication_rejects_invalid_allowed_credentials() {
        let svc = service();

        let err = svc
            .start_passkey_authentication_http(
                StartPasskeyAuthenticationRequest {
                    allowed_credentials: vec!["not json".to_string()],
                },
                |_state| Ok("state-1".to_string()),
            )
            .unwrap_err();

        assert!(matches!(err, AuthError::BadRequest(_)));
    }

    #[test]
    fn invalid_finish_registration_fails_gracefully() {
        let svc = service();
        let user_id = Uuid::new_v7();
        let (_ccr, state) = svc
            .start_passkey_registration(user_id, "claire", "Claire", None)
            .unwrap();

        // Must be valid base64url to deserialize, but still invalid WebAuthn data.
        let bogus: RegisterPublicKeyCredential = serde_json::from_value(serde_json::json!({
            "id": "AQID",
            "rawId": "AQID",
            "type": "public-key",
            "response": {
                "attestationObject": "AQID",
                "clientDataJSON": "AQID"
            }
        }))
        .unwrap();

        let err = svc.finish_passkey_registration(&state, &bogus).unwrap_err();
        assert!(matches!(err, AuthError::PassKeyRegistrationFailed));
    }

    #[test]
    fn passkey_encoding_rejects_invalid_json() {
        let svc = service();
        let err = svc.decode_passkey("not json").unwrap_err();
        assert!(matches!(err, AuthError::BadRequest(_)));
    }

    #[test]
    fn auth_error_mapping_is_stable() {
        assert_eq!(
            AuthError::PassKeyRegistrationFailed.code(),
            "auth.passkey_registration_failed"
        );
        assert_eq!(
            AuthError::PassKeyAuthenticationFailed.code(),
            "auth.passkey_authentication_failed"
        );
        assert_eq!(
            AuthError::PassKeyCounterRegression.code(),
            "auth.passkey_counter_regression"
        );
    }
}
