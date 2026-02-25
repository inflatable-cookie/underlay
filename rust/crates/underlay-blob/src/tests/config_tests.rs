    use super::*;

    #[test]
    fn test_defaults() {
        let config = MediaConfig::default();
        assert_eq!(config.max_file_size_bytes, 50 * 1024 * 1024);
        assert_eq!(config.thumbnail_max_dimension, 300);
    }

    #[test]
    fn test_builder_methods() {
        let config = MediaConfig::default()
            .max_file_size_mb(100)
            .thumbnail_dimension(400);

        assert_eq!(config.max_file_size_bytes, 100 * 1024 * 1024);
        assert_eq!(config.thumbnail_max_dimension, 400);
    }

    #[test]
    fn test_is_size_allowed() {
        let config = MediaConfig::default().max_file_size_mb(10);

        assert!(config.is_size_allowed(5 * 1024 * 1024)); // 5 MB - ok
        assert!(config.is_size_allowed(10 * 1024 * 1024)); // 10 MB - ok (at limit)
        assert!(!config.is_size_allowed(11 * 1024 * 1024)); // 11 MB - too big
    }

    #[test]
    fn test_display() {
        let config = MediaConfig::default();
        assert_eq!(config.max_file_size_display(), "50 MB");

        let config = MediaConfig::default().max_file_size_mb(100);
        assert_eq!(config.max_file_size_display(), "100 MB");
    }