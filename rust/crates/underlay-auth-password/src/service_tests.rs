use super::*;
use underlay_auth::hashing::Argon2Hasher;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use underlay_ratelimit::InMemoryBackend;

use async_trait::async_trait;
use underlay_auth::{Credential, CredentialMetadata, User, UserStatus};
use underlay_core::Uuid;

#[derive(Debug)]
struct MemoryRepo {
    users_by_email: Mutex<HashMap<String, User>>,
    users_by_id: Mutex<HashMap<Uuid, User>>,
    credentials_by_user: Mutex<HashMap<Uuid, Credential>>,
    credential_ids: Mutex<HashSet<Uuid>>,
    failed_counts: Mutex<HashMap<Uuid, u32>>,
    locked_until: Mutex<HashMap<Uuid, chrono::DateTime<chrono::Utc>>>,
    lockout_threshold: u32,
}

impl MemoryRepo {
    fn new(lockout_threshold: u32) -> Self {
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

    async fn insert_user(&self, user: User) {
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

fn make_user(email: &str) -> User {
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

#[tokio::test]
async fn login_success_resets_failures() {
    let user = make_user("a@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(
        repo.clone(),
        hasher,
        verifier,
        rate_limiter,
        Some(PasswordConfig {
            max_failed_attempts: 10,
            ..PasswordConfig::default()
        }),
    );

    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    let err = service
        .verify_login(&user.email, "wrong")
        .await
        .unwrap_err();
    assert!(matches!(err, PasswordAuthError::WrongPassword));

    let ok = service
        .verify_login(&user.email, "S0mething$trong!")
        .await
        .unwrap();
    assert_eq!(ok.id, user.id);

    assert_eq!(repo.get_failed_login_count(user.id).await.unwrap(), 0);
    assert!(repo
        .get_lockout_remaining_seconds(user.id)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn lockout_triggers_after_n_failures() {
    let user = make_user("b@example.com");
    let repo = Arc::new(MemoryRepo::new(2));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(
        repo.clone(),
        hasher,
        verifier,
        rate_limiter,
        Some(PasswordConfig {
            max_failed_attempts: 2,
            lockout_duration_seconds: 900,
            ..PasswordConfig::default()
        }),
    );

    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    let err1 = service
        .verify_login(&user.email, "wrong")
        .await
        .unwrap_err();
    assert!(matches!(err1, PasswordAuthError::WrongPassword));

    let err2 = service
        .verify_login(&user.email, "wrong")
        .await
        .unwrap_err();
    assert!(matches!(err2, PasswordAuthError::AccountLocked { .. }));

    // Subsequent attempts are blocked by get_lockout_remaining_seconds().
    let err3 = service
        .verify_login(&user.email, "S0mething$trong!")
        .await
        .unwrap_err();
    assert!(matches!(err3, PasswordAuthError::AccountLocked { .. }));
}

#[tokio::test]
async fn rate_limit_blocks_login_attempts() {
    let user = make_user("c@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    // Create a rate limiter with a very low limit (1 request)
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(
        repo.clone(),
        hasher,
        verifier,
        rate_limiter.clone(),
        Some(PasswordConfig {
            rate_limit_max_attempts: 1, // Only 1 attempt allowed
            rate_limit_window_seconds: 3600,
            ..PasswordConfig::default()
        }),
    );

    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    let ip = "1.2.3.4";

    // First attempt should succeed
    let ok = service
        .verify_login_with_context(&user.email, "S0mething$trong!", Some(ip))
        .await
        .unwrap();
    assert_eq!(ok.id, user.id);

    // Second attempt should be rate limited
    let err = service
        .verify_login_with_context(&user.email, "S0mething$trong!", Some(ip))
        .await
        .unwrap_err();
    assert!(matches!(err, PasswordAuthError::RateLimited { .. }));
}

#[tokio::test]
async fn compromised_password_rejected_when_enabled() {
    let user = make_user("d@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(
        repo,
        hasher,
        verifier,
        rate_limiter,
        Some(PasswordConfig {
            check_compromised: true,
            compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
            ..PasswordConfig::default()
        }),
    );

    let err = service
        .set_password(user.id, "password123")
        .await
        .unwrap_err();
    assert!(matches!(err, PasswordAuthError::PasswordCompromised));
}

#[tokio::test]
async fn change_password_rejects_wrong_current_password() {
    let user = make_user("e@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(repo.clone(), hasher, verifier, rate_limiter, None);

    service
        .set_password(user.id, "CorrectHorse1!")
        .await
        .unwrap();

    let err = service
        .change_password(user.id, "WrongPassword1!", "NewPassword2#")
        .await
        .unwrap_err();

    assert!(matches!(err, PasswordAuthError::WrongPassword));
}

#[tokio::test]
async fn change_password_rejects_same_password() {
    let user = make_user("f@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(repo.clone(), hasher, verifier, rate_limiter, None);

    service
        .set_password(user.id, "CorrectHorse1!")
        .await
        .unwrap();

    let err = service
        .change_password(user.id, "CorrectHorse1!", "CorrectHorse1!")
        .await
        .unwrap_err();

    assert!(matches!(err, PasswordAuthError::PasswordSameAsCurrent));
}

#[tokio::test]
async fn change_password_updates_hash_and_allows_login() {
    let user = make_user("g@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(repo.clone(), hasher, verifier, rate_limiter, None);

    service
        .set_password(user.id, "CorrectHorse1!")
        .await
        .unwrap();

    service
        .change_password(user.id, "CorrectHorse1!", "NewPassword2#")
        .await
        .unwrap();

    assert!(service
        .verify_login(&user.email, "NewPassword2#")
        .await
        .is_ok());
    assert!(service
        .verify_login(&user.email, "CorrectHorse1!")
        .await
        .is_err());
}

#[tokio::test]
async fn reset_password_updates_hash_and_allows_login() {
    let user = make_user("h@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(repo.clone(), hasher, verifier, rate_limiter, None);

    service
        .set_password(user.id, "CorrectHorse1!")
        .await
        .unwrap();

    service
        .reset_password(user.id, "ResetPassword2#")
        .await
        .unwrap();

    assert!(service
        .verify_login(&user.email, "ResetPassword2#")
        .await
        .is_ok());
}

#[tokio::test]
async fn verify_login_normalizes_email_for_lookup() {
    let user = make_user("i@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let hasher = Arc::new(Argon2Hasher::new());
    let verifier = hasher.clone();
    let rate_limiter = Arc::new(InMemoryBackend::new());

    let service = PasswordAuthService::new(repo.clone(), hasher, verifier, rate_limiter, None);

    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    let logged_in = service
        .verify_login("  I@EXAMPLE.COM  ", "S0mething$trong!")
        .await
        .unwrap();

    assert_eq!(logged_in.id, user.id);
    assert_eq!(logged_in.email, "i@example.com");
}
