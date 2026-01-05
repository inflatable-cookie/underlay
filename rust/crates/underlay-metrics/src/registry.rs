use std::collections::HashMap;

use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};

/// A small wrapper to standardise how apps expose their Prometheus registry.
#[derive(Debug, Clone)]
pub struct DefaultRegistry {
    registry: Registry,
}

impl DefaultRegistry {
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
        }
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn register_default_metrics(&self, namespace: &str) -> Result<(), prometheus::Error> {
        register_default_metrics(&self.registry, namespace)
    }

    pub fn gather_text(&self) -> Result<String, prometheus::Error> {
        let metric_families = self.registry.gather();
        let mut buf = Vec::new();
        let encoder = TextEncoder::new();
        encoder.encode(&metric_families, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

/// Register a small set of "always useful" metrics.
///
/// - On Linux (with Prometheus' `process` feature), registers process metrics.
/// - Always registers a `build_info` gauge with constant labels.
pub fn register_default_metrics(
    registry: &Registry,
    namespace: &str,
) -> Result<(), prometheus::Error> {
    let mut labels = HashMap::new();
    labels.insert("name".to_string(), env!("CARGO_PKG_NAME").to_string());
    labels.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    let build_info = Gauge::with_opts(
        Opts::new("build_info", "Build metadata for this service")
            .namespace(namespace)
            .const_labels(labels),
    )?;

    build_info.set(1.0);
    registry.register(Box::new(build_info))?;

    #[cfg(all(feature = "process", target_os = "linux"))]
    {
        registry.register(Box::new(
            prometheus::process_collector::ProcessCollector::for_self(),
        ))?;
    }

    Ok(())
}

pub fn default_registry() -> DefaultRegistry {
    DefaultRegistry::new()
}
