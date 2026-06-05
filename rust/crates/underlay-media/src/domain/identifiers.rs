use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed identifier for a Media item.
///
/// Media items are stable references that can have multiple versions.
/// Other entities should reference media by this ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaId(pub Uuid);

impl MediaId {
    /// Create a new random MediaId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Strongly-typed identifier for a MediaVersion.
///
/// Each version represents an immutable snapshot of the media content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaVersionId(pub Uuid);

impl MediaVersionId {
    /// Create a new random MediaVersionId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaVersionId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaVersionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaVersionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaVersionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Strongly-typed identifier for a MediaRendition.
///
/// Renditions are derived blobs (thumbnails, previews) generated from versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MediaRenditionId(pub Uuid);

impl MediaRenditionId {
    /// Create a new random MediaRenditionId using UUID v7.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Create a MediaRenditionId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for MediaRenditionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MediaRenditionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MediaRenditionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
