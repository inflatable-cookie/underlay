//! OAuth2 primitives for Underlay-based apps.
//!
//! This crate provides an app-agnostic boundary around OAuth2 flows (Google first):
//! - builds authorization URLs (state + PKCE)
//! - exchanges authorization codes for tokens
//! - fetches user info from the provider
//! - refreshes tokens
//!
//! Apps remain responsible for:
//! - persisting state/PKCE verifier between start/callback steps (server-side)
//! - storing refresh tokens and linking to users/credentials
//! - user creation/linking policy and sessions

mod app_service;
mod config;
mod error;
mod google;
mod model;
mod provider;
mod token_cipher;

pub use app_service::GoogleOAuthAppService;
pub use config::GoogleOAuthConfig;
pub use error::OAuthServiceError;
pub use google::GoogleOAuthService;
pub use model::{
    GoogleUserInfo, OAuthCallbackRequest, OAuthLoginResult, OAuthLoginState, OAuthStart, TokenSet,
};
pub use provider::OAuthProvider;
pub use token_cipher::{OAuthTokenCipher, AUTH_OAUTH_SECRET_KEY_ENV};

pub(crate) use underlay_auth::{
    AuthError, AuthResult, Credential, CredentialMetadata, CredentialRepository, CredentialType,
    User, UserRepository,
};
#[cfg(test)]
pub(crate) use underlay_core::Uuid;

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
