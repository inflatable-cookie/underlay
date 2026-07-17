#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use underlay_core::AppError;

    use crate::{error_response, ApiError, ErrorDetail};

    #[test]
    fn error_response_sets_x_error_code_header() {
        let err = AppError::new("resource.not_found", "Missing");
        let res = error_response(StatusCode::NOT_FOUND, err);
        let header = res
            .headers()
            .get("x-error-code")
            .expect("x-error-code should be set");
        assert_eq!(header.to_str().unwrap(), "resource.not_found");
    }

    #[test]
    fn api_error_sets_code_header_and_carries_detail_in_extensions_only() {
        let res = ApiError::internal("db.query_failed", "Query failed")
            .with_context(serde_json::json!({ "operation": "list_users" }))
            .into_response();

        let code = res
            .headers()
            .get("x-error-code")
            .expect("x-error-code should be set");
        assert_eq!(code.to_str().unwrap(), "db.query_failed");

        // Internal detail must never ship as response headers.
        assert!(res.headers().get("x-error-message").is_none());
        assert!(res.headers().get("x-error-context").is_none());

        let detail = res
            .extensions()
            .get::<ErrorDetail>()
            .expect("error detail should be attached for the logging middleware");
        assert_eq!(detail.message, "Query failed");
        assert_eq!(detail.context["operation"], "list_users");
    }

    #[test]
    fn api_error_includes_field_errors_in_response_body() {
        let mut field_errors = HashMap::new();
        field_errors.insert("email".to_string(), "Invalid email".to_string());

        let res = ApiError::bad_request("validation.failed", "Validation failed")
            .with_field_errors(field_errors)
            .into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = tokio::runtime::Runtime::new()
            .expect("runtime")
            .block_on(async move { to_bytes(res.into_body(), usize::MAX).await.expect("body") });
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("response should be json");
        let error = json["error"].as_object().expect("error object");
        assert!(error.get("fieldErrors").is_some());
        assert!(error.get("field_errors").is_none());
    }

    #[test]
    fn api_error_with_cause_adds_cause_to_extension_context_not_headers() {
        let res = ApiError::internal("db.error", "DB operation failed")
            .with_context(serde_json::json!({ "operation": "update_user" }))
            .with_cause(&"connection timeout")
            .into_response();

        assert!(res.headers().get("x-error-context").is_none());

        let detail = res
            .extensions()
            .get::<ErrorDetail>()
            .expect("error detail should be attached");
        assert_eq!(detail.context["operation"], "update_user");
        assert_eq!(detail.context["cause"], "connection timeout");
    }
}
