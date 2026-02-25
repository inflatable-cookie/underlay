use super::*;

#[test]
fn test_rendition_config_defaults() {
    let config = RenditionConfig::default();
    assert_eq!(config.thumbnail_max_dimension, 400);
    assert_eq!(config.preview_max_dimension, 1200);
    assert_eq!(config.jpeg_quality, 85);
    assert!(!config.square_thumbnails);
    assert!(config.generate_thumbnails);
    assert!(!config.generate_previews);
    assert_eq!(config.thumbnail_name, "thumb");
    assert_eq!(config.preview_name, "preview");
}

#[test]
fn test_rendition_config_builder() {
    let config = RenditionConfig::with_previews()
        .thumbnail_size(256)
        .preview_size(1024)
        .quality(90)
        .square()
        .thumbnail_name("thumb_256")
        .preview_name("large");

    assert_eq!(config.thumbnail_max_dimension, 256);
    assert_eq!(config.preview_max_dimension, 1024);
    assert_eq!(config.jpeg_quality, 90);
    assert!(config.square_thumbnails);
    assert!(config.generate_thumbnails);
    assert!(config.generate_previews);
    assert_eq!(config.thumbnail_name, "thumb_256");
    assert_eq!(config.preview_name, "large");
}

#[test]
fn test_farmyard_compat_config() {
    let config = RenditionConfig::farmyard_compat();
    assert_eq!(config.thumbnail_max_dimension, 128);
    assert_eq!(config.jpeg_quality, 80);
    assert!(config.square_thumbnails);
    assert!(config.generate_thumbnails);
    assert!(!config.generate_previews);
    assert_eq!(config.thumbnail_name, "thumb_128");
}

#[test]
fn test_quality_clamping() {
    let config = RenditionConfig::default().quality(150);
    assert_eq!(config.jpeg_quality, 100);

    let config = RenditionConfig::default().quality(0);
    assert_eq!(config.jpeg_quality, 1);
}
