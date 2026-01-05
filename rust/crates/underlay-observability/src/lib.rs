mod http_trace;
mod request_id;
mod tracing_init;

#[cfg(test)]
mod request_id_tests;

pub use crate::http_trace::trace_layer;
pub use crate::request_id::{RequestId, RequestIdLayer, REQUEST_ID_HEADER};
pub use crate::tracing_init::{init_tracing, LogFormat, ObservabilityConfig};

pub fn request_id_layer() -> RequestIdLayer {
    RequestIdLayer::default()
}
