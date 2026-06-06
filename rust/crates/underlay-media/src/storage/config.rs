/// Configuration for storage key generation.
#[derive(Clone, Debug)]
pub struct StorageKeyConfig {
    /// Base prefix for all media keys (e.g., "media" or "uploads").
    base_prefix: String,
    /// Subdirectory for version files.
    versions_dir: String,
    /// Subdirectory for rendition files.
    renditions_dir: String,
    /// Default extension for rendition files.
    rendition_extension: String,
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
    pub fn with_prefix(prefix: impl AsRef<str>) -> Result<Self, underlay_blob::BlobObjectKeyError> {
        let mut config = Self::default();
        config.base_prefix = validate_path_prefix(prefix.as_ref())?;
        Ok(config)
    }

    /// Set the versions directory name.
    pub fn versions_dir(
        mut self,
        dir: impl AsRef<str>,
    ) -> Result<Self, underlay_blob::BlobObjectKeyError> {
        self.versions_dir = validate_component(dir.as_ref())?;
        Ok(self)
    }

    /// Set the renditions directory name.
    pub fn renditions_dir(
        mut self,
        dir: impl AsRef<str>,
    ) -> Result<Self, underlay_blob::BlobObjectKeyError> {
        self.renditions_dir = validate_component(dir.as_ref())?;
        Ok(self)
    }

    /// Set the default rendition extension.
    pub fn rendition_extension(
        mut self,
        ext: impl AsRef<str>,
    ) -> Result<Self, underlay_blob::BlobObjectKeyError> {
        self.rendition_extension = validate_component(ext.as_ref())?;
        Ok(self)
    }

    pub fn base_prefix(&self) -> &str {
        &self.base_prefix
    }

    pub fn versions_dir_name(&self) -> &str {
        &self.versions_dir
    }

    pub fn renditions_dir_name(&self) -> &str {
        &self.renditions_dir
    }

    pub fn rendition_extension_name(&self) -> &str {
        &self.rendition_extension
    }
}

fn validate_path_prefix(value: &str) -> Result<String, underlay_blob::BlobObjectKeyError> {
    let value = value.trim_matches('/');
    underlay_blob::validate_blob_object_key(value)?;
    Ok(value.to_string())
}

fn validate_component(value: &str) -> Result<String, underlay_blob::BlobObjectKeyError> {
    underlay_blob::validate_blob_object_key(value)?;
    if value.contains('/') {
        return Err(underlay_blob::BlobObjectKeyError::InvalidComponent);
    }
    Ok(value.to_string())
}
