use std::collections::HashMap;
use std::sync::Arc;

use crate::LlmClient;

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    clients: HashMap<String, Arc<dyn LlmClient>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider_key: impl Into<String>, client: Arc<dyn LlmClient>) {
        self.clients.insert(provider_key.into(), client);
    }

    pub fn get(&self, provider_key: &str) -> Option<Arc<dyn LlmClient>> {
        self.clients.get(provider_key).cloned()
    }
}
