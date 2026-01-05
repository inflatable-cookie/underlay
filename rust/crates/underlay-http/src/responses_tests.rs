#[cfg(test)]
mod tests {
    use axum::http::{header, StatusCode};
    use serde::Serialize;

    use crate::{created, list_ok, no_content, ok};

    #[derive(Debug, Serialize)]
    struct TestPayload {
        ok: bool,
    }

    #[test]
    fn ok_returns_200_and_json_content_type() {
        let response = ok(TestPayload { ok: true });
        assert_eq!(response.status(), StatusCode::OK);

        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .expect("content-type should be set");
        assert!(content_type
            .to_str()
            .expect("content-type should be valid")
            .starts_with("application/json"));
    }

    #[test]
    fn created_returns_201() {
        let response = created(TestPayload { ok: true });
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[test]
    fn list_ok_returns_200() {
        let response = list_ok(vec![TestPayload { ok: true }, TestPayload { ok: false }]);
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn no_content_returns_204() {
        let response = no_content();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
