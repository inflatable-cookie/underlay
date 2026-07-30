mod errors;
mod extractors;
mod principal;
mod provider;
mod repository;
mod secret_cipher;
mod types;

#[cfg(feature = "hashing")]
pub mod hashing;

#[cfg(test)]
#[path = "tests/extractors_tests.rs"]
mod extractors_tests;

pub use crate::errors::{AuthError, AuthResult};
pub use crate::extractors::{Authenticated, HasAuthProvider, OptionalAuthenticated};
pub use crate::principal::{Principal, RoleSet};
pub use crate::provider::AuthProvider;
pub use crate::repository::{
    AuditLogRepository, AuthRepository, CredentialRepository, NewSession, RepoResult,
    SessionRepository, UserRepository,
};
pub use crate::secret_cipher::SecretCipher;
pub use crate::types::{
    AuthEvent, AuthEventBuilder, AuthEventType, BackupCode, Credential, CredentialMetadata,
    CredentialType, Session, SessionStatus, User, UserStatus,
};

#[cfg(feature = "hashing")]
pub use crate::hashing::{Argon2Hasher, PasswordHasherExt, PasswordVerifierExt};
