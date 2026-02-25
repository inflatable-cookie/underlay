    use super::*;

    #[test]
    fn test_default_config() {
        let config = HttpServerConfig::default();
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.public_host, "localhost");
    }

    #[test]
    fn test_socket_addr() {
        let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
        assert_eq!(config.socket_addr(), "0.0.0.0:8080");
    }

    #[test]
    fn test_base_url() {
        let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
        assert_eq!(config.http_base_url(), "http://api.example.com:8080");
        assert_eq!(config.https_base_url(), "https://api.example.com:8080");
    }

    #[test]
    fn test_local_defaults() {
        // Clear env vars to test defaults
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("PUBLIC_HOST");

        let config = HttpServerConfig::from_env(Environment::Local);
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 3000);
        // public_host defaults to bind_addr
        assert_eq!(config.public_host, "127.0.0.1");
    }