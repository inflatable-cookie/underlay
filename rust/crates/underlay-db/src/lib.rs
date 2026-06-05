mod db_errors;
mod existence;
mod identifiers;
pub mod media_types;
mod migrations;
pub mod pagination;
mod pool;
mod schemas;
mod sql_dir;

pub use crate::db_errors::{describe_db_error, map_db_error, map_db_error_ref};
pub use crate::existence::{
    number_exists_in_scope, number_exists_in_scope_excluding, value_exists, value_exists_excluding,
    value_exists_excluding_typed, value_exists_in_scope, value_exists_in_scope_excluding,
    value_exists_typed, ExistsCheck,
};
pub use crate::identifiers::{
    format_qualified_table_name, format_schema_table, quote_sql_identifier,
    validate_qualified_table_name, validate_sql_identifier, IdentifierError, QualifiedTableName,
    SqlIdentifier,
};
pub use crate::migrations::{load_migrator_from_dir, run_migrations};
pub use crate::pool::{
    create_pool, DbConfig, DbPool, DEFAULT_ACQUIRE_TIMEOUT_SECS, DEFAULT_IDLE_TIMEOUT_SECS,
    DEFAULT_MAX_CONNECTIONS, DEFAULT_MIN_CONNECTIONS,
};
pub use crate::schemas::{
    drop_schema_identifiers, drop_schemas, parse_schema_name, validate_schema_name,
    DestructiveGuard,
};
pub use crate::sql_dir::{run_sql_dir, run_sql_dir_with_options, SqlDirOptions};

// Media types
pub use crate::media_types::{
    detect_media_kind_from_mime_type, MediaKind, MediaTypeParseError, MediaVersionState,
    MediaVisibility,
};
