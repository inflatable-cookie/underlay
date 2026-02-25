//! Job handler registry.

use std::collections::HashMap;
use std::sync::Arc;

use crate::types::JobHandler;

/// Registry of job handlers.
///
/// Maps job type strings to their handler implementations.
#[derive(Default, Clone)]
pub struct JobRegistry {
    handlers: HashMap<String, Arc<dyn JobHandler>>,
}

impl std::fmt::Debug for JobRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobRegistry")
            .field("job_types", &self.job_types())
            .finish()
    }
}

impl JobRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a job handler.
    ///
    /// The handler's `job_type()` is used as the key for routing jobs.
    pub fn register<H>(&mut self, handler: H)
    where
        H: JobHandler + 'static,
    {
        self.handlers
            .insert(handler.job_type().to_string(), Arc::new(handler));
    }

    /// Get all registered job types.
    pub fn job_types(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    /// Get a handler by job type.
    pub fn handler(&self, job_type: &str) -> Option<Arc<dyn JobHandler>> {
        self.handlers.get(job_type).cloned()
    }

    /// Check if a handler is registered for the given job type.
    pub fn has_handler(&self, job_type: &str) -> bool {
        self.handlers.contains_key(job_type)
    }

    /// Get the number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

#[cfg(test)]
#[path = "tests/registry_tests.rs"]
mod tests;
