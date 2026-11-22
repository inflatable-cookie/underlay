//! Test database utilities using testcontainers
//!
//! Provides an isolated PostgreSQL schema for each test. Schema teardown is
//! not automatic: call [`TestDb::cleanup`] for external databases. Dropping a
//! container-backed instance only destroys that container.
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
//!
//!     // Required when UNDERLAY_TEST_DATABASE_URL is set.
//!     db.cleanup().await.unwrap();
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

/// An isolated test database backed by a Docker container or an external URL.
///
/// Each `TestDb` instance gets its own schema for isolation. `Drop` does not
/// drop that schema. Call [`TestDb::cleanup`] when using an external database
/// (`UNDERLAY_TEST_DATABASE_URL`); otherwise the `test_*` schema remains.
/// Container-backed instances are destroyed with the container.
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
                let (url, container) = tokio::task::spawn_blocking(|| {
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
                    (url, container)
                })
                .await
                .expect("postgres test container setup task panicked");
                (url, Some(container))
            }
        };

        let config = DbConfig::new(database_url.clone())
            .with_max_connections(5)
            .with_min_connections(1)
            .with_acquire_timeout_secs(30)
            .with_idle_timeout_secs(600);

        let setup_pool = create_pool(&config)
            .await
            .expect("Failed to create test database pool");

        // Create a unique typed schema for this test.
        let schema = unique_test_schema();

        sqlx::query(sqlx::AssertSqlSafe(format!(
            "CREATE SCHEMA {}",
            schema.quoted()
        )))
        .execute(&setup_pool)
        .await
        .expect("Failed to create test schema");

        setup_pool.close().await;

        // Bind search_path on every pool connection via after_connect so no
        // connection in the pool can silently fall back to `public`.
        let set_search_path = format!("SET search_path TO {}, public", schema.quoted());
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .idle_timeout(std::time::Duration::from_secs(600))
            .after_connect(move |conn, _meta| {
                let sql = set_search_path.clone();
                Box::pin(async move {
                    sqlx::Executor::execute(conn, sqlx::AssertSqlSafe(sql)).await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("Failed to create schema-isolated test database pool");

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
    /// in the test schema. Supports multi-statement SQL scripts.
    pub async fn load_fixture(&self, sql: &str) -> Result<(), sqlx::Error> {
        sqlx::raw_sql(sqlx::AssertSqlSafe(sql))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Execute a SQL file from a path
    pub async fn load_fixture_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sql = tokio::fs::read_to_string(path).await?;
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
    /// db.cleanup().await.unwrap(); // required for UNDERLAY_TEST_DATABASE_URL
    /// ```
    pub async fn run_migrations(
        &self,
        migrations_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::path::Path;

        let path = Path::new(migrations_path);
        if !tokio::fs::try_exists(path).await? {
            return Err(format!("Migrations path does not exist: {}", migrations_path).into());
        }

        let mut dir = tokio::fs::read_dir(path).await?;
        let mut entries = Vec::new();
        while let Some(entry) = dir.next_entry().await? {
            if entry.path().extension().is_some_and(|ext| ext == "sql") {
                entries.push(entry);
            }
        }

        // Sort by filename to ensure correct order
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let sql = tokio::fs::read_to_string(entry.path()).await?;
            sqlx::raw_sql(sqlx::AssertSqlSafe(sql.as_str()))
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
    /// db.cleanup().await.unwrap(); // required for UNDERLAY_TEST_DATABASE_URL
    /// ```
    pub async fn run_migrator(
        &self,
        migrator: &sqlx::migrate::Migrator,
    ) -> Result<(), sqlx::migrate::MigrateError> {
        migrator.run(&self.pool).await
    }

    /// Drop the test schema.
    ///
    /// Required for external databases (`UNDERLAY_TEST_DATABASE_URL`).
    /// `Drop` does not run this. Container-backed tests may skip it because
    /// destroying the container removes the database. Call it mid-test when
    /// you need to reset the schema before the instance is dropped.
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
        Ok(output) if output.status.success() => (),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!(
                "docker daemon is not reachable (`docker version` exited with {}): {}\n\
                 Start Docker Desktop or Colima (`colima start`), or set {TEST_DATABASE_URL_ENV}.",
                output.status,
                stderr.trim()
            );
        }
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
