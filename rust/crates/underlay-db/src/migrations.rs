use sqlx::migrate::Migrator;

use crate::DbPool;

pub async fn run_migrations(
    pool: &DbPool,
    migrator: &Migrator,
) -> Result<(), sqlx::migrate::MigrateError> {
    migrator.run(pool).await
}
