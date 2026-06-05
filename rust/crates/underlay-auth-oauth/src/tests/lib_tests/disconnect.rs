use underlay_auth::CredentialRepository;

use super::super::{CredentialType, GoogleOAuthAppService};
use super::support::{
    callback_request, login_state, token_set_with_refresh, verified_userinfo, MemoryRepo,
    StubProvider,
};

#[tokio::test]
async fn disconnect_google_removes_credential() {
    let repo = MemoryRepo::new();

    let provider = StubProvider {
        token_set: super::super::TokenSet {
            id_token: None,
            expires_in_seconds: None,
            scope: None,
            token_type: None,
            ..token_set_with_refresh()
        },
        userinfo: verified_userinfo(),
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

    assert!(repo
        .find_oauth_by_user_and_provider(result.user.id, CredentialType::OAuthGoogle)
        .await
        .unwrap()
        .is_some());

    svc.disconnect_google(&repo, result.user.id).await.unwrap();

    assert!(repo
        .find_oauth_by_user_and_provider(result.user.id, CredentialType::OAuthGoogle)
        .await
        .unwrap()
        .is_none());
}
