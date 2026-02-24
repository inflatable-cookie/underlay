mod http_trace;
mod request_id;
mod tracing_init;

#[cfg(test)]
mod request_id_tests;

pub use crate::http_trace::trace_layer;
pub use crate::request_id::{RequestId, RequestIdLayer, REQUEST_ID_HEADER};
pub use crate::tracing_init::{
    init_tracing, init_tracing_for_env, Environment, LogFormat, ObservabilityConfig,
};

pub fn request_id_layer() -> RequestIdLayer {
    RequestIdLayer::default()
}

#[cfg(test)]
mod tests {
    use super::request_id_layer;

    #[test]
    fn request_id_layer_helper_constructs_default_layer() {
        let _layer = request_id_layer();
    }
}
