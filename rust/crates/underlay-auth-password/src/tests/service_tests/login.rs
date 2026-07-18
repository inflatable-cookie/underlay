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
        Some(PasswordConfig::default().with_max_failed_attempts(10)),
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

#[tokio::test]
async fn unknown_email_miss_costs_a_kdf_pass() {
    // g08.010: the unknown-email path must run one KDF pass (dummy_verify) so
    // login timing is not an account-existence oracle. Compare against the
    // wrong-password path, which runs a real Argon2 verify. Without the
    // equalizer the miss path returns in microseconds — orders of magnitude
    // faster — so the generous 4x margin below cannot flake.
    let user = make_user("timing@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(repo, None);
    service
        .set_password(user.id, "S0mething$trong!")
        .await
        .unwrap();

    // Warm-up pass (allocator / first-touch effects on both paths).
    let _ = service.verify_login("nobody@example.com", "pw").await;
    let _ = service.verify_login(&user.email, "wrong-password").await;

    let start = std::time::Instant::now();
    let err = service
        .verify_login("nobody@example.com", "pw")
        .await
        .unwrap_err();
    let miss = start.elapsed();
    assert!(matches!(err, PasswordAuthError::CredentialNotFound));

    let start = std::time::Instant::now();
    let err = service
        .verify_login(&user.email, "wrong-password")
        .await
        .unwrap_err();
    let real_verify = start.elapsed();
    assert!(matches!(err, PasswordAuthError::WrongPassword));

    assert!(
        miss * 4 >= real_verify,
        "unknown-email miss ({miss:?}) must cost ~one KDF pass like a real \
         wrong-password verify ({real_verify:?}) — timing oracle regression"
    );
}
