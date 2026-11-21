#[cfg(test)]
mod tests {
    use axum::extract::FromRequestParts;
    use axum::http::{header, Request, StatusCode};
    use axum::response::IntoResponse;

    use crate::extractors::extract_bearer;
    use crate::{
        AuthError, AuthProvider, AuthResult, Authenticated, HasAuthProvider, OptionalAuthenticated,
        Principal,
    };

    #[test]
    fn bearer_extraction_requires_bearer_prefix() {
        let value = http::HeaderValue::from_static("Token abc");
        assert!(extract_bearer(Some(&value)).unwrap().is_none());
    }

    #[test]
    fn bearer_extraction_trims_token() {
        let value = http::HeaderValue::from_static("Bearer   abc123  ");
        assert_eq!(
            extract_bearer(Some(&value)).unwrap().as_deref(),
            Some("abc123")
        );
    }

    #[test]
    fn bearer_extraction_fails_on_empty_token() {
        let value = http::HeaderValue::from_static("Bearer   ");
        assert!(matches!(
            extract_bearer(Some(&value)),
            Err(AuthError::BadRequest(_))
        ));
    }

    struct RateLimitedProvider;

    #[async_trait::async_trait]
    impl AuthProvider for RateLimitedProvider {
        async fn authenticate_bearer(&self, _bearer_token: &str) -> AuthResult<Principal> {
            Err(AuthError::RateLimited {
                retry_after_seconds: 42,
            })
        }
    }

    impl HasAuthProvider for RateLimitedProvider {
        fn auth_provider(&self) -> &dyn AuthProvider {
            self
        }
    }

    #[tokio::test]
    async fn optional_authentication_rejects_an_empty_bearer_header() {
        let (mut parts, ()) = Request::builder()
            .header(header::AUTHORIZATION, "Bearer   ")
            .body(())
            .unwrap()
            .into_parts();
        let rejection = OptionalAuthenticated::from_request_parts(&mut parts, &RateLimitedProvider)
            .await
            .unwrap_err();
        assert_eq!(rejection.into_response().status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn both_extractors_return_retry_after_and_details_on_rate_limit() {
        let (mut parts, ()) = Request::builder()
            .header(header::AUTHORIZATION, "Bearer token")
            .body(())
            .unwrap()
            .into_parts();
        let required = Authenticated::from_request_parts(&mut parts, &RateLimitedProvider)
            .await
            .unwrap_err();
        let optional = OptionalAuthenticated::from_request_parts(&mut parts, &RateLimitedProvider)
            .await
            .unwrap_err();

        for rejection in [required, optional] {
            let response = rejection.into_response();
            assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
            assert_eq!(response.headers()[header::RETRY_AFTER], "42");

            let body = axum::body::to_bytes(response.into_body(), 1024)
                .await
                .unwrap();
            let envelope: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(
                envelope["error"]["fieldErrors"]["retry_after_seconds"],
                "42"
            );
        }
    }
}
