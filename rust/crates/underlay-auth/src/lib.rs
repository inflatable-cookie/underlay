mod errors;
mod extractors;
mod principal;
mod provider;
mod repository;
mod types;

#[cfg(test)]
mod extractors_tests;

pub use crate::errors::{AuthError, AuthResult};
pub use crate::extractors::{Authenticated, HasAuthProvider, OptionalAuthenticated};
pub use crate::principal::{Principal, RoleSet};
pub use crate::provider::AuthProvider;
pub use crate::repository::{
    AuditLogRepository, AuthRepository, CredentialRepository, RateLimitRepository, RepoResult,
    SessionRepository, UserRepository,
};
pub use crate::types::{
    AuthEvent, AuthEventBuilder, AuthEventType, BackupCode, Credential, CredentialMetadata,
    CredentialType, Session, SessionStatus, User, UserStatus,
};
