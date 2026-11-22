use std::sync::Arc;

use super::super::{PasswordAuthError, PasswordAuthRepository, PasswordConfig};
use super::support::{make_user, service, MemoryRepo};

#[tokio::test]
async fn reset_password_updates_hash_and_allows_login() {
    let user = make_user("h@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(repo, None);

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
async fn reset_password_clears_failed_logins_and_lockout() {
    let user = make_user("locked-reset@example.com");
    let repo = Arc::new(MemoryRepo::new(2));
    repo.insert_user(user.clone()).await;

    let service = service(
        Arc::clone(&repo),
        Some(
            PasswordConfig::default()
                .with_max_failed_attempts(2)
                .with_lockout_duration_seconds(900),
        ),
    );

    service
        .set_password(user.id, "CorrectHorse1!")
        .await
        .unwrap();

    assert!(matches!(
        service.verify_login(&user.email, "wrong-1").await,
        Err(PasswordAuthError::WrongPassword)
    ));
    assert!(matches!(
        service.verify_login(&user.email, "wrong-2").await,
        Err(PasswordAuthError::AccountLocked { .. })
    ));
    assert_eq!(repo.get_failed_login_count(user.id).await.unwrap(), 2);
    assert!(repo
        .get_lockout_remaining_seconds(user.id)
        .await
        .unwrap()
        .is_some());

    service
        .reset_password(user.id, "ResetPassword2#")
        .await
        .unwrap();

    assert_eq!(repo.get_failed_login_count(user.id).await.unwrap(), 0);
    assert!(repo
        .get_lockout_remaining_seconds(user.id)
        .await
        .unwrap()
        .is_none());
    assert!(service
        .verify_login(&user.email, "ResetPassword2#")
        .await
        .is_ok());
}
