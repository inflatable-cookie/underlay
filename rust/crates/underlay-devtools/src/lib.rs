pub(crate) mod migration_bundle;
mod migration_report;
mod seed_bundle;
mod sync_migrations;

pub use migration_bundle::{
    migration_bundle_build, migration_bundle_publish, migration_bundle_pull, migration_run,
    BundleBuildOptions, BundleBuildReport, BundlePublishOptions, BundlePublishReport,
    BundlePullOptions, BundlePullReport, BundleRunOptions, BundleRunReport, MigrationBundleError,
    MigrationBundleRef,
};
pub use migration_report::{
    build_audit_report, build_drift_report, build_drift_report_with_lineage,
    build_integrity_report, build_policy_report, build_recovery_advisories,
    build_verification_report, format_audit_summary, format_decision_governance_report,
    format_decision_invalidation_report, format_drift_category_summary, format_drift_report,
    format_integrity_summary, format_policy_summary, format_recovery_advisories,
    format_verification_summary, load_decide_stage_output, load_decision_index,
    load_decision_journal, load_governance_policy, load_pipeline_run_report,
    load_pipeline_run_report_from_path, top_governance_issues, write_audit_artifact,
    write_verification_artifact, MigrationReportError,
};
pub use seed_bundle::{
    seed_bundle_build, seed_bundle_publish, seed_bundle_pull, SeedBundleBuildOptions,
    SeedBundleBuildReport, SeedBundlePullOptions, SeedBundlePullReport,
};
pub use sync_migrations::{sync_migrations, SyncMigrationsError, SyncMigrationsReport};

use std::future::Future;
use std::pin::Pin;

use underlay_db::{DbConfig, DbPool, DestructiveGuard};

type MigrateFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), sqlx::migrate::MigrateError>> + Send + 'a>>;

#[derive(Debug)]
pub enum DevtoolError {
    MissingEnvVar { name: &'static str },
    Db(sqlx::Error),
    Migrate(sqlx::migrate::MigrateError),
}

impl std::fmt::Display for DevtoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DevtoolError::MissingEnvVar { name } => {
                write!(f, "{name} must be set to run this command")
            }
            DevtoolError::Db(err) => write!(f, "{err}"),
            DevtoolError::Migrate(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for DevtoolError {}

impl From<sqlx::Error> for DevtoolError {
    fn from(value: sqlx::Error) -> Self {
        Self::Db(value)
    }
}

impl From<sqlx::migrate::MigrateError> for DevtoolError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::Migrate(value)
    }
}

pub fn require_env(name: &'static str) -> Result<String, DevtoolError> {
    std::env::var(name).map_err(|_| DevtoolError::MissingEnvVar { name })
}

pub async fn connect(database_url: &str) -> Result<DbPool, DevtoolError> {
    let config = DbConfig::new(database_url);
    Ok(underlay_db::create_pool(&config).await?)
}

pub async fn migrate(
    pool: &DbPool,
    migrator: &sqlx::migrate::Migrator,
) -> Result<(), DevtoolError> {
    underlay_db::run_migrations(pool, migrator).await?;
    Ok(())
}

pub async fn migrate_with(
    pool: &DbPool,
    migrate: impl for<'a> Fn(&'a DbPool) -> MigrateFuture<'a>,
) -> Result<(), DevtoolError> {
    migrate(pool).await?;
    Ok(())
}

pub async fn reset_schemas<S, I>(
    pool: &DbPool,
    schemas: I,
    recreate_public: bool,
    allow_destructive: bool,
) -> Result<(), DevtoolError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let guard = if allow_destructive {
        DestructiveGuard::allow()
    } else {
        DestructiveGuard::disallow()
    };

    underlay_db::drop_schemas(pool, guard, schemas).await?;

    if recreate_public {
        sqlx::query("CREATE SCHEMA public;").execute(pool).await?;
    }

    Ok(())
}

pub async fn migrate_from_env(
    database_url_env: &'static str,
    migrator: &sqlx::migrate::Migrator,
) -> Result<DbPool, DevtoolError> {
    let database_url = require_env(database_url_env)?;
    let pool = connect(&database_url).await?;
    migrate(&pool, migrator).await?;
    Ok(pool)
}

pub async fn migrate_from_env_with(
    database_url_env: &'static str,
    migrate: impl for<'a> Fn(&'a DbPool) -> MigrateFuture<'a>,
) -> Result<DbPool, DevtoolError> {
    let database_url = require_env(database_url_env)?;
    let pool = connect(&database_url).await?;
    migrate_with(&pool, migrate).await?;
    Ok(pool)
}

pub async fn reset_from_env<S, I>(
    database_url_env: &'static str,
    schemas: I,
    recreate_public: bool,
    allow_destructive: bool,
) -> Result<(), DevtoolError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let database_url = require_env(database_url_env)?;
    let pool = connect(&database_url).await?;
    reset_schemas(&pool, schemas, recreate_public, allow_destructive).await?;
    Ok(())
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
