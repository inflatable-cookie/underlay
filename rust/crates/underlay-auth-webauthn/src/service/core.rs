use underlay_auth::{AuthError, AuthResult};
use underlay_core::Uuid;
use webauthn_rs::prelude::*;

use super::WebAuthnService;
use crate::{CredentialId, WebAuthnError};

impl WebAuthnService {
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
}
