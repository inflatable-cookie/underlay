#[derive(Debug, thiserror::Error)]
pub enum OAuthServiceError {
    #[error("invalid oauth configuration")]
    InvalidConfig,
    #[error("oauth exchange failed")]
    ExchangeFailed,
    #[error("oauth userinfo failed")]
    UserInfoFailed,
    #[error("oauth refresh failed")]
    RefreshFailed,
}

underlay_auth::impl_auth_error_from!(OAuthServiceError, err, {
    OAuthServiceError::InvalidConfig => {
        underlay_auth::AuthError::Internal("invalid oauth configuration".into())
    }
    OAuthServiceError::ExchangeFailed => underlay_auth::AuthError::OAuthError("exchange failed".into()),
    OAuthServiceError::UserInfoFailed => underlay_auth::AuthError::OAuthError("userinfo failed".into()),
    OAuthServiceError::RefreshFailed => underlay_auth::AuthError::OAuthTokenRefreshFailed,
});
