mod http_trace;
mod request_id;
#[cfg(feature = "opentelemetry")]
mod trace_context;
mod tracing_init;

#[cfg(test)]
#[path = "tests/request_id_tests.rs"]
mod request_id_tests;

pub use crate::http_trace::trace_layer;
pub use crate::request_id::{RequestId, RequestIdLayer, REQUEST_ID_HEADER};
#[cfg(feature = "opentelemetry")]
pub use crate::trace_context::{TraceContext, TRACEPARENT_HEADER, TRACESTATE_HEADER};
pub use crate::tracing_init::{
    init_tracing, init_tracing_for_env, Environment, LogFormat, ObservabilityConfig,
};

pub fn request_id_layer() -> RequestIdLayer {
    RequestIdLayer::default()
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
