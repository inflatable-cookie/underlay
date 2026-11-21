use underlay_auth::AuthResult;
use underlay_core::Uuid;
use webauthn_rs::prelude::{
    AuthenticationResult, DiscoverableAuthentication, Passkey, PasskeyAuthentication,
    PasskeyRegistration,
};

use super::WebAuthnService;
use crate::{
    CredentialId, FinishPasskeyAuthenticationRequest, FinishPasskeyAuthenticationResponse,
    FinishPasskeyRegistrationRequest, FinishPasskeyRegistrationResponse,
    PasskeyAuthenticationState, StartPasskeyAuthenticationRequest,
    StartPasskeyAuthenticationResponse, StartPasskeyRegistrationRequest,
    StartPasskeyRegistrationResponse,
};

impl WebAuthnService {
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

    /// Start allow-listed authentication or conditional UI when the list is empty.
    ///
    /// Persist the state variant on the server for the corresponding finish
    /// method. Use an empty list only when intentionally starting discovery.
    pub fn start_passkey_authentication_http<P>(
        &self,
        req: StartPasskeyAuthenticationRequest,
        persist_state: P,
    ) -> AuthResult<StartPasskeyAuthenticationResponse>
    where
        P: FnOnce(PasskeyAuthenticationState) -> AuthResult<String>,
    {
        let (options, state) = if req.allowed_credentials.is_empty() {
            let (options, state) = self.start_discoverable_authentication()?;
            (options, PasskeyAuthenticationState::Discoverable(state))
        } else {
            let allowed_credentials = req
                .allowed_credentials
                .iter()
                .map(|encoded| self.decode_passkey(encoded))
                .collect::<AuthResult<Vec<_>>>()?;
            let (options, state) = self.start_passkey_authentication(allowed_credentials)?;
            (options, PasskeyAuthenticationState::AllowListed(state))
        };

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

    /// Verify a discoverable assertion and return the authenticated user and result.
    ///
    /// Consume the saved state for this ceremony. Resolve the passkey from
    /// trusted storage by both the user ID and credential ID supplied to the
    /// lookup callback. Those identifiers are unverified until this method succeeds.
    pub fn finish_discoverable_authentication_http<L, C>(
        &self,
        req: FinishPasskeyAuthenticationRequest,
        load_state: L,
        load_credential: C,
    ) -> AuthResult<(Uuid, AuthenticationResult)>
    where
        L: FnOnce(&str) -> AuthResult<DiscoverableAuthentication>,
        C: FnOnce(Uuid, &CredentialId) -> AuthResult<Passkey>,
    {
        let state = load_state(&req.state_id)?;
        let (user_id, credential_id) =
            self.identify_discoverable_authentication(&req.credential)?;
        let passkey = load_credential(user_id, &credential_id)?;
        let result =
            self.finish_discoverable_authentication(&req.credential, &state, vec![passkey])?;
        Ok((user_id, result))
    }
}
