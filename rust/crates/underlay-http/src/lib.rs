pub mod caching;
mod cached_list;
pub mod context;
pub mod cookies;
mod cors;
mod errors;
mod http_config;
mod page_list;
pub mod pagination;
mod path;
pub mod query;
mod responses;

#[cfg(feature = "error-logging")]
pub mod error_logging;

#[cfg(feature = "error-logging")]
mod db_errors;

#[cfg(feature = "error-logging")]
pub use crate::db_errors::internal_db_error;

#[cfg(feature = "embed")]
pub mod embed;

#[cfg(feature = "openapi")]
pub mod openapi;

#[cfg(test)]
#[path = "tests/cors_tests.rs"]
mod cors_tests;

#[cfg(test)]
#[path = "tests/caching_tests.rs"]
mod caching_tests;

#[cfg(test)]
#[path = "tests/errors_tests.rs"]
mod errors_tests;

#[cfg(all(test, feature = "error-logging"))]
#[path = "tests/error_logging_tests.rs"]
mod error_logging_tests;

#[cfg(test)]
#[path = "tests/responses_tests.rs"]
mod responses_tests;

pub use crate::caching::{
    etag_header_value, if_match_matches, if_none_match_matches, weak_etag_for_bytes, MicroCache,
    SingleFlight, CACHE_CONTROL_ADMIN_REVALIDATE, CACHE_CONTROL_NO_STORE,
};
pub use crate::context::{
    headers, resolve_client_ip, AuthenticatedContext, AuthenticatedUser, ContextError,
    RequestContext, TrustedProxyConfig,
};
pub use crate::cookies::{
    clear_auth_cookies, clear_csrf_cookie, clear_csrf_token_cookie, csrf_token_cookie,
    extract_csrf_token, extract_refresh_token, extract_refresh_token_default, set_auth_cookies,
    set_csrf_cookie, AuthCookieConfig, AuthCookieError, CookieDomain, CookieName, CookiePath,
    SameSite,
};
pub use crate::cors::{
    admin_cors_config, admin_cors_layer, admin_cors_layer_from_env, cors_layer,
    cors_layer_for_env, cors_origins_from_env, try_cors_layer_for_env, CorsConfig,
    CorsConfigError, DEFAULT_CORS_MAX_AGE_SECS,
};
pub use crate::errors::{
    error_response, ApiError, ApiResult, ErrorDetail, ErrorLogContext, ErrorLogSink,
};
pub use crate::http_config::{HttpServerConfig, HttpServerConfigError};
pub use crate::cached_list::{CachedListEndpoint, CachedListResponse};
pub use crate::page_list::PageList;
#[allow(deprecated)]
pub use crate::pagination::PaginationParams;
pub use crate::pagination::{PagePaginationParams, Paginated, PaginationMeta};
pub use crate::path::{parse_uuid_path, parse_uuid_path_raw};
pub use crate::query::{
    parse_sort_string, FieldMapping, FilterField, FilterOperator, ListQueryParams, QueryParams,
    SortDirection, SortField, SqlValue, WhereBuilder,
};
// Preserve `underlay_http::field_mapping!` after the model moved to
// underlay-query.
pub use crate::responses::{created, list_ok, no_content, ok};
#[cfg(feature = "opentelemetry")]
pub use underlay_observability::{TraceContext, TRACEPARENT_HEADER, TRACESTATE_HEADER};
pub use underlay_query::field_mapping;

#[cfg(feature = "error-logging")]
pub use crate::error_logging::{
    append_error_log, count_error_logs, error_logging_middleware, get_error_log_by_id,
    list_error_logs, DbErrorLogSink, ErrorLogFilters, ErrorLogRow, ErrorLogStatusClass,
    ErrorLoggingConfig, ERROR_CONTEXT_HEADER,
};

#[cfg(feature = "embed")]
pub use crate::embed::{
    lookup_audioboom, lookup_embed_metadata, lookup_metadata, EmbedMetaRequest, EmbedMetaResponse,
};

#[cfg(feature = "openapi")]
pub use crate::openapi::{
    ApiErrorBody, ApiErrorEnvelope, ApiListResponse, ApiSingleResponse, ApiUuid,
};
