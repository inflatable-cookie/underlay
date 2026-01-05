mod registry;
mod serve;

pub use crate::registry::{
    default_registry, register_build_info, register_default_metrics,
    register_default_metrics_with_build_info, BuildInfo, DefaultRegistry,
};
pub use crate::serve::metrics_handler;
