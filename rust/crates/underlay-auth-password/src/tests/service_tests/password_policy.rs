use std::sync::Arc;

use super::super::{CompromisedPasswordStrategy, PasswordAuthError, PasswordConfig};
use super::support::{make_user, service, MemoryRepo};
use crate::strength::{PasswordStrength, PasswordStrengthAnalyzer};

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

#[tokio::test]
async fn set_password_requires_good_strength() {
    let user = make_user("policy@example.com");
    let repo = Arc::new(MemoryRepo::new(10));
    repo.insert_user(user.clone()).await;
    let service = service(repo, None);

    let fair_password = "fkhxptwe";
    let analyzer = PasswordStrengthAnalyzer::new();
    assert_eq!(
        analyzer.analyze(fair_password).strength,
        PasswordStrength::Fair
    );
    assert!(matches!(
        service.set_password(user.id, fair_password).await,
        Err(PasswordAuthError::PasswordTooWeak(_))
    ));

    let strong_password = "Z3bra!Lamp#Rain7";
    assert!(analyzer.analyze(strong_password).strength >= PasswordStrength::Good);
    assert!(service.set_password(user.id, strong_password).await.is_ok());
}
