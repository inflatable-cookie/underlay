    use super::*;

    #[test]
    fn test_default_version_key() {
        let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
        let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

        let key = version_key(media_id, version_id, "photo.jpg");
        assert_eq!(
            key,
            "media/01234567-89ab-cdef-0123-456789abcdef/versions/fedcba98-7654-3210-fedc-ba9876543210/photo.jpg"
        );
    }

    #[test]
    fn test_default_rendition_key() {
        let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
        let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

        let key = rendition_key(media_id, version_id, "thumb");
        assert_eq!(
            key,
            "media/01234567-89ab-cdef-0123-456789abcdef/renditions/fedcba98-7654-3210-fedc-ba9876543210/thumb.jpg"
        );
    }

    #[test]
    fn test_custom_config() {
        let config = StorageKeyConfig::with_prefix("uploads")
            .versions_dir("original")
            .renditions_dir("thumbnails")
            .rendition_extension("webp");

        let generator = StorageKeyGenerator::new(config);
        let media_id = Uuid::nil();
        let version_id = Uuid::nil();

        let version_key = generator.version_key(media_id, version_id, "test.png");
        assert_eq!(
            version_key,
            "uploads/00000000-0000-0000-0000-000000000000/original/00000000-0000-0000-0000-000000000000/test.png"
        );

        let rendition_key = generator.rendition_key(media_id, version_id, "thumb");
        assert_eq!(
            rendition_key,
            "uploads/00000000-0000-0000-0000-000000000000/thumbnails/00000000-0000-0000-0000-000000000000/thumb.webp"
        );
    }

    #[test]
    fn test_rendition_key_for_type() {
        let generator = StorageKeyGenerator::with_defaults();
        let media_id = Uuid::nil();
        let version_id = Uuid::nil();

        assert!(generator
            .rendition_key_for_type(media_id, version_id, &RenditionType::Thumbnail)
            .ends_with("/thumb.jpg"));
        assert!(generator
            .rendition_key_for_type(media_id, version_id, &RenditionType::Preview)
            .ends_with("/preview.jpg"));
        assert!(generator
            .rendition_key_for_type(
                media_id,
                version_id,
                &RenditionType::Custom("hero".to_string())
            )
            .ends_with("/hero.jpg"));
    }

    #[test]
    fn test_prefix_generation() {
        let generator = StorageKeyGenerator::with_defaults();
        let media_id = Uuid::nil();
        let version_id = Uuid::nil();

        assert_eq!(
            generator.media_prefix(media_id),
            "media/00000000-0000-0000-0000-000000000000/"
        );
        assert_eq!(
            generator.versions_prefix(media_id),
            "media/00000000-0000-0000-0000-000000000000/versions/"
        );
        assert_eq!(
            generator.renditions_prefix(media_id),
            "media/00000000-0000-0000-0000-000000000000/renditions/"
        );
        assert_eq!(
            generator.version_renditions_prefix(media_id, version_id),
            "media/00000000-0000-0000-0000-000000000000/renditions/00000000-0000-0000-0000-000000000000/"
        );
    }

    #[test]
    fn test_version_filename() {
        assert_eq!(
            version_filename("image/jpeg", Some("my-photo.jpg")),
            "my-photo.jpg"
        );
        assert_eq!(version_filename("image/jpeg", None), "file.jpg");
        assert_eq!(version_filename("image/png", None), "file.png");
        assert_eq!(version_filename("application/pdf", None), "file.pdf");
        assert_eq!(
            version_filename("application/octet-stream", None),
            "file.bin"
        );
    }

    #[test]
    fn test_mime_to_extension() {
        assert_eq!(mime_to_extension("image/jpeg"), "jpg");
        assert_eq!(mime_to_extension("image/png"), "png");
        assert_eq!(mime_to_extension("image/gif"), "gif");
        assert_eq!(mime_to_extension("image/webp"), "webp");
        assert_eq!(mime_to_extension("application/pdf"), "pdf");
        assert_eq!(mime_to_extension("video/mp4"), "mp4");
        assert_eq!(mime_to_extension("unknown/type"), "bin");
    }