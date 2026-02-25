    use super::*;

    #[test]
    fn test_guess_content_type() {
        assert_eq!(guess_content_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_content_type("photo.JPEG"), "image/jpeg");
        assert_eq!(guess_content_type("doc.pdf"), "application/pdf");
        assert_eq!(guess_content_type("video.mp4"), "video/mp4");
        assert_eq!(guess_content_type("font.woff2"), "font/woff2");
        assert_eq!(
            guess_content_type("unknown.xyz"),
            "application/octet-stream"
        );
        assert_eq!(
            guess_content_type("no-extension"),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_route_path_formatting() {
        // Build a valid adapter instance so the test does not rely on UB.
        let mut base = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        base.push(format!("underlay_blob_dev_server_test_{}", nanos));

        let runtime = tokio::runtime::Runtime::new().expect("runtime should build");
        let adapter = runtime
            .block_on(async {
                LocalAdapter::new(crate::adapters::LocalConfig::new(
                    &base,
                    "http://localhost/dev-blobs",
                ))
                .await
            })
            .expect("adapter should initialize");

        let builder = DevBlobRoutes::new(Arc::new(adapter));

        assert_eq!(builder.route_path, "/dev-blobs");

        let _ = std::fs::remove_dir_all(base);
    }