use underlay_db::{create_pool, drop_schemas, validate_schema_name, DbConfig, DestructiveGuard};
use underlay_core::Uuid;

// These tests require a Docker-compatible runtime.
// On macOS, Colima works well:
//   brew install colima docker
//   colima start
//
// Run with:
//   cargo test -p underlay-db --test postgres_integration -- --ignored

use testcontainers::clients::Cli;
use testcontainers::Container;
use testcontainers_modules::postgres::Postgres;

fn docker_client() -> Cli {
    // `testcontainers` uses the `docker` CLI. We check it exists up front so
    // failures are actionable.
    match std::process::Command::new("docker")
        .arg("version")
        .output()
    {
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

fn postgres_database_url(node: &Container<'_, Postgres>) -> String {
    // Using testcontainers so we don't require any preinstalled Postgres.
    // The Docker runtime can be Docker Desktop, Colima, OrbStack, etc.
    //
    // IMPORTANT: We intentionally keep these tests `#[ignore]` by default.

    // e.g. postgres://postgres:postgres@127.0.0.1:<port>/postgres
    format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        node.get_host_port_ipv4(5432)
    )
}

#[tokio::test]
#[ignore]
async fn create_pool_connects() {
    let docker = docker_client();
    let node = docker.run(Postgres::default());

    let database_url = postgres_database_url(&node);
    let config = DbConfig {
        database_url,
        max_connections: 5,
    };

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
    let docker = docker_client();
    let node = docker.run(Postgres::default());

    let database_url = postgres_database_url(&node);
    let config = DbConfig {
        database_url,
        max_connections: 5,
    };

    let pool = create_pool(&config).await.expect("create_pool");

    let schema_id = Uuid::new_v7().to_string().replace('-', "");
    let schema = format!("test_{schema_id}");
    assert!(validate_schema_name(&schema));

    sqlx::query(&format!("CREATE SCHEMA {schema}"))
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
    let docker = docker_client();
    let node = docker.run(Postgres::default());

    let database_url = postgres_database_url(&node);
    let config = DbConfig {
        database_url,
        max_connections: 5,
    };

    let pool = create_pool(&config).await.expect("create_pool");

    let schema_id = Uuid::new_v7().to_string().replace('-', "");
    let schema = format!("test_{schema_id}");
    assert!(validate_schema_name(&schema));

    // Create schema + a table to ensure CASCADE behavior.
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .expect("create schema");

    sqlx::query(&format!("CREATE TABLE {schema}.items(id INT PRIMARY KEY)"))
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
    let docker = docker_client();
    let node = docker.run(Postgres::default());

    let database_url = postgres_database_url(&node);
    let config = DbConfig {
        database_url,
        max_connections: 5,
    };

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
