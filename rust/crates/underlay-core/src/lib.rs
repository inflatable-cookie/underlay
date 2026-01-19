mod dto;
mod error;
mod id;
mod slugify;

pub use crate::dto::{ErrorBody, ErrorEnvelope, ListResponse, SingleResponse};
pub use crate::error::{AppError, AppResult};
pub use crate::id::{IdGenerator, RawUuid, SystemIdGenerator, Uuid};
pub use crate::slugify::{
    is_reserved_slug, is_valid_slug_format, slugify, validate_slug, SlugValidationError,
    RESERVED_SLUGS,
};

#[cfg(test)]
mod tests;
