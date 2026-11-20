use underlay_core::Uuid;
use underlay_db::{
    create_pool, drop_schemas, run_sql_dir_with_options, validate_schema_name, DbConfig,
    DestructiveGuard, SqlDirOptions,
};

// These tests require a Docker-compatible runtime.
// On macOS, Colima works well:
//   brew install colima docker
//   colima start
//
// Run with:
//   oxide test -p underlay-db --test postgres_integration -- --ignored

use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

fn assert_docker_available() {
    // `testcontainers` uses the `docker` CLI. We check it exists up front so
    // failures are actionable.
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
}

/// Start one Postgres container for a test.
///
/// These tests run inside `#[tokio::test]`, so they use `testcontainers`'
/// async runner. The sync `Container` blocks and cleans up on its own runtime,
/// which panics when it is dropped inside a foreign tokio runtime. The returned
/// container must stay alive for the duration of the test.
async fn postgres_fixture() -> (ContainerAsync<Postgres>, String) {
    assert_docker_available();

    // Using testcontainers so we don't require any preinstalled Postgres.
    // The Docker runtime can be Docker Desktop, Colima, OrbStack, etc.
    //
    // IMPORTANT: We intentionally keep these tests `#[ignore]` by default.
    let node = Postgres::default()
        .start()
        .await
        .expect("failed to start postgres test container");

    // e.g. postgres://postgres:postgres@127.0.0.1:<port>/postgres
    let port = node
        .get_host_port_ipv4(5432)
        .await
        .expect("postgres port 5432 should be mapped");
    let database_url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

    (node, database_url)
}

#[tokio::test]
#[ignore]
async fn create_pool_connects() {
    let (_node, database_url) = postgres_fixture().await;
    let config = DbConfig::new(database_url).with_max_connections(5);

    let pool = create_pool(&config).await.expect("create_pool");
    let (one,): (i64,) = sqlx::query_as("SELECT 1::BIGINT")
        .fetch_one(&pool)
        .await
        .expect("SELECT 1");
    assert_eq!(one, 1);
}

#[tokio::test]
#[ignore]
async fn drop_schemas_requires_guard() {
    let (_node, database_url) = postgres_fixture().await;
    let config = DbConfig::new(database_url).with_max_connections(5);

    let pool = create_pool(&config).await.expect("create_pool");

    let schema_id = Uuid::new_v7().to_string().replace('-', "");
    let schema = format!("test_{schema_id}");
    assert!(validate_schema_name(&schema));

    sqlx::query(sqlx::AssertSqlSafe(format!("CREATE SCHEMA {schema}")))
        .execute(&pool)
        .await
        .expect("create schema");

    let err = drop_schemas(&pool, DestructiveGuard::disallow(), [&schema])
        .await
        .expect_err("expected destructive guard error");

    assert!(err.to_string().to_ascii_lowercase().contains("destructive"));
}

#[tokio::test]
#[ignore]
async fn drop_schemas_drops_schema() {
    let (_node, database_url) = postgres_fixture().await;
    let config = DbConfig::new(database_url).with_max_connections(5);

    let pool = create_pool(&config).await.expect("create_pool");

    let schema_id = Uuid::new_v7().to_string().replace('-', "");
    let schema = format!("test_{schema_id}");
    assert!(validate_schema_name(&schema));

    // Create schema + a table to ensure CASCADE behavior.
    sqlx::query(sqlx::AssertSqlSafe(format!("CREATE SCHEMA {schema}")))
        .execute(&pool)
        .await
        .expect("create schema");

    sqlx::query(sqlx::AssertSqlSafe(format!(
        "CREATE TABLE {schema}.items(id INT PRIMARY KEY)"
    )))
    .execute(&pool)
    .await
    .expect("create table");

    drop_schemas(&pool, DestructiveGuard::allow(), [&schema])
        .await
        .expect("drop schemas");

    let (exists,): (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
    )
    .bind(&schema)
    .fetch_one(&pool)
    .await
    .expect("check schema exists");

    assert!(!exists);
}

#[tokio::test]
#[ignore]
async fn drop_schemas_rejects_invalid_schema_name() {
    let (_node, database_url) = postgres_fixture().await;
    let config = DbConfig::new(database_url).with_max_connections(5);

    let pool = create_pool(&config).await.expect("create_pool");

    let invalid_schema = "public; drop schema public";
    assert!(!validate_schema_name(invalid_schema));

    let err = drop_schemas(&pool, DestructiveGuard::allow(), [invalid_schema])
        .await
        .expect_err("expected invalid schema error");

    assert!(err
        .to_string()
        .to_ascii_lowercase()
        .contains("invalid schema name"));
}

#[tokio::test]
#[ignore]
async fn sql_dir_runs_multi_statement_scripts_unsplit() {
    let (_node, database_url) = postgres_fixture().await;
    let config = DbConfig::new(database_url).with_max_connections(5);

    let pool = create_pool(&config).await.expect("create_pool");

    let schema_id = format!("test_{}", Uuid::new_v7().to_string().replace('-', ""));
    let script_dir = std::env::temp_dir().join(format!("underlay-sql-dir-{schema_id}"));
    std::fs::create_dir_all(&script_dir).expect("create script fixture dir");
    std::fs::write(
        script_dir.join("010_seed.sql"),
        format!(
            r#"
CREATE SCHEMA {schema_id};
CREATE TABLE {schema_id}.items (id INT PRIMARY KEY);
DO $$
BEGIN
    INSERT INTO {schema_id}.items (id) VALUES (1);
    INSERT INTO {schema_id}.items (id) VALUES (2);
END $$;
"#
        ),
    )
    .expect("write script fixture");

    run_sql_dir_with_options(
        &pool,
        &script_dir,
        SqlDirOptions::new().with_split_on_semicolon(false),
    )
    .await
    .expect("run unsplit script");

    let (count,): (i64,) = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        "SELECT COUNT(*)::BIGINT FROM {schema_id}.items"
    )))
    .fetch_one(&pool)
    .await
    .expect("count seeded rows");
    assert_eq!(count, 2);

    std::fs::remove_dir_all(&script_dir).expect("remove script fixture");
}
