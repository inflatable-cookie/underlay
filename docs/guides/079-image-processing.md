# Image Processing

Underlay provides image processing utilities through the `underlay-image` crate for generating thumbnails and working with common image formats.

## Overview

The image processing module supports:

- **Thumbnail generation** - Aspect-ratio-preserving resize
- **Square thumbnails** - Center-cropped squares for avatars
- **Multiple input formats** - JPEG, PNG, GIF, WebP
- **Configurable quality** - Control JPEG output quality
- **Preset sizes** - Small, medium, large, and extra-large presets

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
underlay-image = { path = "../underlay/rust/crates/underlay-image" }
```

## Basic Usage

### Generating Thumbnails

Use `generate_thumbnail()` to create aspect-ratio-preserving thumbnails:

```rust
use underlay_image::{generate_thumbnail, ThumbnailConfig};

// Load image data from somewhere
let image_bytes: Vec<u8> = std::fs::read("photo.jpg")?;

// Generate a medium-sized thumbnail (256x256 max)
let config = ThumbnailConfig::medium();
let result = generate_thumbnail(&image_bytes, &config)?;

// Use the result
println!("Thumbnail dimensions: {}x{}", result.width, result.height);
println!("Output size: {} bytes", result.data.len());

// Save or upload result.data
std::fs::write("thumbnail.jpg", &result.data)?;
```

### Configuration Presets

Use built-in presets for common thumbnail sizes:

```rust
use underlay_image::ThumbnailConfig;

// Small: 128x128, quality 80 - for list views
let small = ThumbnailConfig::small();

// Medium: 256x256, quality 85 - balanced default
let medium = ThumbnailConfig::medium();

// Large: 512x512, quality 85 - for detail views
let large = ThumbnailConfig::large();

// Extra-large: 1024x1024, quality 90 - for hero images
let xlarge = ThumbnailConfig::xlarge();
```

### Custom Configuration

Create custom configurations for specific needs:

```rust
use underlay_image::ThumbnailConfig;

// Custom dimensions with builder pattern
let config = ThumbnailConfig::new(400, 300)
    .with_quality(90);

// Or set fields directly
let config = ThumbnailConfig {
    max_width: 800,
    max_height: 600,
    quality: 85,
};
```

### Square Thumbnails

For avatars and profile pictures, use center-cropped squares:

```rust
use underlay_image::generate_square_thumbnail;

let image_bytes = std::fs::read("photo.jpg")?;

// Generate a 128x128 avatar
let result = generate_square_thumbnail(&image_bytes, 128, 85)?;

assert_eq!(result.width, 128);
assert_eq!(result.height, 128);
```

The function:
1. Scales the image so the smaller dimension matches the target size
2. Center-crops to produce a perfect square
3. Outputs as JPEG

## Media Library Integration

When integrating with a media library, generate multiple renditions on upload:

```rust
use underlay_image::{generate_thumbnail, generate_square_thumbnail, ThumbnailConfig};

struct MediaRendition {
    name: &'static str,
    data: Vec<u8>,
    width: u32,
    height: u32,
    mime_type: &'static str,
}

fn generate_renditions(source_data: &[u8]) -> Result<Vec<MediaRendition>, underlay_image::ImageError> {
    let mut renditions = Vec::new();

    // Standard thumbnails
    for (name, config) in [
        ("thumb_small", ThumbnailConfig::small()),
        ("thumb_medium", ThumbnailConfig::medium()),
        ("thumb_large", ThumbnailConfig::large()),
    ] {
        let result = generate_thumbnail(source_data, &config)?;
        renditions.push(MediaRendition {
            name,
            data: result.data,
            width: result.width,
            height: result.height,
            mime_type: result.mime_type,
        });
    }

    // Square avatar thumbnail
    let avatar = generate_square_thumbnail(source_data, 256, 85)?;
    renditions.push(MediaRendition {
        name: "avatar",
        data: avatar.data,
        width: avatar.width,
        height: avatar.height,
        mime_type: avatar.mime_type,
    });

    Ok(renditions)
}
```

## Background Job Processing

For production systems, generate renditions in a background job:

```rust
use underlay_image::{generate_thumbnail, ThumbnailConfig, is_supported_image};

async fn process_media_upload(
    media_id: Uuid,
    source_data: Vec<u8>,
    mime_type: &str,
) -> Result<(), JobError> {
    // Only process supported image formats
    if !is_supported_image(mime_type) {
        return Ok(()); // Skip non-images
    }

    // Generate renditions
    let configs = [
        ("small", ThumbnailConfig::small()),
        ("medium", ThumbnailConfig::medium()),
        ("large", ThumbnailConfig::large()),
    ];

    for (name, config) in configs {
        let result = generate_thumbnail(&source_data, &config)?;

        // Upload rendition to storage
        let key = format!("media/{}/renditions/{}.jpg", media_id, name);
        storage.upload(&key, &result.data, result.mime_type).await?;

        // Record rendition in database
        db::create_rendition(media_id, name, result.width, result.height).await?;
    }

    Ok(())
}
```

## Utility Functions

### Checking Supported Formats

```rust
use underlay_image::is_supported_image;

fn can_generate_thumbnail(mime_type: &str) -> bool {
    is_supported_image(mime_type)
}

assert!(can_generate_thumbnail("image/jpeg"));
assert!(can_generate_thumbnail("image/png"));
assert!(!can_generate_thumbnail("application/pdf"));
```

### Converting Between Formats and MIME Types

```rust
use underlay_image::{format_from_mime, mime_from_format};
use image::ImageFormat;

// MIME to format
let format = format_from_mime("image/png");
assert_eq!(format, Some(ImageFormat::Png));

// Format to MIME
let mime = mime_from_format(ImageFormat::Jpeg);
assert_eq!(mime, "image/jpeg");
```

### Dimension Calculation

Calculate thumbnail dimensions without processing:

```rust
use underlay_image::calculate_thumbnail_dimensions;

// Landscape image into 256x256 box
let (w, h) = calculate_thumbnail_dimensions(1920, 1080, 256, 256);
assert_eq!((w, h), (256, 144)); // Maintains 16:9 ratio

// Portrait image into 256x256 box
let (w, h) = calculate_thumbnail_dimensions(1080, 1920, 256, 256);
assert_eq!((w, h), (144, 256)); // Maintains 9:16 ratio
```

## Best Practices

### 1. Generate Multiple Sizes

Store multiple rendition sizes to serve appropriate images for different contexts:

| Size | Use Case | Typical Dimensions |
|------|----------|-------------------|
| Small | List views, grids | 128x128 |
| Medium | Cards, previews | 256x256 |
| Large | Detail views | 512x512 |
| XLarge | Hero images, lightbox | 1024x1024 |

### 2. Process Asynchronously

Generate thumbnails in background jobs rather than during upload requests:

```rust
// During upload: just store the original
let media_id = storage.upload_original(&file_data).await?;

// Enqueue background job for rendition generation
job_queue.enqueue(GenerateRenditionsJob { media_id }).await?;

// Return immediately to user
Ok(MediaUploadResponse { id: media_id, status: "processing" })
```

### 3. Handle Errors Gracefully

Not all images can be processed. Handle errors without breaking the upload flow:

```rust
match generate_thumbnail(&data, &config) {
    Ok(result) => {
        // Store the thumbnail
    }
    Err(e) => {
        tracing::warn!(%media_id, %e, "Failed to generate thumbnail");
        // Continue without thumbnail - mark media as needing manual review
    }
}
```

### 4. Consider Output Size

JPEG quality affects file size significantly:

| Quality | Typical File Size | Use Case |
|---------|------------------|----------|
| 70-80 | Small | Thumbnails, previews |
| 80-85 | Medium | General use |
| 85-95 | Large | High-quality displays |

### 5. Validate Before Processing

Check MIME type and file size before attempting to process:

```rust
const MAX_IMAGE_SIZE: usize = 20 * 1024 * 1024; // 20MB

fn validate_image_upload(data: &[u8], mime_type: &str) -> Result<(), ValidationError> {
    if !is_supported_image(mime_type) {
        return Err(ValidationError::UnsupportedFormat);
    }

    if data.len() > MAX_IMAGE_SIZE {
        return Err(ValidationError::FileTooLarge);
    }

    Ok(())
}
```

## Error Handling

The `ImageError` enum covers processing failures:

```rust
use underlay_image::ImageError;

match generate_thumbnail(&data, &config) {
    Ok(result) => { /* success */ }
    Err(ImageError::DecodeError(msg)) => {
        // Invalid or corrupted image data
        tracing::error!("Failed to decode image: {}", msg);
    }
    Err(ImageError::EncodeError(msg)) => {
        // Failed to encode output (rare)
        tracing::error!("Failed to encode thumbnail: {}", msg);
    }
    Err(ImageError::UnsupportedFormat(msg)) => {
        // Format not supported
        tracing::warn!("Unsupported image format: {}", msg);
    }
}
```

## Performance Considerations

- **Memory**: Large images require significant memory during processing
- **CPU**: Lanczos3 resampling is high-quality but slower than simpler filters
- **Parallelism**: Generate multiple renditions concurrently using `tokio::spawn`

```rust
use tokio::task::spawn_blocking;

// Process in blocking thread pool to avoid blocking async runtime
let result = spawn_blocking(move || {
    generate_thumbnail(&data, &config)
}).await??;
```
