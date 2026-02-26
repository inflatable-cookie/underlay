use async_trait::async_trait;

use crate::types::{Job, JobHandler, JobHandlerError};
use crate::JobRegistry;

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

#[test]
fn registry_has_handler() {
    let mut registry = JobRegistry::new();
    assert!(!registry.has_handler("test"));

    registry.register(TestHandler);
    assert!(registry.has_handler("test"));
}

#[test]
fn registry_len_and_is_empty() {
    let mut registry = JobRegistry::new();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);

    registry.register(TestHandler);
    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 1);
}
