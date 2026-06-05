use std::sync::Arc;

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
