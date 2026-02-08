//! Attested passkey types and operations (feature-gated).

use serde::{Deserialize, Serialize};
use underlay_auth::AuthResult;
use underlay_core::Uuid;
use webauthn_rs::prelude::*;

use crate::{CredentialId, StoredAttestedPasskey, WebAuthnError, WebAuthnService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyRegistrationRequest {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
    pub exclude_credential_ids: Option<Vec<CredentialId>>,
    pub ui_hint_authenticator_attachment: Option<AuthenticatorAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyRegistrationResponse {
    pub options: CreationChallengeResponse,
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyRegistrationRequest {
    pub state_id: String,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyRegistrationResponse {
    pub stored_attested_passkey: StoredAttestedPasskey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyAuthenticationRequest {
    /// JSON-encoded `AttestedPasskey` values that are allowed for this authentication.
    pub allowed_credentials: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAttestedPasskeyAuthenticationResponse {
    pub options: RequestChallengeResponse,
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyAuthenticationRequest {
    pub state_id: String,
    pub credential: PublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishAttestedPasskeyAuthenticationResponse {
    pub result: AuthenticationResult,
}

impl WebAuthnService {
    pub fn encode_attested_passkey(&self, passkey: &AttestedPasskey) -> AuthResult<String> {
        serde_json::to_string(passkey).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

    pub fn decode_attested_passkey(&self, encoded: &str) -> AuthResult<AttestedPasskey> {
        serde_json::from_str(encoded).map_err(|_| WebAuthnError::InvalidPasskeyEncoding.into())
    }

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

    pub fn attested_passkey_from_stored_attested_passkey(
        &self,
        stored: &StoredAttestedPasskey,
    ) -> AuthResult<AttestedPasskey> {
        self.decode_attested_passkey(&stored.attested_passkey_json)
    }

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

    pub fn finish_attested_passkey_registration(
        &self,
        state: &AttestedPasskeyRegistration,
        response: &RegisterPublicKeyCredential,
    ) -> AuthResult<AttestedPasskey> {
        self.inner
            .finish_attested_passkey_registration(response, state)
            .map_err(|_| WebAuthnError::RegistrationFailed.into())
    }

    pub fn start_attested_passkey_authentication(
        &self,
        allowed_credentials: Vec<AttestedPasskey>,
    ) -> AuthResult<(RequestChallengeResponse, AttestedPasskeyAuthentication)> {
        self.inner
            .start_attested_passkey_authentication(allowed_credentials.as_slice())
            .map_err(|_| WebAuthnError::AuthenticationFailed.into())
    }

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
