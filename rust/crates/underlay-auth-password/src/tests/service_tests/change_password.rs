use std::sync::Arc;

use super::super::{PasswordAuthError, PasswordConfig};
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
        .change_password(user.id, "WrongPassword1!", "weak")
        .await
        .unwrap_err();

    assert!(matches!(err, PasswordAuthError::WrongPassword));
}

#[tokio::test]
async fn change_password_rate_limits_attempts_per_user() {
    let user = make_user("limited@example.com");
    let other_user = make_user("other@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    let service = service(
        repo,
        Some(PasswordConfig::default().with_rate_limit_max_attempts(1)),
    );
    let current_password = "CorrectHorse1!";
    for user_id in [user.id, other_user.id] {
        service
            .set_password(user_id, current_password)
            .await
            .unwrap();
    }
    assert!(matches!(
        service.change_password(user.id, "wrong", "weak").await,
        Err(PasswordAuthError::WrongPassword)
    ));
    assert!(matches!(
        service.change_password(user.id, current_password, "NewPassword2#").await,
        Err(PasswordAuthError::RateLimited { retry_after_seconds }) if retry_after_seconds > 0
    ));
    service
        .change_password(other_user.id, current_password, "NewPassword2#")
        .await
        .unwrap();
}

#[cfg(feature = "hibp")]
#[tokio::test]
async fn change_password_verifies_current_password_before_hibp() {
    use super::super::{CompromisedPasswordStrategy, PasswordAuthRepository};
    use underlay_auth::hashing::{Argon2Hasher, PasswordHasherExt};

    let user = make_user("hibp-order@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    let hash = Argon2Hasher::with_params(32, 1, 1)
        .hash_password(b"CorrectHorse1!")
        .unwrap();
    repo.create_password_credential(user.id, &hash)
        .await
        .unwrap();

    // An invalid HIBP endpoint proves when the service enters the HIBP policy.
    let config = PasswordConfig::default()
        .with_check_compromised(true)
        .with_compromised_password_strategy(CompromisedPasswordStrategy::HibpKAnonymity {
            api_base_url: "http://example.com".into(),
            user_agent: "underlay-test".into(),
        });
    let service = service(repo, Some(config));
    assert!(matches!(
        service
            .change_password(user.id, "wrong", "NewPassword2#")
            .await,
        Err(PasswordAuthError::WrongPassword)
    ));
    assert!(matches!(
        service.change_password(user.id, "CorrectHorse1!", "NewPassword2#").await,
        Err(PasswordAuthError::Internal(message)) if message.contains("HIBP")
    ));
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
