//! Test database utilities using testcontainers
//!
//! Provides an isolated PostgreSQL database for each test, with automatic
//! cleanup when the test completes.
//!
//! # Requirements
//!
//! Requires a Docker-compatible runtime (Docker Desktop, Colima, OrbStack, etc.)
//!
//! # Example
//!
//! ```ignore
//! use underlay_testing::TestDb;
//!
//! #[tokio::test]
//! async fn test_with_database() {
//!     let db = TestDb::new().await;
//!     
//!     // Run migrations
//!     sqlx::migrate!("./migrations").run(db.pool()).await.unwrap();
//!     
//!     // Use the database
//!     sqlx::query("INSERT INTO users (name) VALUES ($1)")
//!         .bind("test")
//!         .execute(db.pool())
//!         .await
//!         .unwrap();
//! }
//! ```

use sqlx::PgPool;
use std::sync::Arc;
use testcontainers::clients::Cli;
use testcontainers::Container;
use testcontainers_modules::postgres::Postgres;
use underlay_db::{create_pool, DbConfig};
use uuid::Uuid;

/// An isolated test database backed by a Docker container.
///
/// The database is automatically cleaned up when this struct is dropped.
/// Each `TestDb` instance gets its own schema for isolation.
pub struct TestDb {
    pool: PgPool,
    schema_name: String,
    // Keep container alive for the lifetime of the test
    _container: Arc<Container<'static, Postgres>>,
    // Keep docker client alive
    _docker: Arc<Cli>,
}

impl TestDb {
    /// Create a new isolated test database.
    ///
    /// This starts a PostgreSQL container (if not already running) and creates
    /// a unique schema for this test.
    ///
    /// # Panics
    ///
    /// Panics if Docker is not available or the database cannot be created.
    pub async fn new() -> Self {
        let docker = Arc::new(docker_client());

        // Use a static container reference via Box::leak for the 'static lifetime
        // This is safe because we keep Arc<Container> alive for the test duration
        let docker_ref: &'static Cli = Box::leak(Box::new(Cli::default()));
        let container = Arc::new(docker_ref.run(Postgres::default()));

        let database_url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432)
        );

        let config = DbConfig {
            database_url,
            max_connections: 5,
        };

        let pool = create_pool(&config)
            .await
            .expect("Failed to create test database pool");

        // Create a unique schema for this test
        let schema_name = format!("test_{}", Uuid::now_v7().to_string().replace('-', ""));

        sqlx::query(&format!("CREATE SCHEMA {schema_name}"))
            .execute(&pool)
            .await
            .expect("Failed to create test schema");

        // Set search path to the test schema
        sqlx::query(&format!("SET search_path TO {schema_name}, public"))
            .execute(&pool)
            .await
            .expect("Failed to set search path");

        Self {
            pool,
            schema_name,
            _container: container,
            _docker: docker,
        }
    }

    /// Get the connection pool for this test database
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the schema name for this test
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Load a SQL fixture file
    ///
    /// The fixture content should be valid SQL that will be executed
    /// in the test schema.
    pub async fn load_fixture(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sql).execute(&self.pool).await?;
        Ok(())
    }

    /// Execute a SQL file from a path
    pub async fn load_fixture_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sql = std::fs::read_to_string(path)?;
        self.load_fixture(&sql).await?;
        Ok(())
    }

    /// Seed data using a closure
    ///
    /// This allows programmatic seeding with type-safe operations.
    pub async fn seed<F, Fut>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(PgPool) -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
    {
        f(self.pool.clone()).await
    }

    /// Run migrations from a directory path.
    ///
    /// This runs all SQL migration files from the specified directory.
    /// Migration files should be named with a numeric prefix (e.g., `001_create_users.sql`).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let db = TestDb::new().await;
    /// db.run_migrations("./migrations").await.unwrap();
    /// ```
    pub async fn run_migrations(
        &self,
        migrations_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        use std::path::Path;

        let path = Path::new(migrations_path);
        if !path.exists() {
            return Err(format!("Migrations path does not exist: {}", migrations_path).into());
        }

        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "sql")
                    .unwrap_or(false)
            })
            .collect();

        // Sort by filename to ensure correct order
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let sql = fs::read_to_string(entry.path())?;
            sqlx::query(&sql)
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Migration {:?} failed: {}", entry.file_name(), e))?;
        }

        Ok(())
    }

    /// Run migrations using SQLx's embedded migrations.
    ///
    /// This is the preferred method when using `sqlx::migrate!()` macro.
    /// Call this with the migrator from your crate.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use sqlx::migrate::Migrator;
    ///
    /// static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    ///
    /// let db = TestDb::new().await;
    /// db.run_migrator(&MIGRATOR).await.unwrap();
    /// ```
    pub async fn run_migrator(
        &self,
        migrator: &sqlx::migrate::Migrator,
    ) -> Result<(), sqlx::migrate::MigrateError> {
        migrator.run(&self.pool).await
    }

    /// Clean up the test schema
    ///
    /// This is called automatically on drop, but can be called manually
    /// if you want to reset the database mid-test.
    pub async fn cleanup(&self) -> Result<(), sqlx::Error> {
        sqlx::query(&format!(
            "DROP SCHEMA IF EXISTS {} CASCADE",
            self.schema_name
        ))
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // Note: We can't run async cleanup in Drop, but the container
        // will be cleaned up when it's dropped. For schema cleanup,
        // tests should call cleanup() explicitly if needed, or the
        // container destruction will clean everything anyway.
    }
}

/// Check for Docker availability and return a client
fn docker_client() -> Cli {
    match std::process::Command::new("docker").arg("version").output() {
        Ok(_) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            panic!(
                "docker CLI not found. Install Colima + docker CLI and run `colima start`.\n\n\
macOS (Homebrew):\n\
  brew install colima docker\n\
  colima start\n\
  docker ps\n"
            );
        }
        Err(err) => {
            panic!("failed to run `docker version`: {err}");
        }
    }

    Cli::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require Docker and are ignored by default
    // Run with: cargo test -p underlay-testing --features db -- --ignored

    #[tokio::test]
    #[ignore]
    async fn test_db_creates_isolated_schema() {
        let db = TestDb::new().await;

        // Schema should exist
        let (exists,): (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
        )
        .bind(db.schema_name())
        .fetch_one(db.pool())
        .await
        .expect("check schema exists");

        assert!(exists, "Test schema should exist");
    }

    #[tokio::test]
    #[ignore]
    async fn test_db_load_fixture() {
        let db = TestDb::new().await;

        // Create a table via fixture
        db.load_fixture("CREATE TABLE items (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
            .await
            .expect("load fixture");

        // Insert some data
        db.load_fixture("INSERT INTO items (name) VALUES ('test1'), ('test2')")
            .await
            .expect("insert data");

        // Verify data
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*)::BIGINT FROM items")
            .fetch_one(db.pool())
            .await
            .expect("count items");

        assert_eq!(count.0, 2);
    }

    #[tokio::test]
    #[ignore]
    async fn test_db_isolation() {
        // Create two test databases
        let db1 = TestDb::new().await;
        let db2 = TestDb::new().await;

        // They should have different schemas
        assert_ne!(db1.schema_name(), db2.schema_name());

        // Create table in db1
        db1.load_fixture("CREATE TABLE isolated_table (id INT)")
            .await
            .expect("create table in db1");

        // Table should not exist in db2's schema
        let exists: (bool,) = sqlx::query_as(&format!(
            "SELECT EXISTS(SELECT 1 FROM information_schema.tables \
             WHERE table_schema = '{}' AND table_name = 'isolated_table')",
            db2.schema_name()
        ))
        .fetch_one(db2.pool())
        .await
        .expect("check table exists in db2");

        assert!(!exists.0, "Table should not exist in db2's schema");
    }
}
