use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use underlay_auth::hashing::Argon2Hasher;
use underlay_auth::{Credential, CredentialMetadata, User, UserStatus};
use underlay_core::Uuid;
use underlay_ratelimit::InMemoryBackend;

use super::super::{
    FailedLoginAttempt, PasswordAuthError, PasswordAuthRepository, PasswordAuthResult,
    PasswordAuthService, PasswordConfig,
};

pub(crate) type TestPasswordAuthService =
    PasswordAuthService<MemoryRepo, Argon2Hasher, Argon2Hasher, InMemoryBackend>;

#[derive(Debug)]
pub(crate) struct MemoryRepo {
    users_by_email: Mutex<HashMap<String, User>>,
    users_by_id: Mutex<HashMap<Uuid, User>>,
    credentials_by_user: Mutex<HashMap<Uuid, Credential>>,
    credential_ids: Mutex<HashSet<Uuid>>,
    failed_counts: Mutex<HashMap<Uuid, u32>>,
    locked_until: Mutex<HashMap<Uuid, chrono::DateTime<chrono::Utc>>>,
    lockout_threshold: u32,
}

impl MemoryRepo {
    pub(crate) fn new(lockout_threshold: u32) -> Self {
        Self {
            users_by_email: Mutex::new(HashMap::new()),
            users_by_id: Mutex::new(HashMap::new()),
            credentials_by_user: Mutex::new(HashMap::new()),
            credential_ids: Mutex::new(HashSet::new()),
            failed_counts: Mutex::new(HashMap::new()),
            locked_until: Mutex::new(HashMap::new()),
            lockout_threshold,
        }
    }

    pub(crate) async fn insert_user(&self, user: User) {
        self.users_by_email
            .lock()
            .await
            .insert(user.email.clone(), user.clone());
        self.users_by_id.lock().await.insert(user.id, user);
    }
}

#[async_trait]
impl PasswordAuthRepository for MemoryRepo {
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>> {
        Ok(self.users_by_email.lock().await.get(email).cloned())
    }

    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>> {
        Ok(self.users_by_id.lock().await.get(&user_id).cloned())
    }

    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>> {
        Ok(self.credentials_by_user.lock().await.get(&user_id).cloned())
    }

    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential> {
        let now = chrono::Utc::now();
        let cred = Credential {
            id: Uuid::new_v7(),
            user_id,
            credential_type: underlay_auth::CredentialType::Password,
            secret_encrypted: password_hash.to_string(),
            metadata: CredentialMetadata::Password {
                algorithm: "argon2id".to_string(),
                memory_kb: 65536,
                iterations: 3,
                parallelism: 4,
            },
            verified: true,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        };

        self.credential_ids.lock().await.insert(cred.id);
        self.credentials_by_user
            .lock()
            .await
            .insert(user_id, cred.clone());
        Ok(cred)
    }

    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()> {
        if !self.credential_ids.lock().await.contains(&credential_id) {
            return Err(PasswordAuthError::CredentialNotFound);
        }

        let mut creds = self.credentials_by_user.lock().await;
        let (user_id, mut cred) = creds
            .iter()
            .find(|(_, c)| c.id == credential_id)
            .map(|(k, v)| (*k, v.clone()))
            .ok_or(PasswordAuthError::CredentialNotFound)?;

        cred.secret_encrypted = password_hash.to_string();
        cred.updated_at = chrono::Utc::now();
        creds.insert(user_id, cred);
        Ok(())
    }

    async fn delete_password_credential(&self, credential_id: Uuid) -> PasswordAuthResult<()> {
        self.credential_ids.lock().await.remove(&credential_id);
        let mut creds = self.credentials_by_user.lock().await;
        creds.retain(|_, c| c.id != credential_id);
        Ok(())
    }

    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt> {
        let mut counts = self.failed_counts.lock().await;
        let entry = counts.entry(user_id).or_insert(0);
        *entry += 1;
        let count = *entry;

        let threshold = std::cmp::min(self.lockout_threshold, max_failed_attempts);
        let lockout_remaining_seconds = if count >= threshold {
            let until =
                chrono::Utc::now() + chrono::Duration::seconds(lockout_duration_seconds as i64);
            self.locked_until.lock().await.insert(user_id, until);
            Some(lockout_duration_seconds)
        } else {
            None
        };

        Ok(FailedLoginAttempt {
            count,
            lockout_remaining_seconds,
        })
    }

    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()> {
        self.failed_counts.lock().await.remove(&user_id);
        self.locked_until.lock().await.remove(&user_id);
        Ok(())
    }

    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32> {
        Ok(*self.failed_counts.lock().await.get(&user_id).unwrap_or(&0))
    }

    async fn get_lockout_remaining_seconds(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<u64>> {
        let mut locked_until = self.locked_until.lock().await;
        let Some(until) = locked_until.get(&user_id).copied() else {
            return Ok(None);
        };

        let now = chrono::Utc::now();
        if until <= now {
            locked_until.remove(&user_id);
            return Ok(None);
        }

        let secs = (until - now).num_seconds();
        Ok(Some(secs.max(1) as u64))
    }
}

pub(crate) fn make_user(email: &str) -> User {
    let now = chrono::Utc::now();
    User {
        id: Uuid::new_v7(),
        email: email.to_string(),
        display_name: Some("Test".to_string()),
        status: UserStatus::Active,
        created_at: now,
        updated_at: now,
    }
}

pub(crate) fn service(
    repo: Arc<MemoryRepo>,
    config: Option<PasswordConfig>,
) -> TestPasswordAuthService {
    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());
    PasswordAuthService::new(repo, hasher, verifier, rate_limiter, config)
}
