use std::path::Path;

use sqlx::migrate::Migrator;

use crate::DbPool;

pub async fn run_migrations(
    pool: &DbPool,
    migrator: &Migrator,
) -> Result<(), sqlx::migrate::MigrateError> {
    migrator.run(pool).await
}

/// Load a sqlx migrator from a directory at runtime.
///
/// This is useful for dev tooling, but production binaries should typically embed
/// migrations with `sqlx::migrate!`.
pub async fn load_migrator_from_dir(
    dir: impl AsRef<Path>,
) -> Result<Migrator, sqlx::migrate::MigrateError> {
    Migrator::new(dir.as_ref()).await
}

#[cfg(test)]
#[path = "tests/migrations_tests.rs"]
mod tests;
