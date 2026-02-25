use axum::http::{header, HeaderValue, StatusCode};
use axum::response::IntoResponse;

use crate::{metrics_handler, BuildInfo, DefaultRegistry};

#[test]
fn build_info_labels_are_present_in_scrape_output() {
    let registry = DefaultRegistry::new();

    registry
        .register_default_metrics_with_build_info(
            "testns",
            BuildInfo {
                name: "my-service",
                version: "1.2.3",
            },
        )
        .expect("register_default_metrics_with_build_info should succeed");

    let text = registry.gather_text().expect("gather_text should succeed");

    assert!(text.contains("testns_build_info"));
    assert!(text.contains("name=\"my-service\""));
    assert!(text.contains("version=\"1.2.3\""));
}

#[tokio::test]
async fn metrics_handler_returns_plaintext_prometheus_format() {
    let registry = DefaultRegistry::new();
    registry
        .register_default_metrics_with_build_info(
            "testns",
            BuildInfo {
                name: "my-service",
                version: "1.2.3",
            },
        )
        .expect("register_default_metrics_with_build_info should succeed");

    let response = metrics_handler(axum::extract::State(registry))
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .expect("content-type header should be present");

    assert_eq!(
        content_type,
        &HeaderValue::from_static("text/plain; version=0.0.4")
    );
}
