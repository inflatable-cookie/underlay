use underlay_core::Uuid;

/// Identifier type for artists.
///
/// Uses UUIDv7 for time-ordered, lexicographically sortable identifiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtistId(pub Uuid);

impl ArtistId {
    /// Create a new artist ID with a fresh UUIDv7.
    pub fn new() -> Self {
        Self(Uuid::new_v7())
    }

    /// Create from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID.
    pub fn into_uuid(self) -> Uuid {
        self.0
    }

    /// Get a reference to the underlying UUID.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Parse from a string.
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        s.parse().map(Self)
    }
}

impl From<Uuid> for ArtistId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ArtistId> for Uuid {
    fn from(value: ArtistId) -> Self {
        value.0
    }
}

impl From<ArtistId> for String {
    fn from(value: ArtistId) -> Self {
        value.0.to_string()
    }
}

impl std::fmt::Display for ArtistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ArtistId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

/// Result type for core operations.
pub type CoreResult<T> = Result<T, CoreError>;

/// Error type for core operations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("ID parsing error: {0}")]
    IdParse(#[from] uuid::Error),

    #[error("Invalid ID format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artist_id_generates_v7_uuid() {
        let id = ArtistId::new();
        assert!(!id.0.as_uuid().is_nil());

        // UUIDv7 has timestamp in the first 48 bits
        let ts = id.0.as_uuid().to_u128() >> 80;
        assert!(ts > 0);
    }

    #[test]
    fn artist_id_display_and_parse() {
        let id = ArtistId::new();
        let s = id.to_string();
        let parsed = ArtistId::parse(&s).unwrap();
        assert_eq!(id, parsed);
    }
}
