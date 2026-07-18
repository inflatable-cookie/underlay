mod db_errors;
mod existence;
mod identifiers;
mod migrations;
pub mod pagination;
mod pool;
mod schemas;
mod sql_dir;

pub use crate::db_errors::{describe_db_error, map_db_error, map_db_error_ref};
pub use crate::existence::{value_exists_excluding_typed, value_exists_typed, TypedExistsCheck};
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
