//! Password authentication service.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use underlay_core::Uuid;

use underlay_auth::{AuthError, Credential, CredentialMetadata, User, UserStatus};
use underlay_ratelimit::{RateLimitBackend, RateLimitConfig};

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use crate::hasher::{PasswordHasherExt, PasswordVerifierExt};
use crate::strength::PasswordStrengthAnalyzer;

/// Configuration for password authentication.
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    /// Maximum failed login attempts before lockout.
    pub max_failed_attempts: u32,
    /// Lockout duration in seconds.
    pub lockout_duration_seconds: u64,
    /// Rate limit window in seconds.
    pub rate_limit_window_seconds: u64,
    /// Maximum attempts per rate limit window.
    pub rate_limit_max_attempts: u32,
    /// Minimum password length.
    pub min_password_length: usize,
    /// Whether to check compromised passwords.
    pub check_compromised: bool,
    /// Strategy for checking if a password is compromised.
    pub compromised_password_strategy: CompromisedPasswordStrategy,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
            rate_limit_window_seconds: 3600,
            rate_limit_max_attempts: 10,
            min_password_length: 8,
            check_compromised: false,
            compromised_password_strategy: CompromisedPasswordStrategy::LocalBlocklist,
        }
    }
}

/// Strategy for compromised-password checks.
#[derive(Debug, Clone)]
pub enum CompromisedPasswordStrategy {
    /// Offline-only: local blocklist of extremely common passwords.
    ///
    /// This is fast and has no network dependency, but does not detect all breached passwords.
    LocalBlocklist,

    /// Online (optional): HIBP Pwned Passwords k-anonymity range API.
    ///
    /// This sends only the first 5 hex chars of the SHA-1 password hash.
    #[cfg(feature = "hibp")]
    HibpKAnonymity {
        /// Base URL for the API.
        ///
        /// Example: `https://api.pwnedpasswords.com`.
        api_base_url: String,
        /// User agent string for HTTP requests.
        user_agent: String,
    },
}

/// Repository for user and credential operations.
#[async_trait]
pub trait PasswordAuthRepository: Send + Sync {
    /// Find a user by email.
    async fn find_user_by_email(&self, email: &str) -> PasswordAuthResult<Option<User>>;

    /// Find a user by ID.
    async fn find_user_by_id(&self, user_id: Uuid) -> PasswordAuthResult<Option<User>>;

    /// Find a password credential for a user.
    async fn find_password_credential(
        &self,
        user_id: Uuid,
    ) -> PasswordAuthResult<Option<Credential>>;

    /// Create a password credential for a user.
    async fn create_password_credential(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<Credential>;

    /// Update a password credential.
    async fn update_password_credential(
        &self,
        credential_id: Uuid,
        password_hash: &str,
    ) -> PasswordAuthResult<()>;

    /// Delete a password credential.
    async fn delete_password_credential(&self, credential_id: Uuid) -> PasswordAuthResult<()>;

    /// Record a failed login attempt.
    ///
    /// Implementations should increment failure counters and, when the threshold is reached,
    /// apply a lockout and return the remaining lockout seconds.
    async fn record_failed_login(
        &self,
        user_id: Uuid,
        max_failed_attempts: u32,
        lockout_duration_seconds: u64,
    ) -> PasswordAuthResult<FailedLoginAttempt>;

    /// Reset failed login attempts.
    async fn reset_failed_logins(&self, user_id: Uuid) -> PasswordAuthResult<()>;

    /// Get failed login count for a user.
    async fn get_failed_login_count(&self, user_id: Uuid) -> PasswordAuthResult<u32>;

    /// Returns remaining lockout seconds, or `None` if not locked out.
    async fn get_lockout_remaining_seconds(&self, user_id: Uuid)
        -> PasswordAuthResult<Option<u64>>;
}

/// Result of recording a failed login attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FailedLoginAttempt {
    pub count: u32,
    pub lockout_remaining_seconds: Option<u64>,
}

/// Service for password-based authentication.
#[derive(Debug, Clone)]
pub struct PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    repository: Arc<R>,
    hasher: Arc<H>,
    verifier: Arc<V>,
    rate_limiter: Arc<L>,
    analyzer: Arc<PasswordStrengthAnalyzer>,
    config: PasswordConfig,
}

impl<R, H, V, L> PasswordAuthService<R, H, V, L>
where
    R: PasswordAuthRepository,
    H: PasswordHasherExt,
    V: PasswordVerifierExt,
    L: RateLimitBackend,
{
    /// Create a new password auth service.
    pub fn new(
        repository: Arc<R>,
        hasher: Arc<H>,
        verifier: Arc<V>,
        rate_limiter: Arc<L>,
        config: Option<PasswordConfig>,
    ) -> Self {
        let config = config.unwrap_or_default();
        let analyzer = PasswordStrengthAnalyzer::new().with_min_length(config.min_password_length);

        Self {
            repository,
            hasher,
            verifier,
            rate_limiter,
            analyzer: Arc::new(analyzer),
            config,
        }
    }

    async fn is_compromised_password(&self, password: &str) -> PasswordAuthResult<bool> {
        if !self.config.check_compromised {
            return Ok(false);
        }

        match &self.config.compromised_password_strategy {
            CompromisedPasswordStrategy::LocalBlocklist => {
                Ok(self.analyzer.is_common_password(password))
            }
            #[cfg(feature = "hibp")]
            CompromisedPasswordStrategy::HibpKAnonymity {
                api_base_url,
                user_agent,
            } => hibp_k_anonymity_check(password, api_base_url, user_agent).await,
        }
    }

    /// Create or replace a user's password credential.
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> PasswordAuthResult<Credential> {
        let analysis = self.analyzer.analyze(password);

        if self.is_compromised_password(password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let new_hash = self.hasher.hash_password(password.as_bytes())?;

        if let Some(existing) = self.repository.find_password_credential(user_id).await? {
            if self
                .verifier
                .verify_password(password.as_bytes(), &existing.secret_encrypted)?
            {
                return Err(PasswordAuthError::PasswordSameAsCurrent.into());
            }

            self.repository
                .update_password_credential(existing.id, &new_hash)
                .await?;

            let updated = self
                .repository
                .find_password_credential(user_id)
                .await?
                .ok_or(PasswordAuthError::Internal(
                    "Password credential missing after update".to_string(),
                ))?;

            Ok(updated)
        } else {
            self.repository
                .create_password_credential(user_id, &new_hash)
                .await
        }
    }

    /// Verify login credentials.
    pub async fn verify_login(&self, email: &str, password: &str) -> PasswordAuthResult<User> {
        self.verify_login_with_context(email, password, None).await
    }

    /// Verify login credentials with an optional context (e.g. IP address).
    pub async fn verify_login_with_context(
        &self,
        email: &str,
        password: &str,
        ip: Option<&str>,
    ) -> PasswordAuthResult<User> {
        let normalized_email = email.trim().to_lowercase();
        let rate_limit_key = match ip {
            Some(ip) => format!("login:{}:{}", normalized_email, ip),
            None => format!("login:{}", normalized_email),
        };

        // Check rate limit using the injected backend
        let rate_limit_config = RateLimitConfig::new(
            self.config.rate_limit_max_attempts as u64,
            Duration::from_secs(self.config.rate_limit_window_seconds),
        );
        let rate_result = self
            .rate_limiter
            .check_and_increment(&rate_limit_key, &rate_limit_config)
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("Rate limit error: {}", e)))?;

        if rate_result.is_denied() {
            return Err(PasswordAuthError::RateLimited {
                retry_after_seconds: rate_result.retry_after_secs(),
            }
            .into());
        }

        let user = match self
            .repository
            .find_user_by_email(&normalized_email)
            .await?
        {
            Some(u) => u,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        if user.status == UserStatus::Suspended {
            return Err(AuthError::AccountSuspended.into());
        }
        if user.status == UserStatus::Deleted {
            return Err(AuthError::AccountDeleted.into());
        }

        if let Some(retry_after_seconds) = self
            .repository
            .get_lockout_remaining_seconds(user.id)
            .await?
        {
            return Err(PasswordAuthError::AccountLocked {
                retry_after_seconds,
            }
            .into());
        }

        let credential = match self.repository.find_password_credential(user.id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound),
        };

        let password_hash = match &credential.metadata {
            CredentialMetadata::Password { .. } => &credential.secret_encrypted,
            _ => {
                return Err(
                    PasswordAuthError::Internal("Invalid credential type".to_string()).into(),
                );
            }
        };

        let is_valid = self
            .verifier
            .verify_password(password.as_bytes(), password_hash)?;

        if is_valid {
            self.repository.reset_failed_logins(user.id).await?;
            Ok(user)
        } else {
            let attempt = self
                .repository
                .record_failed_login(
                    user.id,
                    self.config.max_failed_attempts,
                    self.config.lockout_duration_seconds,
                )
                .await?;

            if let Some(retry_after_seconds) = attempt.lockout_remaining_seconds {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds,
                }
                .into())
            } else if attempt.count >= self.config.max_failed_attempts {
                Err(PasswordAuthError::AccountLocked {
                    retry_after_seconds: self.config.lockout_duration_seconds,
                }
                .into())
            } else {
                Err(PasswordAuthError::WrongPassword)
            }
        }
    }

    /// Change a user's password.
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        let analysis = self.analyzer.analyze(new_password);

        if self.is_compromised_password(new_password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound.into()),
        };

        let current_hash = &credential.secret_encrypted;
        let is_valid = self
            .verifier
            .verify_password(current_password.as_bytes(), current_hash)?;

        if !is_valid {
            return Err(PasswordAuthError::WrongPassword);
        }

        if self
            .verifier
            .verify_password(new_password.as_bytes(), current_hash)?
        {
            return Err(PasswordAuthError::PasswordSameAsCurrent.into());
        }

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }

    /// Reset a user's password (admin/internal use).
    pub async fn reset_password(
        &self,
        user_id: Uuid,
        new_password: &str,
    ) -> PasswordAuthResult<()> {
        let analysis = self.analyzer.analyze(new_password);

        if self.is_compromised_password(new_password).await? {
            return Err(PasswordAuthError::PasswordCompromised.into());
        }

        if analysis.strength < crate::strength::PasswordStrength::Fair {
            return Err(PasswordAuthError::PasswordTooWeak(analysis.feedback.join(". ")).into());
        }

        let credential = match self.repository.find_password_credential(user_id).await? {
            Some(c) => c,
            None => return Err(PasswordAuthError::CredentialNotFound.into()),
        };

        let new_hash = self.hasher.hash_password(new_password.as_bytes())?;

        self.repository
            .update_password_credential(credential.id, &new_hash)
            .await?;

        Ok(())
    }
}

#[cfg(feature = "hibp")]
async fn hibp_k_anonymity_check(
    password: &str,
    api_base_url: &str,
    user_agent: &str,
) -> PasswordAuthResult<bool> {
    use reqwest::header::HeaderValue;
    use sha1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();

    let mut hex = String::with_capacity(digest.len() * 2);
    for b in digest {
        hex.push_str(&format!("{:02X}", b));
    }

    let (prefix, suffix) = hex.split_at(5);

    let base = api_base_url.trim_end_matches('/');
    let url = format!("{}/range/{}", base, prefix);

    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|e| PasswordAuthError::Internal(format!("failed to build HIBP client: {}", e)))?;

    let resp = client
        .get(url)
        // Request padding to reduce information disclosure.
        .header("Add-Padding", HeaderValue::from_static("true"))
        .send()
        .await
        .map_err(|e| PasswordAuthError::Internal(format!("HIBP request failed: {}", e)))?;

    if !resp.status().is_success() {
        return Err(PasswordAuthError::Internal(format!(
            "HIBP returned {}",
            resp.status()
        )));
    }

    let body = resp
        .text()
        .await
        .map_err(|e| PasswordAuthError::Internal(format!("HIBP response read failed: {}", e)))?;

    Ok(hibp_range_body_contains_suffix(&body, suffix))
}

#[cfg(feature = "hibp")]
fn hibp_range_body_contains_suffix(body: &str, suffix: &str) -> bool {
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((candidate_suffix, _count)) = line.split_once(':') else {
            continue;
        };

        if candidate_suffix.trim().eq_ignore_ascii_case(suffix) {
            return true;
        }
    }
    false
}

#[cfg(all(test, feature = "hibp"))]
mod hibp_tests {
    use super::{hibp_k_anonymity_check, hibp_range_body_contains_suffix};

    #[test]
    fn hibp_range_body_parser_matches_suffix_case_insensitive() {
        let body = "ABCDEF1234567890:2\nFEDCBA9876543210:10\n";
        assert!(hibp_range_body_contains_suffix(body, "abcdef1234567890"));
        assert!(hibp_range_body_contains_suffix(body, "ABCDEF1234567890"));
        assert!(!hibp_range_body_contains_suffix(body, "00000"));
    }

    #[test]
    fn hibp_range_body_parser_ignores_bad_lines() {
        let body = "\n\nNOT_A_MATCH\nAAAAA:1\nBBBBB : 2\n";
        assert!(hibp_range_body_contains_suffix(body, "AAAAA"));
        assert!(hibp_range_body_contains_suffix(body, "BBBBB"));
    }

    #[tokio::test]
    async fn hibp_k_anonymity_check_can_run_against_local_server() {
        use sha1::{Digest, Sha1};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let password = "password123";

        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let digest = hasher.finalize();

        let mut hex = String::with_capacity(digest.len() * 2);
        for b in digest {
            hex.push_str(&format!("{:02X}", b));
        }

        let (prefix, suffix) = hex.split_at(5);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let base = format!("http://{}", addr);

        // Serve exactly one request.
        let expected_path = format!("/range/{prefix}");
        let suffix_line = format!("{suffix}:42\n");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0_u8; 4096];
            let n = socket.read(&mut buf).await.unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);

            assert!(req.starts_with("GET "));
            assert!(req.contains(&expected_path));

            let body = suffix_line;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            socket.write_all(resp.as_bytes()).await.unwrap();
        });

        let ok = hibp_k_anonymity_check(password, &base, "underlay-test")
            .await
            .unwrap();

        assert!(ok);

        server.await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::Argon2Hasher;
    use std::collections::{HashMap, HashSet};
    use tokio::sync::Mutex;
    use underlay_ratelimit::InMemoryBackend;

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
            display_name: "Test".to_string(),
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
}
