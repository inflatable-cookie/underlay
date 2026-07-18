#[cfg(test)]
mod tests {
    use http::header::HeaderName;

    use underlay_observability::Environment;

    use crate::cors::{try_cors_layer_for_env, CorsConfig, DEFAULT_CORS_MAX_AGE_SECS};

    // ========================================================================
    // Default Configuration
    // ========================================================================

    #[test]
    fn default_config_allows_no_cross_origin() {
        let config = CorsConfig::default();
        assert!(!config.allow_any_origin());
        assert!(!config.mirror_origin());
        assert!(config.allowed_origins().is_empty());
    }

    #[test]
    fn wildcard_is_an_explicit_opt_in() {
        let config = CorsConfig::default().with_any_origin();
        assert!(config.allow_any_origin());
    }

    #[test]
    fn default_config_has_no_credentials() {
        let config = CorsConfig::default();
        assert!(!config.allow_credentials());
    }

    #[test]
    fn default_config_has_standard_headers() {
        let config = CorsConfig::default();
        let header_names: Vec<&str> = config
            .allowed_headers()
            .iter()
            .map(|h| h.as_str())
            .collect();

        assert!(header_names.contains(&"authorization"));
        assert!(header_names.contains(&"content-type"));
        assert!(header_names.contains(&"x-request-id"));
    }

    #[test]
    fn default_config_has_one_hour_max_age() {
        let config = CorsConfig::default();
        assert_eq!(config.max_age_secs(), DEFAULT_CORS_MAX_AGE_SECS);
        assert_eq!(config.max_age_secs(), 3600);
    }

    #[test]
    fn new_returns_default_config() {
        let config = CorsConfig::new();
        let default = CorsConfig::default();

        assert_eq!(config.allow_any_origin(), default.allow_any_origin());
        assert_eq!(config.mirror_origin(), default.mirror_origin());
        assert_eq!(config.allow_credentials(), default.allow_credentials());
        assert_eq!(config.max_age_secs(), default.max_age_secs());
    }

    // ========================================================================
    // Origin Configuration
    // ========================================================================

    #[test]
    fn with_any_origin_sets_flags_correctly() {
        let config = CorsConfig::new()
            .with_mirror_origin() // Set mirror first
            .with_any_origin(); // Then override with any

        assert!(config.allow_any_origin());
        assert!(!config.mirror_origin());
    }

    #[test]
    fn with_mirror_origin_sets_flags_correctly() {
        let config = CorsConfig::new()
            .with_any_origin() // Set any first
            .with_mirror_origin(); // Then override with mirror

        assert!(!config.allow_any_origin());
        assert!(config.mirror_origin());
    }

    #[test]
    fn with_origins_sets_explicit_origins() {
        let config =
            CorsConfig::new().with_origins(["https://example.com", "https://app.example.com"]);

        assert!(!config.allow_any_origin());
        assert!(!config.mirror_origin());
        assert_eq!(config.allowed_origins().len(), 2);
    }

    #[test]
    fn with_origins_clears_any_and_mirror() {
        let config = CorsConfig::new()
            .with_mirror_origin()
            .with_origins(["https://example.com"]);

        assert!(!config.allow_any_origin());
        assert!(!config.mirror_origin());
    }

    #[test]
    fn with_origins_filters_invalid_values() {
        // Invalid header values should be filtered out
        let config = CorsConfig::new().with_origins([
            "https://valid.com",
            "invalid\nheader", // Newlines are invalid in header values
            "https://also-valid.com",
        ]);

        // Should only have the valid origins
        assert_eq!(config.allowed_origins().len(), 2);
    }

    #[test]
    fn try_with_origins_rejects_invalid_values() {
        let error = CorsConfig::new()
            .try_with_origins(["https://valid.com", "invalid\nheader"])
            .expect_err("invalid origin should fail");

        assert!(error.to_string().contains("invalid CORS origin"));
    }

    #[test]
    fn with_origin_values_accepts_parsed_origins() {
        let origins = vec![
            http::header::HeaderValue::from_static("https://a.com"),
            http::header::HeaderValue::from_static("https://b.com"),
        ];

        let config = CorsConfig::new().with_origin_values(origins);

        assert!(!config.allow_any_origin());
        assert_eq!(config.allowed_origins().len(), 2);
    }

    #[test]
    fn with_origins_accepts_various_iterators() {
        // Vec
        let config1 = CorsConfig::new().with_origins(vec!["https://a.com", "https://b.com"]);
        assert_eq!(config1.allowed_origins().len(), 2);

        // Array
        let config2 = CorsConfig::new().with_origins(["https://c.com"]);
        assert_eq!(config2.allowed_origins().len(), 1);

        // Slice via iter
        let origins = ["https://d.com", "https://e.com"];
        let config3 = CorsConfig::new().with_origins(origins);
        assert_eq!(config3.allowed_origins().len(), 2);
    }

    // ========================================================================
    // Header Configuration
    // ========================================================================

    #[test]
    fn with_header_adds_to_existing() {
        let config = CorsConfig::new().with_header(HeaderName::from_static("x-custom-header"));

        let header_names: Vec<&str> = config
            .allowed_headers()
            .iter()
            .map(|h| h.as_str())
            .collect();

        // Should have defaults plus custom
        assert!(header_names.contains(&"authorization"));
        assert!(header_names.contains(&"content-type"));
        assert!(header_names.contains(&"x-custom-header"));
    }

    #[test]
    fn with_headers_replaces_defaults() {
        let config = CorsConfig::new().with_headers(vec![HeaderName::from_static("x-only-this")]);

        assert_eq!(config.allowed_headers().len(), 1);
        assert_eq!(config.allowed_headers()[0].as_str(), "x-only-this");
    }

    #[test]
    fn with_header_can_be_chained() {
        let config = CorsConfig::new()
            .with_header(HeaderName::from_static("x-first"))
            .with_header(HeaderName::from_static("x-second"));

        let header_names: Vec<&str> = config
            .allowed_headers()
            .iter()
            .map(|h| h.as_str())
            .collect();

        assert!(header_names.contains(&"x-first"));
        assert!(header_names.contains(&"x-second"));
    }

    // ========================================================================
    // Credentials Configuration
    // ========================================================================

    #[test]
    fn with_credentials_true_enables_credentials() {
        let config = CorsConfig::new().with_credentials(true);
        assert!(config.allow_credentials());
    }

    #[test]
    fn with_credentials_false_disables_credentials() {
        let config = CorsConfig::new()
            .with_credentials(true)
            .with_credentials(false);

        assert!(!config.allow_credentials());
    }

    // ========================================================================
    // Max Age Configuration
    // ========================================================================

    #[test]
    fn with_max_age_sets_custom_value() {
        let config = CorsConfig::new().with_max_age(7200);
        assert_eq!(config.max_age_secs(), 7200);
    }

    #[test]
    fn with_max_age_can_be_zero() {
        let config = CorsConfig::new().with_max_age(0);
        assert_eq!(config.max_age_secs(), 0);
    }

    // ========================================================================
    // Builder Chaining
    // ========================================================================

    #[test]
    fn full_production_config() {
        let config = CorsConfig::new()
            .with_origins(["https://app.example.com", "https://admin.example.com"])
            .with_credentials(true)
            .with_max_age(86400) // 24 hours
            .with_header(HeaderName::from_static("x-csrf-token"));

        assert!(!config.allow_any_origin());
        assert!(!config.mirror_origin());
        assert!(config.allow_credentials());
        assert_eq!(config.max_age_secs(), 86400);
        assert_eq!(config.allowed_origins().len(), 2);

        let header_names: Vec<&str> = config
            .allowed_headers()
            .iter()
            .map(|h| h.as_str())
            .collect();
        assert!(header_names.contains(&"x-csrf-token"));
    }

    #[test]
    fn local_dev_config_with_mirror() {
        let config = CorsConfig::new()
            .with_mirror_origin()
            .with_credentials(true);

        assert!(!config.allow_any_origin());
        assert!(config.mirror_origin());
        assert!(config.allow_credentials());
    }

    // ========================================================================
    // Environment Gating
    // ========================================================================

    #[test]
    fn mirror_with_credentials_builds_in_local_and_test() {
        for env in [Environment::Local, Environment::Test] {
            let config = CorsConfig::new()
                .with_mirror_origin()
                .with_credentials(true);
            assert!(try_cors_layer_for_env(config, env).is_ok());
        }
    }

    #[test]
    fn mirror_with_credentials_rejected_outside_local_and_test() {
        for env in [Environment::Dev, Environment::Staging, Environment::Prod] {
            let config = CorsConfig::new()
                .with_mirror_origin()
                .with_credentials(true);
            let error = try_cors_layer_for_env(config, env)
                .expect_err("mirror + credentials must be rejected outside Local/Test");
            assert!(error.to_string().contains("only"));
        }
    }

    #[test]
    fn mirror_without_credentials_builds_in_any_environment() {
        let config = CorsConfig::new().with_mirror_origin();
        assert!(try_cors_layer_for_env(config, Environment::Prod).is_ok());
    }

    #[test]
    #[should_panic(expected = "explicit environment gate")]
    fn cors_layer_panics_on_mirror_with_credentials() {
        let config = CorsConfig::new()
            .with_mirror_origin()
            .with_credentials(true);
        let _ = crate::cors::cors_layer(config);
    }

    #[test]
    fn internal_service_config() {
        let config = CorsConfig::new().with_any_origin().with_credentials(false);

        assert!(config.allow_any_origin());
        assert!(!config.allow_credentials());
    }

    // ========================================================================
    // Clone and Debug
    // ========================================================================

    #[test]
    fn config_is_cloneable() {
        let config = CorsConfig::new()
            .with_origins(["https://example.com"])
            .with_credentials(true);

        let cloned = config.clone();

        assert_eq!(cloned.allow_credentials(), config.allow_credentials());
        assert_eq!(
            cloned.allowed_origins().len(),
            config.allowed_origins().len()
        );
    }

    #[test]
    fn config_is_debuggable() {
        let config = CorsConfig::new();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("CorsConfig"));
        assert!(debug_str.contains("allow_any_origin"));
    }

    // ========================================================================
    // DEFAULT_CORS_MAX_AGE_SECS Constant
    // ========================================================================

    #[test]
    fn default_max_age_constant_is_one_hour() {
        assert_eq!(DEFAULT_CORS_MAX_AGE_SECS, 3600);
    }

    #[test]
    fn accessors_expose_config_without_field_access() {
        let config = CorsConfig::new()
            .with_origins(["https://example.com"])
            .with_credentials(true);

        assert!(!config.allow_any_origin());
        assert!(!config.mirror_origin());
        assert!(config.allow_credentials());
        assert_eq!(config.allowed_origins().len(), 1);
        assert_eq!(config.allowed_headers().len(), 3);
        assert_eq!(config.max_age_secs(), DEFAULT_CORS_MAX_AGE_SECS);
    }
}
