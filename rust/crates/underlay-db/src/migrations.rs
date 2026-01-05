use sqlx::migrate::Migrator;

use crate::DbPool;

pub async fn run_migrations(
    pool: &DbPool,
    migrator: &'static Migrator,
) -> Result<(), sqlx::migrate::MigrateError> {
    migrator.run(pool).await
}
