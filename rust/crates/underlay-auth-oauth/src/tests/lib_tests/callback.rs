use super::super::{
    AuthError, CredentialRepository, CredentialType, GoogleOAuthAppService, NO_GOOGLE_REFRESH_TOKEN,
};
use super::support::{
    basic_token_set, callback_request, login_state, token_set_with_refresh, unverified_userinfo,
    verified_userinfo, MemoryRepo, StubProvider,
};

#[tokio::test]
async fn handle_google_callback_creates_user_and_credential() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: token_set_with_refresh(),
        userinfo: super::super::GoogleUserInfo {
            picture: Some("https://example.com/p.png".to_string()),
            ..verified_userinfo()
        },
    };

    let svc = GoogleOAuthAppService::new(provider);

    let result = svc
        .handle_google_callback(
            &repo,
            callback_request("state-1"),
            login_state("state-1"),
            |secret| Ok(format!("enc:{secret}")),
        )
        .await
        .unwrap();

    assert!(result.is_new_user);
    assert_eq!(result.user.email, "claire@example.com");
    assert_eq!(result.user.display_name, Some("Claire".to_string()));
    assert_eq!(
        result.credential.credential_type,
        CredentialType::OAuthGoogle
    );
    assert!(result.credential.secret_encrypted.contains("enc:"));
}

#[tokio::test]
async fn handle_google_callback_rejects_state_mismatch() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: basic_token_set(),
        userinfo: verified_userinfo(),
    };

    let svc = GoogleOAuthAppService::new(provider);

    let err = svc
        .handle_google_callback(
            &repo,
            callback_request("state-bad"),
            login_state("state-expected"),
            |_secret| Ok("enc".to_string()),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, AuthError::BadRequest(_)));
}

#[tokio::test]
async fn handle_google_callback_requires_verified_email() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: basic_token_set(),
        userinfo: unverified_userinfo(),
    };

    let svc = GoogleOAuthAppService::new(provider).with_require_verified_email(true);

    let err = svc
        .handle_google_callback(
            &repo,
            callback_request("state-1"),
            login_state("state-1"),
            |_secret| Ok("enc".to_string()),
        )
        .await
        .unwrap_err();

    assert!(matches!(err, AuthError::OAuthError(_)));
}

#[tokio::test]
async fn callback_rejects_a_different_google_subject_for_the_same_email() {
    let repo = MemoryRepo::new();
    let mut provider = StubProvider {
        token_set: token_set_with_refresh(),
        userinfo: verified_userinfo(),
    };
    let linked = GoogleOAuthAppService::new(provider.clone())
        .handle_google_callback(
            &repo,
            callback_request("state"),
            login_state("state"),
            |token| Ok(format!("enc:{token}")),
        )
        .await
        .unwrap();

    provider.userinfo.sub = "different-google-subject".into();
    let result = GoogleOAuthAppService::new(provider)
        .handle_google_callback(
            &repo,
            callback_request("state"),
            login_state("state"),
            |_| panic!("refresh token must not be encrypted for a mismatched subject"),
        )
        .await;

    assert!(matches!(result, Err(AuthError::OAuthError(_))));
    assert_eq!(
        repo.find_oauth_by_user_and_provider(linked.user.id, CredentialType::OAuthGoogle)
            .await
            .unwrap(),
        Some(linked.credential)
    );
}

#[tokio::test]
async fn callback_only_persists_encrypted_refresh_tokens_when_issued() {
    let repo = MemoryRepo::new();
    let mut provider = StubProvider {
        token_set: basic_token_set(),
        userinfo: verified_userinfo(),
    };
    let linked = GoogleOAuthAppService::new(provider.clone())
        .handle_google_callback(
            &repo,
            callback_request("state"),
            login_state("state"),
            |_| panic!("no refresh token to encrypt"),
        )
        .await
        .unwrap();
    assert_eq!(linked.credential.secret_encrypted, NO_GOOGLE_REFRESH_TOKEN);

    provider.token_set.refresh_token = Some("new-refresh".into());
    let updated = GoogleOAuthAppService::new(provider.clone())
        .handle_google_callback(
            &repo,
            callback_request("state"),
            login_state("state"),
            |token| Ok(format!("enc:{token}")),
        )
        .await
        .unwrap();
    assert_eq!(linked.credential.id, updated.credential.id);
    assert_eq!(updated.credential.secret_encrypted, "enc:new-refresh");

    for omitted in [None, Some(String::new())] {
        provider.token_set.refresh_token = omitted;
        let result = GoogleOAuthAppService::new(provider.clone())
            .handle_google_callback(
                &repo,
                callback_request("state"),
                login_state("state"),
                |_| panic!("no refresh token to encrypt"),
            )
            .await
            .unwrap();
        assert_eq!(result.credential, updated.credential);
    }
}
