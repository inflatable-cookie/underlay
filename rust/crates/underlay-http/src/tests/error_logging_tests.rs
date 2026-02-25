#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::{
        body::Body,
        extract::State,
        http::{Request, StatusCode},
        middleware,
        response::IntoResponse,
        routing::get,
        Router,
    };
    use serde_json::json;
    use tower::ServiceExt;
    use underlay_testing::TestDb;

    use crate::{
        error_logging_middleware, list_error_logs, ApiError, ErrorLogFilters, ErrorLoggingConfig,
    };

    async fn failing_handler(State(_): State<()>) -> impl IntoResponse {
        ApiError::internal("test.internal_failure", "Integration test failure")
            .with_context(json!({
                "operation": "integration.error_logging",
                "entity_id": "abc-123",
            }))
            .into_response()
    }

    #[tokio::test]
    #[ignore = "requires Docker test database"]
    async fn middleware_persists_api_error_handler_context() {
        let db = TestDb::new().await;
        db.run_migrations("./migrations")
            .await
            .expect("migrations should run");

        let config = ErrorLoggingConfig::new(db.pool().clone()).with_source("underlay-http-test");

        let app = Router::new()
            .route("/fail", get(failing_handler))
            .with_state(())
            .layer(middleware::from_fn_with_state(
                config,
                error_logging_middleware,
            ));

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/fail?mode=integration")
                    .header("user-agent", "underlay-http-integration-test")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let mut row = None;
        for _ in 0..20 {
            let entries = list_error_logs(
                db.pool(),
                ErrorLogFilters {
                    endpoint: Some("/fail".to_string()),
                    limit: 1,
                    ..Default::default()
                },
            )
            .await
            .expect("should list logs");

            if let Some(found) = entries.into_iter().next() {
                row = Some(found);
                break;
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let row = row.expect("expected middleware to persist one error log row");
        assert_eq!(row.status_code, 500);
        assert_eq!(row.error_code, "test.internal_failure");
        assert_eq!(row.message, "Integration test failure");

        let source = row.context.get("source").and_then(|v| v.as_str());
        assert_eq!(source, Some("underlay-http-test"));

        let query = row.context.get("query").and_then(|v| v.as_str());
        assert_eq!(query, Some("mode=integration"));

        let user_agent = row.context.get("user_agent").and_then(|v| v.as_str());
        assert_eq!(user_agent, Some("underlay-http-integration-test"));

        let operation = row
            .context
            .get("handler_context")
            .and_then(|v| v.get("operation"))
            .and_then(|v| v.as_str());
        assert_eq!(operation, Some("integration.error_logging"));
    }
}
