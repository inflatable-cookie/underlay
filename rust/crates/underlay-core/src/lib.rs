mod dto;
mod error;
mod id;

pub use crate::dto::{ErrorBody, ErrorEnvelope, ListResponse, SingleResponse};
pub use crate::error::{AppError, AppResult, ErrorCode};
pub use crate::id::{IdGenerator, RawUuid, SystemIdGenerator, Uuid};

#[cfg(test)]
#[path = "tests/core_tests.rs"]
mod tests;
