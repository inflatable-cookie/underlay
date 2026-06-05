use std::sync::Arc;

use super::super::PasswordAuthError;
use super::support::{make_user, service, MemoryRepo};

#[tokio::test]
async fn change_password_rejects_wrong_current_password() {
    let user = make_user("e@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(repo, None);

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

    let service = service(repo, None);

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

    let service = service(repo, None);

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
