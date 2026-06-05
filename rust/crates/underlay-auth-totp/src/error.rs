#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TotpError {
    #[error("invalid totp secret")]
    InvalidSecret,
    #[error("invalid totp code")]
    InvalidCode,
    #[error("replay detected")]
    Replay,
}

underlay_auth::impl_auth_error_from!(TotpError, err, {
    TotpError::InvalidSecret | TotpError::InvalidCode | TotpError::Replay => {
        underlay_auth::AuthError::TwoFactorInvalid
    }
});
