use std::sync::Arc;

use super::super::{PasswordAuthError, PasswordConfig};
use super::support::{make_user, service, MemoryRepo};

#[tokio::test]
async fn lockout_triggers_after_n_failures() {
    let user = make_user("b@example.com");
    let repo = Arc::new(MemoryRepo::new(2));
    repo.insert_user(user.clone()).await;

    let service = service(
        repo,
        Some(
            PasswordConfig::default()
                .with_max_failed_attempts(2)
                .with_lockout_duration_seconds(900),
        ),
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

    let service = service(
        repo,
        Some(
            PasswordConfig::default()
                .with_rate_limit_max_attempts(1)
                .with_rate_limit_window_seconds(3600),
        ),
    );

    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    let ip = "1.2.3.4";

    let ok = service
        .verify_login_with_context(&user.email, "S0mething$trong!", Some(ip))
        .await
        .unwrap();
    assert_eq!(ok.id, user.id);

    let err = service
        .verify_login_with_context(&user.email, "S0mething$trong!", Some(ip))
        .await
        .unwrap_err();
    assert!(matches!(err, PasswordAuthError::RateLimited { .. }));
}
