use chrono::{Duration, Utc};

use super::super::{EmailTotpConfig, EmailTotpError, EmailTotpService};
use super::support::{
    default_rate_limit, MockCodeRepo, MockCodeRepoState, MockSender, MockSessionRepo,
    MockSessionRepoState,
};
use crate::repository::RateLimitStatus;

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
