use chrono::{DateTime, Duration, Utc};
use std::sync::Mutex;
use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt};

use super::super::{EmailTotpSender, VERIFICATION_METHOD_EMAIL_TOTP};
use crate::error::{EmailTotpError, EmailTotpResult};
use crate::repository::{
    EmailTotpCodeRepository, RateLimitStatus, StoredCode, VerificationSession,
    VerificationSessionRepository,
};

#[derive(Debug)]
pub(crate) struct MockCodeRepoState {
    pub(crate) rate_limit: RateLimitStatus,
    pub(crate) active_code: Option<StoredCode>,
    pub(crate) increment_attempts_result: i32,
    pub(crate) stored: Vec<(String, String, String, DateTime<Utc>, i32)>,
    pub(crate) increment_send_calls: usize,
    pub(crate) increment_attempt_calls: Vec<String>,
    pub(crate) marked_used: Vec<String>,
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

pub(crate) struct MockCodeRepo {
    pub(crate) state: Mutex<MockCodeRepoState>,
}

impl MockCodeRepo {
    pub(crate) fn new(state: MockCodeRepoState) -> Self {
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
pub(crate) struct MockSessionRepoState {
    pub(crate) session_to_create: Option<VerificationSession>,
    pub(crate) consumed: Option<VerificationSession>,
    pub(crate) fetched: Option<VerificationSession>,
    pub(crate) create_calls: usize,
}

pub(crate) struct MockSessionRepo {
    pub(crate) state: Mutex<MockSessionRepoState>,
}

impl MockSessionRepo {
    pub(crate) fn new(state: MockSessionRepoState) -> Self {
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
        self.state
            .lock()
            .expect("lock")
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
        Ok(self.state.lock().expect("lock").fetched.clone())
    }
}

#[derive(Debug, Default)]
pub(crate) struct MockSenderState {
    pub(crate) sent: Vec<(String, String, String, i32)>,
}

pub(crate) struct MockSender {
    pub(crate) state: Mutex<MockSenderState>,
}

impl MockSender {
    pub(crate) fn new() -> Self {
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

pub(crate) fn hash(code: &str) -> String {
    Argon2Hasher::default()
        .hash_password(code.as_bytes())
        .expect("hashing should succeed")
}

pub(crate) fn session(id: &str, user_id: &str, purpose: &str) -> VerificationSession {
    VerificationSession {
        id: id.to_string(),
        user_id: user_id.to_string(),
        purpose: purpose.to_string(),
        method: VERIFICATION_METHOD_EMAIL_TOTP.to_string(),
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::minutes(10),
    }
}

pub(crate) fn default_rate_limit() -> RateLimitStatus {
    RateLimitStatus {
        send_count: 0,
        attempt_count: 0,
        is_limited: false,
    }
}
