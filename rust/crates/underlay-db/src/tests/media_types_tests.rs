    use super::*;

    #[test]
    fn test_media_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&MediaKind::Image).unwrap(),
            "\"image\""
        );
        assert_eq!(serde_json::to_string(&MediaKind::Pdf).unwrap(), "\"pdf\"");
    }

    #[test]
    fn test_media_kind_deserialization() {
        assert_eq!(
            serde_json::from_str::<MediaKind>("\"image\"").unwrap(),
            MediaKind::Image
        );
        assert_eq!(
            serde_json::from_str::<MediaKind>("\"pdf\"").unwrap(),
            MediaKind::Pdf
        );
    }

    #[test]
    fn test_media_kind_from_str() {
        assert_eq!("image".parse::<MediaKind>().unwrap(), MediaKind::Image);
        assert_eq!("IMAGE".parse::<MediaKind>().unwrap(), MediaKind::Image);
        assert_eq!("pdf".parse::<MediaKind>().unwrap(), MediaKind::Pdf);
        assert!("invalid".parse::<MediaKind>().is_err());
    }

    #[test]
    fn test_media_visibility_serialization() {
        assert_eq!(
            serde_json::to_string(&MediaVisibility::Public).unwrap(),
            "\"public\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVisibility::Restricted).unwrap(),
            "\"restricted\""
        );
    }

    #[test]
    fn test_media_visibility_from_str() {
        assert_eq!(
            "public".parse::<MediaVisibility>().unwrap(),
            MediaVisibility::Public
        );
        assert_eq!(
            "restricted".parse::<MediaVisibility>().unwrap(),
            MediaVisibility::Restricted
        );
        assert!("invalid".parse::<MediaVisibility>().is_err());
    }

    #[test]
    fn test_media_version_state_serialization() {
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Uploading).unwrap(),
            "\"uploading\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Ready).unwrap(),
            "\"ready\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Failed).unwrap(),
            "\"failed\""
        );
        assert_eq!(
            serde_json::to_string(&MediaVersionState::Purging).unwrap(),
            "\"purging\""
        );
    }

    #[test]
    fn test_media_version_state_from_str() {
        assert_eq!(
            "uploading".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Uploading
        );
        assert_eq!(
            "ready".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Ready
        );
        assert_eq!(
            "failed".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Failed
        );
        assert_eq!(
            "purging".parse::<MediaVersionState>().unwrap(),
            MediaVersionState::Purging
        );
        assert!("invalid".parse::<MediaVersionState>().is_err());
    }

    #[test]
    fn test_media_version_state_is_terminal() {
        assert!(!MediaVersionState::Uploading.is_terminal());
        assert!(MediaVersionState::Ready.is_terminal());
        assert!(MediaVersionState::Failed.is_terminal());
        assert!(!MediaVersionState::Purging.is_terminal());
    }

    #[test]
    fn test_detect_media_kind_from_mime_type() {
        assert_eq!(
            detect_media_kind_from_mime_type("image/jpeg"),
            Some(MediaKind::Image)
        );
        assert_eq!(
            detect_media_kind_from_mime_type("image/png"),
            Some(MediaKind::Image)
        );
        assert_eq!(
            detect_media_kind_from_mime_type("application/pdf"),
            Some(MediaKind::Pdf)
        );
        assert_eq!(detect_media_kind_from_mime_type("text/plain"), None);
        assert_eq!(detect_media_kind_from_mime_type("video/mp4"), None);
    }