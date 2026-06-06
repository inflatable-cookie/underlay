//! Image processing utilities.
//!
//! Functions for generating thumbnails and other image renditions from source
//! images. Supports JPEG, PNG, GIF, and WebP input formats with JPEG output.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_media::image::{generate_thumbnail, ThumbnailConfig};
//!
//! let config = ThumbnailConfig::medium();
//! let result = generate_thumbnail(&image_bytes, &config)?;
//! ```

use image::{GenericImageView, ImageFormat, ImageReader, imageops::FilterType};
use std::io::Cursor;

/// Error type for image processing operations.
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    /// Failed to decode the source image.
    #[error("Image decode error: {0}")]
    DecodeError(String),
    /// Failed to encode the output image.
    #[error("Image encode error: {0}")]
    EncodeError(String),
    /// Unsupported image format.
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// Result of generating a thumbnail.
#[derive(Debug)]
pub struct ThumbnailResult {
    /// The thumbnail image data (JPEG encoded).
    pub data: Vec<u8>,
    /// Width of the thumbnail in pixels.
    pub width: u32,
    /// Height of the thumbnail in pixels.
    pub height: u32,
    /// MIME type of the output (always "image/jpeg").
    pub mime_type: &'static str,
}

/// Configuration for thumbnail generation.
///
/// # Example
///
/// ```rust
/// use underlay_media::image::ThumbnailConfig;
///
/// // Use a preset
/// let config = ThumbnailConfig::medium();
///
/// // Or create a custom config
/// let config = ThumbnailConfig::new(400, 300).with_quality(90);
/// ```
#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    /// Maximum width of the thumbnail.
    max_width: u32,
    /// Maximum height of the thumbnail.
    max_height: u32,
    /// JPEG quality (1-100). Higher values produce better quality but larger files.
    quality: u8,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            max_width: 256,
            max_height: 256,
            quality: 85,
        }
    }
}

impl ThumbnailConfig {
    /// Create a new thumbnail config with the given dimensions.
    pub fn new(max_width: u32, max_height: u32) -> Self {
        Self {
            max_width,
            max_height,
            quality: 85,
        }
    }

    /// Set the JPEG quality (1-100).
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    /// Return the maximum thumbnail width.
    pub fn max_width(&self) -> u32 {
        self.max_width
    }

    /// Return the maximum thumbnail height.
    pub fn max_height(&self) -> u32 {
        self.max_height
    }

    /// Return the JPEG quality setting.
    pub fn quality(&self) -> u8 {
        self.quality
    }

    /// Create a small thumbnail config (128x128, quality 80).
    ///
    /// Suitable for list views and compact displays.
    pub fn small() -> Self {
        Self {
            max_width: 128,
            max_height: 128,
            quality: 80,
        }
    }

    /// Create a medium thumbnail config (256x256, quality 85).
    ///
    /// Good balance between size and quality for most use cases.
    pub fn medium() -> Self {
        Self::default()
    }

    /// Create a large thumbnail config (512x512, quality 85).
    ///
    /// Suitable for detail views and high-DPI displays.
    pub fn large() -> Self {
        Self {
            max_width: 512,
            max_height: 512,
            quality: 85,
        }
    }

    /// Create an extra-large thumbnail config (1024x1024, quality 90).
    ///
    /// Suitable for hero images and full-screen previews.
    pub fn xlarge() -> Self {
        Self {
            max_width: 1024,
            max_height: 1024,
            quality: 90,
        }
    }
}

/// Generate a thumbnail from image bytes.
///
/// The thumbnail preserves the aspect ratio of the original image and
/// fits within the specified maximum dimensions. The output is always
/// JPEG encoded.
///
/// # Arguments
///
/// * `data` - The source image bytes (JPEG, PNG, GIF, or WebP)
/// * `config` - Configuration for the thumbnail
///
/// # Returns
///
/// A `ThumbnailResult` containing the JPEG-encoded thumbnail data.
///
/// # Example
///
/// ```rust,ignore
/// use underlay_media::image::{generate_thumbnail, ThumbnailConfig};
///
/// let config = ThumbnailConfig::medium();
/// let result = generate_thumbnail(&image_bytes, &config)?;
///
/// // result.data contains the JPEG bytes
/// // result.width and result.height contain the actual dimensions
/// ```
pub fn generate_thumbnail(
    data: &[u8],
    config: &ThumbnailConfig,
) -> Result<ThumbnailResult, ImageError> {
    // Decode the source image
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?
        .decode()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?;

    // Calculate dimensions preserving aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let (new_width, new_height) = calculate_thumbnail_dimensions(
        orig_width,
        orig_height,
        config.max_width(),
        config.max_height(),
    );

    // Skip resizing if the image is already smaller than target
    let thumb = if new_width >= orig_width && new_height >= orig_height {
        img
    } else {
        img.resize(new_width, new_height, FilterType::Lanczos3)
    };

    // Encode as JPEG
    let mut output = Vec::new();
    let mut encoder =
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, config.quality());
    encoder
        .encode(
            thumb.to_rgb8().as_raw(),
            thumb.width(),
            thumb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| ImageError::EncodeError(e.to_string()))?;

    Ok(ThumbnailResult {
        data: output,
        width: thumb.width(),
        height: thumb.height(),
        mime_type: "image/jpeg",
    })
}

/// Generate a square thumbnail by center-cropping the image.
///
/// The image is first resized so the smaller dimension matches the target size,
/// then center-cropped to produce a square. This is ideal for avatars, profile
/// pictures, and other contexts where a consistent square aspect ratio is needed.
///
/// # Arguments
///
/// * `data` - The source image bytes (JPEG, PNG, GIF, or WebP)
/// * `size` - The target width and height of the square thumbnail
/// * `quality` - JPEG quality (1-100)
///
/// # Example
///
/// ```rust,ignore
/// use underlay_media::image::generate_square_thumbnail;
///
/// // Generate a 128x128 avatar thumbnail
/// let result = generate_square_thumbnail(&image_bytes, 128, 85)?;
/// assert_eq!(result.width, 128);
/// assert_eq!(result.height, 128);
/// ```
pub fn generate_square_thumbnail(
    data: &[u8],
    size: u32,
    quality: u8,
) -> Result<ThumbnailResult, ImageError> {
    // Decode the source image
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?
        .decode()
        .map_err(|e| ImageError::DecodeError(e.to_string()))?;

    let (orig_width, orig_height) = img.dimensions();

    // Calculate scale to make smaller dimension = size
    let scale = if orig_width < orig_height {
        size as f32 / orig_width as f32
    } else {
        size as f32 / orig_height as f32
    };

    let scaled_width = (orig_width as f32 * scale).ceil() as u32;
    let scaled_height = (orig_height as f32 * scale).ceil() as u32;

    // Resize first (if needed)
    let resized = if scaled_width <= orig_width && scaled_height <= orig_height {
        // Image is larger than target, resize down
        img.resize_exact(scaled_width, scaled_height, FilterType::Lanczos3)
    } else if orig_width >= size && orig_height >= size {
        // Image is at least as large as target in both dimensions, no resize needed
        img
    } else {
        // Image is smaller than target, scale up
        img.resize_exact(scaled_width, scaled_height, FilterType::Lanczos3)
    };

    // Center crop to square
    let x = (resized.width().saturating_sub(size)) / 2;
    let y = (resized.height().saturating_sub(size)) / 2;
    let crop_size = size.min(resized.width()).min(resized.height());
    let thumb = resized.crop_imm(x, y, crop_size, crop_size);

    // Encode as JPEG
    let mut output = Vec::new();
    let quality = quality.clamp(1, 100);
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
    encoder
        .encode(
            thumb.to_rgb8().as_raw(),
            thumb.width(),
            thumb.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| ImageError::EncodeError(e.to_string()))?;

    Ok(ThumbnailResult {
        data: output,
        width: thumb.width(),
        height: thumb.height(),
        mime_type: "image/jpeg",
    })
}

/// Calculate thumbnail dimensions preserving aspect ratio.
///
/// Returns dimensions that fit within the maximum bounds while maintaining
/// the original aspect ratio.
///
/// # Arguments
///
/// * `orig_width` - Original image width
/// * `orig_height` - Original image height
/// * `max_width` - Maximum allowed width
/// * `max_height` - Maximum allowed height
///
/// # Returns
///
/// A tuple of (width, height) for the thumbnail.
pub fn calculate_thumbnail_dimensions(
    orig_width: u32,
    orig_height: u32,
    max_width: u32,
    max_height: u32,
) -> (u32, u32) {
    let width_ratio = max_width as f32 / orig_width as f32;
    let height_ratio = max_height as f32 / orig_height as f32;
    let ratio = width_ratio.min(height_ratio);

    let new_width = (orig_width as f32 * ratio).round() as u32;
    let new_height = (orig_height as f32 * ratio).round() as u32;

    // Ensure at least 1x1
    (new_width.max(1), new_height.max(1))
}

/// Check if a MIME type is a supported image format.
///
/// Supported formats: JPEG, PNG, GIF, WebP.
///
/// # Example
///
/// ```rust
/// use underlay_media::image::is_supported_image;
///
/// assert!(is_supported_image("image/jpeg"));
/// assert!(is_supported_image("image/png"));
/// assert!(!is_supported_image("image/svg+xml"));
/// assert!(!is_supported_image("application/pdf"));
/// ```
pub fn is_supported_image(mime_type: &str) -> bool {
    matches!(
        mime_type,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp"
    )
}

/// Get the image format from a MIME type.
///
/// Returns `None` for unsupported formats.
///
/// # Example
///
/// ```rust
/// use underlay_media::image::format_from_mime;
/// use image::ImageFormat;
///
/// assert_eq!(format_from_mime("image/jpeg"), Some(ImageFormat::Jpeg));
/// assert_eq!(format_from_mime("image/png"), Some(ImageFormat::Png));
/// assert_eq!(format_from_mime("text/plain"), None);
/// ```
pub fn format_from_mime(mime_type: &str) -> Option<ImageFormat> {
    match mime_type {
        "image/jpeg" => Some(ImageFormat::Jpeg),
        "image/png" => Some(ImageFormat::Png),
        "image/gif" => Some(ImageFormat::Gif),
        "image/webp" => Some(ImageFormat::WebP),
        _ => None,
    }
}

/// Get the MIME type for an image format.
///
/// # Example
///
/// ```rust
/// use underlay_media::image::mime_from_format;
/// use image::ImageFormat;
///
/// assert_eq!(mime_from_format(ImageFormat::Jpeg), "image/jpeg");
/// assert_eq!(mime_from_format(ImageFormat::Png), "image/png");
/// ```
pub fn mime_from_format(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
#[path = "tests/image_tests.rs"]
mod tests;
