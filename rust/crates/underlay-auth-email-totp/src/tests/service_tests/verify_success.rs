use chrono::{Duration, Utc};

use super::super::{EmailTotpConfig, EmailTotpService, VERIFICATION_METHOD_EMAIL_TOTP};
use super::support::{
    default_rate_limit, hash, session, MockCodeRepo, MockCodeRepoState, MockSender,
    MockSessionRepo, MockSessionRepoState,
};
use crate::repository::StoredCode;

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
