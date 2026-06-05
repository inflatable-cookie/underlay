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

underlay_auth::impl_auth_error_from!(WebAuthnError, err, {
    WebAuthnError::InvalidConfig => {
        underlay_auth::AuthError::Internal("invalid webauthn configuration".into())
    }
    WebAuthnError::RegistrationFailed => underlay_auth::AuthError::PassKeyRegistrationFailed,
    WebAuthnError::AuthenticationFailed => underlay_auth::AuthError::PassKeyAuthenticationFailed,
    WebAuthnError::CounterRegression => underlay_auth::AuthError::PassKeyCounterRegression,
    WebAuthnError::InvalidPasskeyEncoding => {
        underlay_auth::AuthError::BadRequest("invalid passkey encoding".into())
    }
});
