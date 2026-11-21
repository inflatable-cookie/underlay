use serde::{Deserialize, Serialize};
use uuid::Uuid as InnerUuid;

// Re-export the raw uuid type for interoperability with crates that use it directly
pub use uuid::Uuid as RawUuid;

/// Canonical UUID type for Underlay-based projects (UUIDv7).
///
/// This is a thin newtype wrapper around `uuid::Uuid` so that
/// we can centralise generation and parsing rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uuid(pub InnerUuid);

impl Uuid {
    /// Generate a new UUIDv7 value.
    pub fn new_v7() -> Self {
        Uuid(InnerUuid::now_v7())
    }

    /// Parse a UUID from its string representation.
    pub fn parse_str(input: &str) -> Result<Self, uuid::Error> {
        InnerUuid::parse_str(input).map(Uuid)
    }

    /// Get the inner `uuid::Uuid`.
    pub fn into_inner(self) -> InnerUuid {
        self.0
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RawUuid> for Uuid {
    fn from(id: RawUuid) -> Self {
        Self(id)
    }
}

impl From<Uuid> for RawUuid {
    fn from(id: Uuid) -> Self {
        id.0
    }
}

impl AsRef<RawUuid> for Uuid {
    fn as_ref(&self) -> &RawUuid {
        &self.0
    }
}

impl std::str::FromStr for Uuid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s)
    }
}

/// Trait for components that can generate UUIDv7 identifiers.
pub trait IdGenerator: Send + Sync {
    fn new_uuid(&self) -> Uuid;
}

/// Default UUIDv7 generator.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemIdGenerator;

impl IdGenerator for SystemIdGenerator {
    fn new_uuid(&self) -> Uuid {
        Uuid::new_v7()
    }
}
