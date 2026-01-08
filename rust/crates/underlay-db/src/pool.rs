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

#[cfg(test)]
mod tests {
    use super::DbConfig;

    #[test]
    fn db_config_default_max_connections() {
        let config = DbConfig::new("postgres://localhost/test");
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn db_config_with_custom_max_connections() {
        let config = DbConfig {
            database_url: "postgres://localhost/test".to_string(),
            max_connections: 50,
        };
        assert_eq!(config.max_connections, 50);
    }

    #[test]
    fn db_config_stores_database_url() {
        let url = "postgres://localhost:5432/mydb";
        let config = DbConfig::new(url);
        assert_eq!(config.database_url, url);
    }

    #[test]
    fn db_config_from_string() {
        let config = DbConfig::new("postgresql://user:pass@localhost/db");
        assert_eq!(config.database_url, "postgresql://user:pass@localhost/db");
    }

    #[test]
    fn db_config_clone_works() {
        let config = DbConfig::new("postgres://localhost/test");
        let cloned = config.clone();
        assert_eq!(cloned.database_url, config.database_url);
        assert_eq!(cloned.max_connections, config.max_connections);
    }

    #[test]
    fn db_config_debug_contains_relevant_info() {
        let config = DbConfig::new("postgres://user:secretpassword@localhost/test");
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("postgres://user:"));
        assert!(debug_str.contains("test"));
    }
}
