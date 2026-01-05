use std::collections::HashMap;
use std::sync::Arc;

use crate::JobHandler;

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
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, handler: H)
    where
        H: JobHandler + 'static,
    {
        self.handlers
            .insert(handler.job_type().to_string(), Arc::new(handler));
    }

    pub fn job_types(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }

    pub fn handler(&self, job_type: &str) -> Option<Arc<dyn JobHandler>> {
        self.handlers.get(job_type).cloned()
    }
}
