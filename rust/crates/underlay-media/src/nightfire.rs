//! Shared Nightfire media-usage extraction helpers.

mod context;
mod extractor;
mod matcher;
mod registry;
mod resolver;
mod walk;

pub use context::NightfireMediaVisitContext;
pub use extractor::{NightfireBlockMediaUsageExtractor, NightfireMediaUsageExtractor};
pub use matcher::{
    NightfireFieldNameMatcher, NightfireMediaFieldRule, NightfireMediaReferenceMatch,
    NightfireMediaReferenceMatcher,
};
pub use registry::{
    NightfireBlockMediaHandler, NightfireBlockMediaHandlerMap, NightfireBlockMediaHandlerRegistry,
    NightfireBlockMediaReference, NightfireBlockMediaRegistration, NightfireNestedValuePointer,
};
pub use resolver::resolve_nightfire_media_usage;

#[cfg(test)]
#[path = "tests/nightfire_tests.rs"]
mod tests;
