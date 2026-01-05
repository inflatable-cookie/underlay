use async_trait::async_trait;

use crate::{AuthResult, Principal};

/// Provider boundary for authentication.
///
/// Underlay defines the interface; applications provide implementations.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal>;
}
