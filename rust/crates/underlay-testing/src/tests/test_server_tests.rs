    use super::*;
    use axum::routing::get;
    use axum::Json;
    use serde::Deserialize;

    async fn hello() -> &'static str {
        "Hello, World!"
    }

    async fn json_endpoint() -> Json<serde_json::Value> {
        Json(serde_json::json!({ "message": "hello" }))
    }

    #[tokio::test]
    async fn test_get_request() {
        let app = Router::new().route("/hello", get(hello));
        let server = TestServer::new(app);

        let response = server.get("/hello").send().await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_json_response() {
        let app = Router::new().route("/json", get(json_endpoint));
        let server = TestServer::new(app);

        let response = server.get("/json").send().await;

        response.assert_ok();

        #[derive(Deserialize)]
        struct Response {
            message: String,
        }

        let data: Response = response.json();
        assert_eq!(data.message, "hello");
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = Router::new();
        let server = TestServer::new(app);

        let response = server.get("/nonexistent").send().await;

        response.assert_not_found();
    }

    #[tokio::test]
    async fn test_headers() {
        let app = Router::new().route("/hello", get(hello));
        let server = TestServer::new(app);

        let response = server
            .get("/hello")
            .header("X-Custom", "value")
            .send()
            .await;

        response.assert_ok();
    }

    #[tokio::test]
    async fn test_json_body() {
        use axum::routing::post;

        async fn echo(Json(body): Json<serde_json::Value>) -> Json<serde_json::Value> {
            Json(body)
        }

        let app = Router::new().route("/echo", post(echo));
        let server = TestServer::new(app);

        let payload = serde_json::json!({ "name": "test" });
        let response = server.post("/echo").json(&payload).send().await;

        response.assert_ok();
        let result: serde_json::Value = response.json();
        assert_eq!(result["name"], "test");
    }