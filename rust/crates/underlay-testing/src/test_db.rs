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
use testcontainers::runners::SyncRunner;
use testcontainers::Container;
use testcontainers_modules::postgres::Postgres;
use underlay_db::{
    create_pool, drop_schema_identifiers, DbConfig, DestructiveGuard, SqlIdentifier,
};
use uuid::Uuid;

/// An isolated test database backed by a Docker container.
///
/// The database is automatically cleaned up when this struct is dropped.
/// Each `TestDb` instance gets its own schema for isolation.
/// Environment variable pointing at an already-provisioned Postgres. When set,
/// `TestDb` connects to it directly instead of starting a testcontainer, so the
/// suite runs against a CI Postgres service, an `effigy container` Postgres, or
/// any local instance — no Docker API required. Per-test schema isolation still
/// keeps concurrent tests from colliding on the shared database.
///
/// **Use a throwaway database.** In external mode nothing tears the database
/// down afterwards: each test leaves its `test_*` schema behind unless
/// [`TestDb::cleanup`] is called (and some suites — e.g. `underlay-jobs-postgres`
/// — rebuild fixed schemas like `platform` destructively). A CI service
/// container or local scratch container is the intended target, not a
/// long-lived shared database.
pub const TEST_DATABASE_URL_ENV: &str = "UNDERLAY_TEST_DATABASE_URL";

pub struct TestDb {
    pool: PgPool,
    schema: SqlIdentifier,
    // Keep the testcontainer alive for the lifetime of the test. `None` when
    // connected to an external database via `UNDERLAY_TEST_DATABASE_URL`.
    _container: Option<Arc<Container<Postgres>>>,
}

impl TestDb {
    /// Create a new isolated test database.
    ///
    /// This starts a PostgreSQL container (if not already running) and creates
    /// a unique schema for this test.
    ///
    /// # Panics
    ///
    /// Panics if neither `UNDERLAY_TEST_DATABASE_URL` is set nor Docker is
    /// available, or if the database cannot be created.
    pub async fn new() -> Self {
        // Prefer an already-provisioned Postgres when the env var is set; only
        // fall back to a testcontainer (which needs a Docker API) otherwise.
        let (database_url, container) = match std::env::var(TEST_DATABASE_URL_ENV) {
            Ok(url) if !url.trim().is_empty() => (url, None),
            _ => {
                assert_docker_available();
                let container = Arc::new(
                    Postgres::default()
                        .start()
                        .expect("failed to start postgres test container"),
                );
                let url = format!(
                    "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                    container
                        .get_host_port_ipv4(5432)
                        .expect("postgres port 5432 should be mapped")
                );
                (url, Some(container))
            }
        };

        let config = DbConfig::new(database_url)
            .with_max_connections(5)
            .with_min_connections(1)
            .with_acquire_timeout_secs(30)
            .with_idle_timeout_secs(600);

        let pool = create_pool(&config)
            .await
            .expect("Failed to create test database pool");

        // Create a unique typed schema for this test.
        let schema = unique_test_schema();

        sqlx::query(sqlx::AssertSqlSafe(format!("CREATE SCHEMA {}", schema.quoted())))
            .execute(&pool)
            .await
            .expect("Failed to create test schema");

        // Set search path to the test schema
        sqlx::query(sqlx::AssertSqlSafe(format!(
            "SET search_path TO {}, public",
            schema.quoted()
        )))
            .execute(&pool)
            .await
            .expect("Failed to set search path");

        Self {
            pool,
            schema,
            _container: container,
        }
    }

    /// Get the connection pool for this test database
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get the schema name for this test
    pub fn schema_name(&self) -> &str {
        self.schema.as_str()
    }

    /// Load a SQL fixture file
    ///
    /// The fixture content should be valid SQL that will be executed
    /// in the test schema.
    pub async fn load_fixture(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::query(sqlx::AssertSqlSafe(sql)).execute(&self.pool).await?;
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
            sqlx::query(sqlx::AssertSqlSafe(sql))
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
        drop_schema_identifiers(&self.pool, DestructiveGuard::allow(), [&self.schema]).await
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        // Async schema cleanup is explicit; dropping the container cleans the database.
    }
}

fn unique_test_schema() -> SqlIdentifier {
    let schema_name = format!("test_{}", Uuid::now_v7().to_string().replace('-', ""));
    SqlIdentifier::parse(schema_name).expect("generated test schema name should be valid")
}

/// Check for Docker availability.
fn assert_docker_available() {
    match std::process::Command::new("docker").arg("version").output() {
        Ok(_) => (),
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
}

#[cfg(test)]
#[path = "tests/test_db_tests.rs"]
mod tests;
