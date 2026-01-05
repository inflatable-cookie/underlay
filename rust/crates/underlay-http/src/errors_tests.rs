#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use underlay_core::AppError;

    use crate::error_response;

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
}
