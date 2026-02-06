#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use underlay_core::AppError;

    use crate::{error_response, ApiError};

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
    fn api_error_sets_required_headers() {
        let res = ApiError::internal("db.query_failed", "Query failed")
            .with_context(serde_json::json!({ "operation": "list_users" }))
            .into_response();

        let code = res
            .headers()
            .get("x-error-code")
            .expect("x-error-code should be set");
        let message = res
            .headers()
            .get("x-error-message")
            .expect("x-error-message should be set");
        let context = res
            .headers()
            .get("x-error-context")
            .expect("x-error-context should be set");

        assert_eq!(code.to_str().unwrap(), "db.query_failed");
        assert_eq!(message.to_str().unwrap(), "Query failed");

        let decoded =
            urlencoding::decode(context.to_str().unwrap()).expect("context should decode");
        let parsed: serde_json::Value =
            serde_json::from_str(&decoded).expect("context should be valid json");
        assert_eq!(parsed["operation"], "list_users");
    }

    #[test]
    fn api_error_includes_field_errors_in_response_body() {
        let mut field_errors = HashMap::new();
        field_errors.insert("email".to_string(), "Invalid email".to_string());

        let res = ApiError::bad_request("validation.failed", "Validation failed")
            .with_field_errors(field_errors)
            .into_response();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn api_error_with_cause_adds_cause_to_context() {
        let res = ApiError::internal("db.error", "DB operation failed")
            .with_context(serde_json::json!({ "operation": "update_user" }))
            .with_cause(&"connection timeout")
            .into_response();

        let context_header = res
            .headers()
            .get("x-error-context")
            .expect("x-error-context should be set");

        let decoded =
            urlencoding::decode(context_header.to_str().unwrap()).expect("context should decode");
        let parsed: serde_json::Value =
            serde_json::from_str(&decoded).expect("context should be valid json");
        assert_eq!(parsed["operation"], "update_user");
        assert_eq!(parsed["cause"], "connection timeout");
    }
}
