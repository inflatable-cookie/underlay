use underlay_auth::{AuthError, AuthResult, CredentialMetadata};
use underlay_core::Uuid;
use url::Url;
use webauthn_rs::prelude::*;

use crate::{
    CredentialId, FinishPasskeyAuthenticationRequest, FinishPasskeyAuthenticationResponse,
    FinishPasskeyRegistrationRequest, FinishPasskeyRegistrationResponse, PasskeySyncInfo,
    StartPasskeyAuthenticationRequest, StartPasskeyAuthenticationResponse,
    StartPasskeyRegistrationRequest, StartPasskeyRegistrationResponse, StoredPasskey,
    StoredPasskeyUpdate, WebAuthnConfig, WebAuthnError,
};

#[derive(Debug, Clone)]
pub struct WebAuthnService {
    pub(crate) inner: Webauthn,
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
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn start passkey registration failed");
                WebAuthnError::RegistrationFailed.into()
            })
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
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn finish passkey registration failed");
                WebAuthnError::RegistrationFailed.into()
            })
    }

    pub fn start_passkey_authentication(
        &self,
        allowed_credentials: Vec<Passkey>,
    ) -> AuthResult<(RequestChallengeResponse, PasskeyAuthentication)> {
        self.inner
            .start_passkey_authentication(allowed_credentials.as_slice())
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn start passkey authentication failed");
                WebAuthnError::AuthenticationFailed.into()
            })
    }

    /// Begin a discoverable (username-less) authentication.
    ///
    /// This is primarily used for conditional UI / passkey autofill flows.
    pub fn start_discoverable_authentication(
        &self,
    ) -> AuthResult<(RequestChallengeResponse, DiscoverableAuthentication)> {
        self.inner
            .start_discoverable_authentication()
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn start discoverable authentication failed");
                AuthError::from(WebAuthnError::AuthenticationFailed)
            })
    }

    /// Extract the user UUID and credential ID from a discoverable authentication response.
    pub fn identify_discoverable_authentication(
        &self,
        response: &PublicKeyCredential,
    ) -> AuthResult<(Uuid, CredentialId)> {
        let (user_uuid, cred_id_bytes) = self
            .inner
            .identify_discoverable_authentication(response)
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn identify discoverable authentication failed");
                AuthError::from(WebAuthnError::AuthenticationFailed)
            })?;

        Ok((Uuid(user_uuid), CredentialId::from(cred_id_bytes)))
    }

    pub fn finish_passkey_authentication(
        &self,
        response: &PublicKeyCredential,
        state: &PasskeyAuthentication,
    ) -> AuthResult<AuthenticationResult> {
        self.inner
            .finish_passkey_authentication(response, state)
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn finish passkey authentication failed");
                match err {
                    WebauthnError::CredentialCounterUpdateFailure => {
                        WebAuthnError::CounterRegression.into()
                    }
                    _ => WebAuthnError::AuthenticationFailed.into(),
                }
            })
    }

    pub fn finish_discoverable_authentication(
        &self,
        response: &PublicKeyCredential,
        state: &DiscoverableAuthentication,
        allowed_credentials: Vec<Passkey>,
    ) -> AuthResult<AuthenticationResult> {
        let discoverable_keys = allowed_credentials
            .iter()
            .map(DiscoverableKey::from)
            .collect::<Vec<_>>();

        self.inner
            .finish_discoverable_authentication(response, state.clone(), &discoverable_keys)
            .map_err(|err| {
                tracing::warn!(error = ?err, "webauthn finish discoverable authentication failed");
                match err {
                    WebauthnError::CredentialCounterUpdateFailure => {
                        WebAuthnError::CounterRegression.into()
                    }
                    _ => WebAuthnError::AuthenticationFailed.into(),
                }
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
}
