use super::*;
use crate::repository::{RateLimitStatus, StoredCode, VerificationSession};
use chrono::{DateTime, Duration};
use std::sync::Mutex;
use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt};

#[derive(Debug)]
struct MockCodeRepoState {
    rate_limit: RateLimitStatus,
    active_code: Option<StoredCode>,
    increment_attempts_result: i32,
    stored: Vec<(String, String, String, DateTime<Utc>, i32)>,
    increment_send_calls: usize,
    increment_attempt_calls: Vec<String>,
    marked_used: Vec<String>,
}

impl Default for MockCodeRepoState {
    fn default() -> Self {
        Self {
            rate_limit: RateLimitStatus {
                send_count: 0,
                attempt_count: 0,
                is_limited: false,
            },
            active_code: None,
            increment_attempts_result: 0,
            stored: Vec::new(),
            increment_send_calls: 0,
            increment_attempt_calls: Vec::new(),
            marked_used: Vec::new(),
        }
    }
}

struct MockCodeRepo {
    state: Mutex<MockCodeRepoState>,
}

impl MockCodeRepo {
    fn new(state: MockCodeRepoState) -> Self {
        Self {
            state: Mutex::new(state),
        }
    }
}

#[async_trait::async_trait]
impl EmailTotpCodeRepository for MockCodeRepo {
    async fn check_rate_limit(
        &self,
        _user_id: &str,
        _purpose: &str,
        _max_per_hour: i32,
    ) -> EmailTotpResult<RateLimitStatus> {
        Ok(self.state.lock().expect("lock").rate_limit.clone())
    }

    async fn increment_send_count(&self, _user_id: &str, _purpose: &str) -> EmailTotpResult<()> {
        self.state.lock().expect("lock").increment_send_calls += 1;
        Ok(())
    }

    async fn store_code(
        &self,
        user_id: &str,
        email: &str,
        code_hash: &str,
        _purpose: &str,
        expires_at: DateTime<Utc>,
        max_attempts: i32,
    ) -> EmailTotpResult<String> {
        self.state.lock().expect("lock").stored.push((
            user_id.to_string(),
            email.to_string(),
            code_hash.to_string(),
            expires_at,
            max_attempts,
        ));
        Ok("stored-code-id".to_string())
    }

    async fn get_active_code(
        &self,
        _user_id: &str,
        _purpose: &str,
    ) -> EmailTotpResult<Option<StoredCode>> {
        Ok(self.state.lock().expect("lock").active_code.clone())
    }

    async fn increment_attempts(&self, code_id: &str) -> EmailTotpResult<i32> {
        let mut state = self.state.lock().expect("lock");
        state.increment_attempt_calls.push(code_id.to_string());
        Ok(state.increment_attempts_result)
    }

    async fn mark_code_used(&self, code_id: &str) -> EmailTotpResult<()> {
        self.state
            .lock()
            .expect("lock")
            .marked_used
            .push(code_id.to_string());
        Ok(())
    }
}

#[derive(Debug, Default)]
struct MockSessionRepoState {
    session_to_create: Option<VerificationSession>,
    consumed: Option<VerificationSession>,
    fetched: Option<VerificationSession>,
    create_calls: usize,
    consume_calls: usize,
    get_calls: usize,
}

struct MockSessionRepo {
    state: Mutex<MockSessionRepoState>,
}

impl MockSessionRepo {
    fn new(state: MockSessionRepoState) -> Self {
        Self {
            state: Mutex::new(state),
        }
    }
}

#[async_trait::async_trait]
impl VerificationSessionRepository for MockSessionRepo {
    async fn create_session(
        &self,
        _user_id: &str,
        _purpose: &str,
        _method: &str,
        _expires_at: DateTime<Utc>,
    ) -> EmailTotpResult<VerificationSession> {
        let mut state = self.state.lock().expect("lock");
        state.create_calls += 1;
        state
            .session_to_create
            .clone()
            .ok_or(EmailTotpError::Storage("missing session".to_string()))
    }

    async fn consume_session(
        &self,
        _session_id: &str,
        _user_id: &str,
        _purpose: &str,
    ) -> EmailTotpResult<VerificationSession> {
        let mut state = self.state.lock().expect("lock");
        state.consume_calls += 1;
        state
            .consumed
            .clone()
            .ok_or(EmailTotpError::SessionNotFound)
    }

    async fn get_session(
        &self,
        _session_id: &str,
        _user_id: &str,
        _purpose: &str,
    ) -> EmailTotpResult<Option<VerificationSession>> {
        let mut state = self.state.lock().expect("lock");
        state.get_calls += 1;
        Ok(state.fetched.clone())
    }
}

#[derive(Debug, Default)]
struct MockSenderState {
    sent: Vec<(String, String, String, i32)>,
}

struct MockSender {
    state: Mutex<MockSenderState>,
}

impl MockSender {
    fn new() -> Self {
        Self {
            state: Mutex::new(MockSenderState::default()),
        }
    }
}

#[async_trait::async_trait]
impl EmailTotpSender for MockSender {
    async fn send_code(
        &self,
        to_email: &str,
        code: &str,
        purpose: &str,
        expiry_minutes: i32,
    ) -> EmailTotpResult<()> {
        self.state.lock().expect("lock").sent.push((
            to_email.to_string(),
            code.to_string(),
            purpose.to_string(),
            expiry_minutes,
        ));
        Ok(())
    }
}

fn hash(code: &str) -> String {
    Argon2Hasher::default()
        .hash_password(code.as_bytes())
        .expect("hashing should succeed")
}

fn session(id: &str, user_id: &str, purpose: &str) -> VerificationSession {
    VerificationSession {
        id: id.to_string(),
        user_id: user_id.to_string(),
        purpose: purpose.to_string(),
        method: VERIFICATION_METHOD_EMAIL_TOTP.to_string(),
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::minutes(10),
    }
}

fn default_rate_limit() -> RateLimitStatus {
    RateLimitStatus {
        send_count: 0,
        attempt_count: 0,
        is_limited: false,
    }
}

#[tokio::test]
async fn request_code_returns_rate_limited_when_limit_hit() {
    let code_repo = MockCodeRepo::new(MockCodeRepoState {
        rate_limit: RateLimitStatus {
            send_count: 10,
            attempt_count: 10,
            is_limited: true,
        },
        ..Default::default()
    });
    let service = EmailTotpService::new(
        code_repo,
        MockSessionRepo::new(MockSessionRepoState::default()),
        MockSender::new(),
        EmailTotpConfig::default(),
    );

    let err = service
        .request_code("user-1", "u@example.com", "login")
        .await
        .expect_err("rate limited requests should fail");
    assert!(matches!(err, EmailTotpError::RateLimited));
}

#[tokio::test]
async fn request_code_stores_hash_and_sends_email() {
    let code_repo = MockCodeRepo::new(MockCodeRepoState {
        rate_limit: default_rate_limit(),
        ..Default::default()
    });
    let sender = MockSender::new();
    let service = EmailTotpService::new(
        code_repo,
        MockSessionRepo::new(MockSessionRepoState::default()),
        sender,
        EmailTotpConfig::default(),
    );

    let expires_at = service
        .request_code("user-1", "u@example.com", "login")
        .await
        .expect("request should succeed");

    assert!(expires_at > Utc::now() - Duration::seconds(1));
    let state = service.code_repository.state.lock().expect("lock");
    assert_eq!(state.stored.len(), 1);
    assert_eq!(state.increment_send_calls, 1);
    assert_eq!(state.stored[0].0, "user-1");
    assert_eq!(state.stored[0].1, "u@example.com");
}

#[tokio::test]
async fn verify_code_returns_expected_error_conditions() {
    let base = MockCodeRepoState {
        rate_limit: default_rate_limit(),
        ..Default::default()
    };
    let sender = MockSender::new();
    let session_repo = MockSessionRepo::new(MockSessionRepoState {
        session_to_create: Some(session("s1", "user-1", "login")),
        ..Default::default()
    });

    let svc_missing = EmailTotpService::new(
        MockCodeRepo::new(base),
        session_repo,
        sender,
        EmailTotpConfig::default(),
    );
    let err = svc_missing
        .verify_code("user-1", "login", "123456")
        .await
        .expect_err("missing code should fail");
    assert!(matches!(err, EmailTotpError::NoActiveCode));

    let expired = StoredCode {
        id: "code1".to_string(),
        code_hash: hash("123456"),
        expires_at: Utc::now() - Duration::minutes(1),
        attempts: 0,
        max_attempts: 5,
    };
    let svc_expired = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            active_code: Some(expired),
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState::default()),
        MockSender::new(),
        EmailTotpConfig::default(),
    );
    let err = svc_expired
        .verify_code("user-1", "login", "123456")
        .await
        .expect_err("expired code should fail");
    assert!(matches!(err, EmailTotpError::CodeExpired));

    let exhausted = StoredCode {
        id: "code2".to_string(),
        code_hash: hash("123456"),
        expires_at: Utc::now() + Duration::minutes(5),
        attempts: 3,
        max_attempts: 3,
    };
    let svc_exhausted = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            active_code: Some(exhausted),
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState::default()),
        MockSender::new(),
        EmailTotpConfig::default(),
    );
    let err = svc_exhausted
        .verify_code("user-1", "login", "123456")
        .await
        .expect_err("exhausted code should fail");
    assert!(matches!(err, EmailTotpError::TooManyAttempts));
}

#[tokio::test]
async fn verify_code_invalid_increments_attempts() {
    let service = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            active_code: Some(StoredCode {
                id: "code1".to_string(),
                code_hash: hash("123456"),
                expires_at: Utc::now() + Duration::minutes(5),
                attempts: 0,
                max_attempts: 5,
            }),
            increment_attempts_result: 1,
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState::default()),
        MockSender::new(),
        EmailTotpConfig::default(),
    );

    let err = service
        .verify_code("user-1", "login", "000000")
        .await
        .expect_err("invalid code should fail");
    assert!(matches!(err, EmailTotpError::InvalidCode));
    let state = service.code_repository.state.lock().expect("lock");
    assert_eq!(state.increment_attempt_calls, vec!["code1".to_string()]);
}

#[tokio::test]
async fn verify_code_success_marks_used_and_creates_session() {
    let service = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            active_code: Some(StoredCode {
                id: "code1".to_string(),
                code_hash: hash("123456"),
                expires_at: Utc::now() + Duration::minutes(5),
                attempts: 0,
                max_attempts: 5,
            }),
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState {
            session_to_create: Some(session("session-1", "user-1", "login")),
            ..Default::default()
        }),
        MockSender::new(),
        EmailTotpConfig::default(),
    );

    let created = service
        .verify_code("user-1", "login", "123456")
        .await
        .expect("valid code should succeed");
    assert_eq!(created.id, "session-1");
    assert_eq!(created.method, VERIFICATION_METHOD_EMAIL_TOTP);

    let code_state = service.code_repository.state.lock().expect("lock");
    assert_eq!(code_state.marked_used, vec!["code1".to_string()]);
    let session_state = service.session_repository.state.lock().expect("lock");
    assert_eq!(session_state.create_calls, 1);
}

#[tokio::test]
async fn verify_code_only_success_marks_used_without_session_creation() {
    let service = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            active_code: Some(StoredCode {
                id: "code-only".to_string(),
                code_hash: hash("123456"),
                expires_at: Utc::now() + Duration::minutes(5),
                attempts: 0,
                max_attempts: 5,
            }),
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState::default()),
        MockSender::new(),
        EmailTotpConfig::default(),
    );

    service
        .verify_code_only("user-1", "login", " 123456 ")
        .await
        .expect("code-only verification should succeed");

    let code_state = service.code_repository.state.lock().expect("lock");
    assert_eq!(code_state.marked_used, vec!["code-only".to_string()]);
    let session_state = service.session_repository.state.lock().expect("lock");
    assert_eq!(session_state.create_calls, 0);
}

#[tokio::test]
async fn consume_and_get_session_delegate_to_repository() {
    let service = EmailTotpService::new(
        MockCodeRepo::new(MockCodeRepoState {
            rate_limit: default_rate_limit(),
            ..Default::default()
        }),
        MockSessionRepo::new(MockSessionRepoState {
            consumed: Some(session("s-consume", "user-1", "login")),
            fetched: Some(session("s-fetch", "user-1", "login")),
            ..Default::default()
        }),
        MockSender::new(),
        EmailTotpConfig::default(),
    );

    let consumed = service
        .consume_session("s-consume", "user-1", "login")
        .await
        .expect("consume should succeed");
    assert_eq!(consumed.id, "s-consume");

    let fetched = service
        .get_session("s-fetch", "user-1", "login")
        .await
        .expect("get should succeed")
        .expect("session should exist");
    assert_eq!(fetched.id, "s-fetch");
}
