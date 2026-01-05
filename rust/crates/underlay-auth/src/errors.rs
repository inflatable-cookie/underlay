use underlay_core::AppError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    /// Missing credentials.
    Unauthorized,
    /// Credentials present but do not grant access.
    Forbidden,
    /// Credentials are present but invalid/expired.
    InvalidToken,
}

impl AuthError {
    pub fn into_app_error(self) -> AppError {
        match self {
            AuthError::Unauthorized => AppError::new("auth.unauthorized", "You are not signed in."),
            AuthError::Forbidden => AppError::new("auth.forbidden", "You do not have permission."),
            AuthError::InvalidToken => AppError::new("auth.unauthorized", "Invalid credentials."),
        }
    }
}

pub type AuthResult<T> = Result<T, AuthError>;
