pub mod context;
pub mod cookies;
mod cors;
mod errors;
pub mod pagination;
mod path;
pub mod query;
mod responses;

#[cfg(feature = "validation")]
mod validation;

#[cfg(feature = "error-logging")]
pub mod error_logging;

#[cfg(test)]
mod errors_tests;

#[cfg(test)]
mod responses_tests;

pub use crate::context::{
    headers, AuthenticatedContext, AuthenticatedUser, ContextError, RequestContext,
};
pub use crate::cookies::{
    clear_auth_cookies, extract_refresh_token, set_auth_cookies, AuthCookieConfig,
};
pub use crate::cors::{cors_layer, CorsConfig};
pub use crate::errors::{error_response, ErrorLogContext, ErrorLogSink};
pub use crate::pagination::{Paginated, PaginationMeta, PaginationParams};
pub use crate::query::{
    parse_sort_string, FilterField, FilterOperator, QueryParams, SortDirection, SortField,
    WhereBuilder,
};
pub use crate::path::{parse_uuid_path, parse_uuid_path_raw};
pub use crate::responses::{created, list_ok, no_content, ok};

#[cfg(feature = "validation")]
pub use crate::validation::{validation_to_app_error, ValidateExt};

#[cfg(feature = "error-logging")]
pub use crate::error_logging::{
    append_error_log, list_error_logs, DbErrorLogSink, ErrorLogFilters, ErrorLogRow,
};
