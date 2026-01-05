mod registry;
mod serve;

pub use crate::registry::{default_registry, register_default_metrics, DefaultRegistry};
pub use crate::serve::metrics_handler;
