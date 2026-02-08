mod errors;
mod extractors;
mod principal;
mod provider;
mod repository;
mod types;

#[cfg(feature = "postgres")]
pub mod state;

#[cfg(feature = "hashing")]
pub mod hashing;

#[cfg(test)]
mod extractors_tests;

pub use crate::errors::{AuthError, AuthResult};
pub use crate::extractors::{Authenticated, HasAuthProvider, OptionalAuthenticated};
pub use crate::principal::{Principal, RoleSet};
pub use crate::provider::AuthProvider;
pub use crate::repository::{
    AuditLogRepository, AuthRepository, CredentialRepository, NewSession, RepoResult,
    SessionRepository, UserRepository,
};
pub use crate::types::{
    AuthEvent, AuthEventBuilder, AuthEventType, BackupCode, Credential, CredentialMetadata,
    CredentialType, Session, SessionStatus, User, UserStatus,
};

#[cfg(feature = "postgres")]
pub use crate::state::{AuthStateError, AuthStateRow, AuthStateStore};

#[cfg(feature = "hashing")]
pub use crate::hashing::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};
