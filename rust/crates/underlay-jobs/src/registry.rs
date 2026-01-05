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

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::{Job, JobHandler, JobHandlerError, JobRegistry};

    #[derive(Debug)]
    struct TestHandler;

    #[async_trait]
    impl JobHandler for TestHandler {
        fn job_type(&self) -> &'static str {
            "test"
        }

        async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
            Ok(())
        }
    }

    #[test]
    fn registry_tracks_registered_job_types_and_handler_lookup() {
        let mut registry = JobRegistry::new();
        registry.register(TestHandler);

        let types = registry.job_types();
        assert_eq!(types, vec!["test".to_string()]);

        assert!(registry.handler("test").is_some());
        assert!(registry.handler("missing").is_none());
    }
}
