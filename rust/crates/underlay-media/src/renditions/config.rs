/// Configuration for rendition generation.
#[derive(Clone, Debug)]
pub struct RenditionConfig {
    /// Maximum dimension for thumbnail renditions.
    pub thumbnail_max_dimension: u32,
    /// Maximum dimension for preview renditions.
    pub preview_max_dimension: u32,
    /// JPEG quality for rendered images (1-100).
    pub jpeg_quality: u8,
    /// Whether to generate square (cropped) thumbnails.
    pub square_thumbnails: bool,
    /// Whether to generate thumbnails for images.
    pub generate_thumbnails: bool,
    /// Whether to generate previews for images.
    pub generate_previews: bool,
    /// Custom thumbnail rendition name (e.g., "thumb_128" for Farmyard compatibility).
    pub thumbnail_name: String,
    /// Custom preview rendition name.
    pub preview_name: String,
}

impl Default for RenditionConfig {
    fn default() -> Self {
        Self {
            thumbnail_max_dimension: 400,
            preview_max_dimension: 1200,
            jpeg_quality: 85,
            square_thumbnails: false,
            generate_thumbnails: true,
            generate_previews: false,
            thumbnail_name: "thumb".to_string(),
            preview_name: "preview".to_string(),
        }
    }
}

impl RenditionConfig {
    /// Create a config with only thumbnails enabled.
    pub fn thumbnails_only() -> Self {
        Self {
            generate_thumbnails: true,
            generate_previews: false,
            ..Default::default()
        }
    }

    /// Create a config with both thumbnails and previews enabled.
    pub fn with_previews() -> Self {
        Self {
            generate_thumbnails: true,
            generate_previews: true,
            ..Default::default()
        }
    }

    /// Create a farmyard-compatible config with 128x128 square thumbnails.
    ///
    /// This matches the default Farmyard configuration:
    /// - 128px square thumbnails
    /// - Quality 80
    /// - Rendition name "thumb_128"
    pub fn farmyard_compat() -> Self {
        Self {
            thumbnail_max_dimension: 128,
            jpeg_quality: 80,
            square_thumbnails: true,
            generate_thumbnails: true,
            generate_previews: false,
            thumbnail_name: "thumb_128".to_string(),
            ..Default::default()
        }
    }

    /// Set the thumbnail maximum dimension.
    pub fn thumbnail_size(mut self, max_dim: u32) -> Self {
        self.thumbnail_max_dimension = max_dim;
        self
    }

    /// Set the preview maximum dimension.
    pub fn preview_size(mut self, max_dim: u32) -> Self {
        self.preview_max_dimension = max_dim;
        self
    }

    /// Set the JPEG quality.
    pub fn quality(mut self, quality: u8) -> Self {
        self.jpeg_quality = quality.clamp(1, 100);
        self
    }

    /// Enable square (center-cropped) thumbnails.
    pub fn square(mut self) -> Self {
        self.square_thumbnails = true;
        self
    }

    /// Set custom thumbnail rendition name.
    pub fn thumbnail_name(mut self, name: impl Into<String>) -> Self {
        self.thumbnail_name = name.into();
        self
    }

    /// Set custom preview rendition name.
    pub fn preview_name(mut self, name: impl Into<String>) -> Self {
        self.preview_name = name.into();
        self
    }
}
