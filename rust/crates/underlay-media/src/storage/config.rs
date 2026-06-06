/// Configuration for storage key generation.
#[derive(Clone, Debug)]
pub struct StorageKeyConfig {
    /// Base prefix for all media keys (e.g., "media" or "uploads").
    pub base_prefix: String,
    /// Subdirectory for version files.
    pub versions_dir: String,
    /// Subdirectory for rendition files.
    pub renditions_dir: String,
    /// Default extension for rendition files.
    pub rendition_extension: String,
}

impl Default for StorageKeyConfig {
    fn default() -> Self {
        Self {
            base_prefix: "media".to_string(),
            versions_dir: "versions".to_string(),
            renditions_dir: "renditions".to_string(),
            rendition_extension: "jpg".to_string(),
        }
    }
}

impl StorageKeyConfig {
    /// Create a new configuration with the given base prefix.
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            base_prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Set the versions directory name.
    pub fn versions_dir(mut self, dir: impl Into<String>) -> Self {
        self.versions_dir = dir.into();
        self
    }

    /// Set the renditions directory name.
    pub fn renditions_dir(mut self, dir: impl Into<String>) -> Self {
        self.renditions_dir = dir.into();
        self
    }

    /// Set the default rendition extension.
    pub fn rendition_extension(mut self, ext: impl Into<String>) -> Self {
        self.rendition_extension = ext.into();
        self
    }
}
