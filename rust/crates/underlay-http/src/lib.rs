pub mod caching;
pub mod context;
pub mod cookies;
mod cors;
mod errors;
mod http_config;
pub mod pagination;
mod path;
pub mod query;
mod responses;

#[cfg(feature = "error-logging")]
pub mod error_logging;

#[cfg(feature = "embed")]
pub mod embed;

#[cfg(feature = "openapi")]
pub mod openapi;

#[cfg(test)]
mod cors_tests;

#[cfg(test)]
mod caching_tests;

#[cfg(test)]
mod errors_tests;

#[cfg(all(test, feature = "error-logging"))]
mod error_logging_tests;

#[cfg(test)]
mod responses_tests;

pub use crate::caching::{
    etag_header_value, if_match_matches, if_none_match_matches, weak_etag_for_bytes, MicroCache,
    SingleFlight, CACHE_CONTROL_ADMIN_REVALIDATE, CACHE_CONTROL_NO_STORE,
};
pub use crate::context::{
    headers, AuthenticatedContext, AuthenticatedUser, ContextError, RequestContext,
};
pub use crate::cookies::{
    clear_auth_cookies, extract_refresh_token, extract_refresh_token_default, set_auth_cookies,
    AuthCookieConfig, SameSite,
};
pub use crate::cors::{cors_layer, CorsConfig, DEFAULT_CORS_MAX_AGE_SECS};
pub use crate::errors::{error_response, ApiError, ApiResult, ErrorLogContext, ErrorLogSink};
pub use crate::http_config::HttpServerConfig;
pub use crate::pagination::{Paginated, PaginationMeta, PaginationParams};
pub use crate::path::{parse_uuid_path, parse_uuid_path_raw};
pub use crate::query::{
    parse_sort_string, FieldMapping, FilterField, FilterOperator, QueryParams, SortDirection,
    SortField, WhereBuilder,
};
pub use crate::responses::{created, list_ok, no_content, ok};

#[cfg(feature = "error-logging")]
pub use crate::error_logging::{
    append_error_log, count_error_logs, error_logging_middleware, get_error_log_by_id,
    list_error_logs, DbErrorLogSink, ErrorLogFilters, ErrorLogRow, ErrorLoggingConfig,
    ERROR_CONTEXT_HEADER,
};

#[cfg(feature = "embed")]
pub use crate::embed::{
    lookup_audioboom, lookup_embed_metadata, lookup_metadata, EmbedMetaRequest, EmbedMetaResponse,
};

#[cfg(feature = "openapi")]
pub use crate::openapi::{
    ApiErrorBody, ApiErrorEnvelope, ApiListResponse, ApiSingleResponse, ApiUuid,
};
