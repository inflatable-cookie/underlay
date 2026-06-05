use chrono::{Duration, Utc};

use super::super::{EmailTotpConfig, EmailTotpError, EmailTotpService};
use super::support::{
    default_rate_limit, hash, session, MockCodeRepo, MockCodeRepoState, MockSender,
    MockSessionRepo, MockSessionRepoState,
};
use crate::repository::StoredCode;

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
