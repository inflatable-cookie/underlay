mod cors;
mod errors;
mod responses;

#[cfg(feature = "error-logging")]
pub mod error_logging;

#[cfg(test)]
mod errors_tests;

#[cfg(test)]
mod responses_tests;

pub use crate::cors::{cors_layer, CorsConfig};
pub use crate::errors::{error_response, ErrorLogContext, ErrorLogSink};
pub use crate::responses::{created, list_ok, no_content, ok};

#[cfg(feature = "error-logging")]
pub use crate::error_logging::{
    append_error_log, list_error_logs, DbErrorLogSink, ErrorLogFilters, ErrorLogRow,
};
