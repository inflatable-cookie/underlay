use std::collections::BTreeMap;
use std::sync::Arc;

use underlay_nightfire::BlockRegistration;

use crate::domain::{MediaId, MediaUsageRole};
use crate::error::MediaResult;

use super::NightfireMediaVisitContext;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireBlockMediaReference {
    pub media_id: MediaId,
    pub usage_role: MediaUsageRole,
    /// JSON Pointer relative to `block.data`.
    pub data_pointer: String,
}

impl NightfireBlockMediaReference {
    pub fn new(
        media_id: MediaId,
        usage_role: MediaUsageRole,
        data_pointer: impl Into<String>,
    ) -> Self {
        Self {
            media_id,
            usage_role,
            data_pointer: data_pointer.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireNestedValuePointer {
    /// JSON Pointer relative to `block.data`.
    pub data_pointer: String,
}

impl NightfireNestedValuePointer {
    pub fn new(data_pointer: impl Into<String>) -> Self {
        Self {
            data_pointer: data_pointer.into(),
        }
    }
}

pub trait NightfireBlockMediaHandler: Send + Sync {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireBlockMediaReference>>;

    fn nested_nightfire_values(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireNestedValuePointer>> {
        Ok(Vec::new())
    }
}

pub trait NightfireBlockMediaHandlerRegistry: Send + Sync {
    fn handler_for(&self, block_type: &str) -> Option<&dyn NightfireBlockMediaHandler>;
}

#[derive(Clone)]
pub struct NightfireBlockMediaRegistration {
    pub block_type: String,
    pub handler: Arc<dyn NightfireBlockMediaHandler>,
}

impl NightfireBlockMediaRegistration {
    pub fn new(
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) -> Self {
        Self {
            block_type: block_type.into(),
            handler: Arc::new(handler),
        }
    }
}

#[derive(Default)]
pub struct NightfireBlockMediaHandlerMap {
    handlers: BTreeMap<String, Arc<dyn NightfireBlockMediaHandler>>,
}

impl NightfireBlockMediaHandlerMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_registrations(
        registrations: impl IntoIterator<Item = NightfireBlockMediaRegistration>,
    ) -> Self {
        let mut map = Self::new();
        map.extend_registrations(registrations);
        map
    }

    pub fn from_block_registrations<C>(
        registrations: impl IntoIterator<Item = BlockRegistration<C, NightfireBlockMediaRegistration>>,
    ) -> Self {
        let mut map = Self::new();
        map.extend_block_registrations(registrations);
        map
    }

    pub fn with_handler(
        mut self,
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) -> Self {
        self.register(block_type, handler);
        self
    }

    pub fn register(
        &mut self,
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) {
        self.handlers.insert(block_type.into(), Arc::new(handler));
    }

    pub fn with_registration(mut self, registration: NightfireBlockMediaRegistration) -> Self {
        self.register_registration(registration);
        self
    }

    pub fn register_registration(&mut self, registration: NightfireBlockMediaRegistration) {
        self.handlers
            .insert(registration.block_type, registration.handler);
    }

    pub fn extend_registrations(
        &mut self,
        registrations: impl IntoIterator<Item = NightfireBlockMediaRegistration>,
    ) {
        for registration in registrations {
            self.register_registration(registration);
        }
    }

    pub fn register_block_registration<C>(
        &mut self,
        registration: BlockRegistration<C, NightfireBlockMediaRegistration>,
    ) {
        let (_descriptor, metadata) = registration.into_parts();
        self.register_registration(metadata);
    }

    pub fn extend_block_registrations<C>(
        &mut self,
        registrations: impl IntoIterator<Item = BlockRegistration<C, NightfireBlockMediaRegistration>>,
    ) {
        for registration in registrations {
            self.register_block_registration(registration);
        }
    }
}

impl NightfireBlockMediaHandlerRegistry for NightfireBlockMediaHandlerMap {
    fn handler_for(&self, block_type: &str) -> Option<&dyn NightfireBlockMediaHandler> {
        self.handlers
            .get(block_type)
            .map(|handler| handler.as_ref() as &dyn NightfireBlockMediaHandler)
    }
}
