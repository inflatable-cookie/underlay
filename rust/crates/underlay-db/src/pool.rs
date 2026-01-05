use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub type DbPool = PgPool;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl DbConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            max_connections: 10,
        }
    }
}

pub async fn create_pool(config: &DbConfig) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
}
