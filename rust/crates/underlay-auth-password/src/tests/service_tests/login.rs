use std::sync::Arc;

use super::super::{PasswordAuthError, PasswordAuthRepository, PasswordConfig};
use super::support::{make_user, service, MemoryRepo};

#[tokio::test]
async fn login_success_resets_failures() {
    let user = make_user("a@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(
        repo.clone(),
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
async fn verify_login_normalizes_email_for_lookup() {
    let user = make_user("i@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(repo, None);

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
