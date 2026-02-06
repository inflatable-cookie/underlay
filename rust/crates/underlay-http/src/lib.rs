pub mod context;
pub mod cookies;
mod cors;
mod errors;
mod field_validation;
mod http_config;
pub mod pagination;
mod path;
pub mod query;
mod responses;

#[cfg(feature = "validation")]
mod validation;

#[cfg(feature = "nightfire")]
mod nightfire;

#[cfg(feature = "error-logging")]
pub mod error_logging;

#[cfg(feature = "embed")]
pub mod embed;

#[cfg(test)]
mod cors_tests;

#[cfg(test)]
mod errors_tests;

#[cfg(test)]
mod responses_tests;

pub use crate::context::{
    headers, AuthenticatedContext, AuthenticatedUser, ContextError, RequestContext,
};
pub use crate::cookies::{
    clear_auth_cookies, extract_refresh_token, extract_refresh_token_default, set_auth_cookies,
    AuthCookieConfig, SameSite,
};
pub use crate::cors::{cors_layer, CorsConfig, DEFAULT_CORS_MAX_AGE_SECS};
pub use crate::errors::{error_response, ApiError, ApiResult, ErrorLogContext, ErrorLogSink};
pub use crate::field_validation::{
    parse_optional_uuid_for_validation, parse_uuid_for_validation, ValidationResult,
};
pub use crate::pagination::{Paginated, PaginationMeta, PaginationParams};
pub use crate::path::{parse_uuid_path, parse_uuid_path_raw};
pub use crate::query::{
    parse_sort_string, FieldMapping, FilterField, FilterOperator, QueryParams, SortDirection,
    SortField, WhereBuilder,
};
pub use crate::responses::{created, list_ok, no_content, ok};
pub use crate::http_config::HttpServerConfig;

#[cfg(feature = "validation")]
pub use crate::validation::{validation_to_app_error, ValidateExt};

#[cfg(feature = "nightfire")]
pub use crate::nightfire::nightfire_validation_to_app_error;

#[cfg(feature = "error-logging")]
pub use crate::error_logging::{
    append_error_log, count_error_logs, error_logging_middleware, error_response_with_context,
    get_error_log_by_id, list_error_logs, DbErrorLogSink, ErrorLogFilters, ErrorLoggingConfig,
    ErrorLogRow, ERROR_CONTEXT_HEADER,
};

#[cfg(feature = "embed")]
pub use crate::embed::{
    lookup_audioboom, lookup_embed_metadata, lookup_metadata, EmbedMetaRequest, EmbedMetaResponse,
};
