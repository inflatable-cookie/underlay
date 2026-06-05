use serde::{Deserialize, Serialize};

/// Type of rendition (derived image).
///
/// Renditions are pre-generated versions of media at different sizes
/// or formats for efficient delivery.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenditionType {
    /// Small thumbnail (typically 128-256px).
    Thumbnail,
    /// Larger preview image (typically 512-1024px).
    Preview,
    /// Custom rendition type with a specific name.
    #[serde(untagged)]
    Custom(String),
}

impl RenditionType {
    /// Get the string representation of the rendition type.
    pub fn as_str(&self) -> &str {
        match self {
            RenditionType::Thumbnail => "thumbnail",
            RenditionType::Preview => "preview",
            RenditionType::Custom(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for RenditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for RenditionType {
    fn from(s: &str) -> Self {
        match s {
            "thumbnail" => RenditionType::Thumbnail,
            "preview" => RenditionType::Preview,
            other => RenditionType::Custom(other.to_string()),
        }
    }
}

impl From<String> for RenditionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "thumbnail" => RenditionType::Thumbnail,
            "preview" => RenditionType::Preview,
            _ => RenditionType::Custom(s),
        }
    }
}
