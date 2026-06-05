use underlay_blob::BlobObjectKey;

/// Result of generating a rendition.
#[derive(Debug)]
pub struct RenditionResult {
    /// Object key where the rendition was stored.
    pub object_key: BlobObjectKey,
    /// MIME type of the rendition.
    pub mime_type: String,
    /// Size in bytes.
    pub byte_size: i64,
    /// Width in pixels.
    pub width: i32,
    /// Height in pixels.
    pub height: i32,
}
