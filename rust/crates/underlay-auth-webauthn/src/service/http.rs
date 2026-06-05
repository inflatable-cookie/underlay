use underlay_auth::AuthResult;
use webauthn_rs::prelude::{PasskeyAuthentication, PasskeyRegistration};

use super::WebAuthnService;
use crate::{
    FinishPasskeyAuthenticationRequest, FinishPasskeyAuthenticationResponse,
    FinishPasskeyRegistrationRequest, FinishPasskeyRegistrationResponse,
    StartPasskeyAuthenticationRequest, StartPasskeyAuthenticationResponse,
    StartPasskeyRegistrationRequest, StartPasskeyRegistrationResponse,
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
