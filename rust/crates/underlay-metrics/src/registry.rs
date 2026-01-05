use std::collections::HashMap;

use prometheus::{Encoder, Gauge, Opts, Registry, TextEncoder};

/// Build metadata for this service.
#[derive(Debug, Clone, Copy)]
pub struct BuildInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
}

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
}

impl Default for DefaultRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRegistry {
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn register_default_metrics(&self, namespace: &str) -> Result<(), prometheus::Error> {
        register_default_metrics(&self.registry, namespace)
    }

    pub fn register_default_metrics_with_build_info(
        &self,
        namespace: &str,
        build_info: BuildInfo<'_>,
    ) -> Result<(), prometheus::Error> {
        register_default_metrics_with_build_info(&self.registry, namespace, build_info)
    }

    pub fn gather_text(&self) -> Result<String, prometheus::Error> {
        let metric_families = self.registry.gather();
        let mut buf = Vec::new();
        let encoder = TextEncoder::new();
        encoder.encode(&metric_families, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

/// Register a `build_info` gauge with constant labels.
pub fn register_build_info(
    registry: &Registry,
    namespace: &str,
    build_info: BuildInfo<'_>,
) -> Result<(), prometheus::Error> {
    let mut labels = HashMap::new();
    labels.insert("name".to_string(), build_info.name.to_string());
    labels.insert("version".to_string(), build_info.version.to_string());

    let build_info = Gauge::with_opts(
        Opts::new("build_info", "Build metadata for this service")
            .namespace(namespace)
            .const_labels(labels),
    )?;

    build_info.set(1.0);
    registry.register(Box::new(build_info))?;

    Ok(())
}

/// Register a small set of "always useful" metrics.
///
/// - On Linux (with Prometheus' `process` feature), registers process metrics.
/// - Always registers a `build_info` gauge with constant labels.
///
/// For accurate `build_info` values for the calling service, prefer
/// `register_default_metrics_with_build_info`.
pub fn register_default_metrics(
    registry: &Registry,
    namespace: &str,
) -> Result<(), prometheus::Error> {
    register_default_metrics_with_build_info(
        registry,
        namespace,
        BuildInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        },
    )
}

/// Like `register_default_metrics`, but lets the application provide the build metadata.
pub fn register_default_metrics_with_build_info(
    registry: &Registry,
    namespace: &str,
    build_info: BuildInfo<'_>,
) -> Result<(), prometheus::Error> {
    register_build_info(registry, namespace, build_info)?;

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
