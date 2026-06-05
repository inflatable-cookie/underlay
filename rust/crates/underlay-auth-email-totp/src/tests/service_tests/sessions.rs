use super::super::{EmailTotpConfig, EmailTotpService};
use super::support::{
    default_rate_limit, session, MockCodeRepo, MockCodeRepoState, MockSender, MockSessionRepo,
    MockSessionRepoState,
};

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
