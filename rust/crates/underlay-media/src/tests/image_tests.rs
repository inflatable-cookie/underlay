use super::*;

#[test]
fn test_calculate_dimensions_landscape() {
    let (w, h) = calculate_thumbnail_dimensions(1920, 1080, 256, 256);
    assert_eq!(w, 256);
    assert_eq!(h, 144);
}

#[test]
fn test_calculate_dimensions_portrait() {
    let (w, h) = calculate_thumbnail_dimensions(1080, 1920, 256, 256);
    assert_eq!(w, 144);
    assert_eq!(h, 256);
}

#[test]
fn test_calculate_dimensions_square() {
    let (w, h) = calculate_thumbnail_dimensions(1000, 1000, 256, 256);
    assert_eq!(w, 256);
    assert_eq!(h, 256);
}

#[test]
fn test_calculate_dimensions_smaller_than_target() {
    // When source is smaller, scale maintains ratio
    let (w, h) = calculate_thumbnail_dimensions(100, 50, 256, 256);
    assert_eq!(w, 256);
    assert_eq!(h, 128);
}

#[test]
fn test_calculate_dimensions_non_square_target() {
    // Wide target area
    let (w, h) = calculate_thumbnail_dimensions(1000, 1000, 400, 200);
    assert_eq!(w, 200);
    assert_eq!(h, 200);

    // Tall target area
    let (w, h) = calculate_thumbnail_dimensions(1000, 1000, 200, 400);
    assert_eq!(w, 200);
    assert_eq!(h, 200);
}

#[test]
fn test_calculate_dimensions_minimum_size() {
    // Very small source should produce at least 1x1
    let (w, h) = calculate_thumbnail_dimensions(1, 1, 256, 256);
    assert!(w >= 1);
    assert!(h >= 1);
}

#[test]
fn test_thumbnail_config_presets() {
    let small = ThumbnailConfig::small();
    assert_eq!(small.max_width(), 128);
    assert_eq!(small.max_height(), 128);

    let medium = ThumbnailConfig::medium();
    assert_eq!(medium.max_width(), 256);
    assert_eq!(medium.max_height(), 256);

    let large = ThumbnailConfig::large();
    assert_eq!(large.max_width(), 512);
    assert_eq!(large.max_height(), 512);

    let xlarge = ThumbnailConfig::xlarge();
    assert_eq!(xlarge.max_width(), 1024);
    assert_eq!(xlarge.max_height(), 1024);
}

#[test]
fn test_thumbnail_config_builder() {
    let config = ThumbnailConfig::new(300, 200).with_quality(90);
    assert_eq!(config.max_width(), 300);
    assert_eq!(config.max_height(), 200);
    assert_eq!(config.quality(), 90);
}

#[test]
fn test_thumbnail_config_quality_clamping() {
    let config = ThumbnailConfig::new(100, 100).with_quality(150);
    assert_eq!(config.quality(), 100);

    let config = ThumbnailConfig::new(100, 100).with_quality(0);
    assert_eq!(config.quality(), 1);
}

#[test]
fn test_is_supported_image() {
    assert!(is_supported_image("image/jpeg"));
    assert!(is_supported_image("image/png"));
    assert!(is_supported_image("image/gif"));
    assert!(is_supported_image("image/webp"));
    assert!(!is_supported_image("image/svg+xml"));
    assert!(!is_supported_image("application/pdf"));
    assert!(!is_supported_image("text/plain"));
}

#[test]
fn test_format_from_mime() {
    assert_eq!(format_from_mime("image/jpeg"), Some(ImageFormat::Jpeg));
    assert_eq!(format_from_mime("image/png"), Some(ImageFormat::Png));
    assert_eq!(format_from_mime("image/gif"), Some(ImageFormat::Gif));
    assert_eq!(format_from_mime("image/webp"), Some(ImageFormat::WebP));
    assert_eq!(format_from_mime("image/svg+xml"), None);
    assert_eq!(format_from_mime("text/plain"), None);
}

#[test]
fn test_mime_from_format() {
    assert_eq!(mime_from_format(ImageFormat::Jpeg), "image/jpeg");
    assert_eq!(mime_from_format(ImageFormat::Png), "image/png");
    assert_eq!(mime_from_format(ImageFormat::Gif), "image/gif");
    assert_eq!(mime_from_format(ImageFormat::WebP), "image/webp");
}

#[test]
fn square_thumbnail_preserves_png_alpha() {
    let mut source = image::RgbaImage::new(32, 32);
    for pixel in source.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 0]);
    }
    source.put_pixel(16, 16, image::Rgba([0, 255, 0, 255]));

    let mut input = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(source)
        .write_to(&mut input, ImageFormat::Png)
        .expect("encode source png");

    let result = generate_square_thumbnail_preserving_alpha(input.get_ref(), 16, 80)
        .expect("generate alpha-preserving thumbnail");

    assert_eq!(result.mime_type, "image/png");
    assert_eq!(result.width, 16);
    assert_eq!(result.height, 16);

    let decoded = image::load_from_memory(&result.data)
        .expect("decode generated thumbnail")
        .to_rgba8();
    assert_eq!(decoded.get_pixel(0, 0).0[3], 0);
}

#[test]
fn square_thumbnail_without_alpha_still_uses_jpeg() {
    let source = image::RgbImage::from_pixel(32, 32, image::Rgb([0, 255, 0]));
    let mut input = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(source)
        .write_to(&mut input, ImageFormat::Png)
        .expect("encode source png");

    let result = generate_square_thumbnail_preserving_alpha(input.get_ref(), 16, 80)
        .expect("generate opaque thumbnail");

    assert_eq!(result.mime_type, "image/jpeg");
    assert_eq!(result.width, 16);
    assert_eq!(result.height, 16);
}
