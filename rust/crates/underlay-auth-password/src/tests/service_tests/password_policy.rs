use std::sync::Arc;

use super::super::{CompromisedPasswordStrategy, PasswordAuthError, PasswordConfig};
use super::support::{make_user, service, MemoryRepo};

#[tokio::test]
async fn compromised_password_rejected_when_enabled() {
    let user = make_user("d@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;

    let service = service(
        repo,
        Some(
            PasswordConfig::default()
                .with_check_compromised(true)
                .with_compromised_password_strategy(CompromisedPasswordStrategy::LocalBlocklist),
        ),
    );

    let err = service
        .set_password(user.id, "password123")
        .await
        .unwrap_err();
    assert!(matches!(err, PasswordAuthError::PasswordCompromised));
}
