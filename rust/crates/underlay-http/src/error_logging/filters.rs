use sqlx::{Postgres, QueryBuilder};

/// Filters for querying error log entries.
#[derive(Debug, Clone)]
pub struct ErrorLogFilters {
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    pub until: Option<chrono::DateTime<chrono::Utc>>,
    pub status_class: Option<ErrorLogStatusClass>,
    pub status_code: Option<i32>,
    pub error_code: Option<String>,
    pub endpoint: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLogStatusClass {
    Client,
    Server,
}

impl Default for ErrorLogFilters {
    fn default() -> Self {
        Self {
            since: None,
            until: None,
            status_class: None,
            status_code: None,
            error_code: None,
            endpoint: None,
            limit: 100,
            offset: 0,
        }
    }
}

pub(crate) fn push_error_log_filters<'a>(
    query: &mut QueryBuilder<'a, Postgres>,
    filters: &'a ErrorLogFilters,
) {
    if let Some(since) = filters.since {
        query.push(" AND occurred_at >= ").push_bind(since);
    }
    if let Some(until) = filters.until {
        query.push(" AND occurred_at <= ").push_bind(until);
    }
    if let Some(status_code) = filters.status_code {
        query.push(" AND status_code = ").push_bind(status_code);
    } else if let Some(status_class) = filters.status_class {
        match status_class {
            ErrorLogStatusClass::Client => {
                query.push(" AND status_code >= 400 AND status_code < 500");
            }
            ErrorLogStatusClass::Server => {
                query.push(" AND status_code >= 500 AND status_code < 600");
            }
        }
    }
    if let Some(error_code) = filters.error_code.as_deref() {
        query.push(" AND error_code = ").push_bind(error_code);
    }
    if let Some(endpoint) = filters.endpoint.as_deref() {
        query.push(" AND endpoint = ").push_bind(endpoint);
    }
}
