# 050 - Database and Migrations

This document covers setting up the database layer using SQLx, including connection pool management and migration handling.

## Database Crate Structure

Keep migrations in a standard `migrations/` folder at the crate root (this matches `sqlx migrate` conventions):

```
apps/nursery/crates/db/
├── Cargo.toml
├── migrations/
│   └── 20250101000000_init.sql
└── src/
    └── lib.rs
```

## Step 1: Create Database Crate

Create `apps/nursery/crates/db/Cargo.toml`:

```toml
[package]
name = "myapp-db"
version.workspace = true
edition.workspace = true

[dependencies]
sqlx = { workspace = true }
tokio = { workspace = true }
```

Create `apps/nursery/crates/db/src/lib.rs`:

```rust
//! Database utilities and connection management.

pub use sqlx::PgPool;

/// Create a PostgreSQL connection pool.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Run embedded database migrations.
///
/// This uses SQLx's compile-time migration embedding.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

/// Runtime migration runner (optional).
///
/// Use this if you want the migration path to be configurable.
/// Note: this is not the `sqlx::migrate!` macro.
pub async fn run_migrations_from_path(
    pool: &PgPool,
    migrations_path: &std::path::Path,
) -> Result<(), sqlx::migrate::MigrateError> {
    let migrator = sqlx::migrate::Migrator::new(migrations_path).await?;
    migrator.run(pool).await
}
```

## Step 2: Create Initial Migration

From `apps/nursery/crates/db`:

```bash
mkdir -p migrations
sqlx migrate add init
```

Edit the generated migration file (SQLx will name it with a timestamp). Example `migrations/20250101000000_init.sql`:

```sql
-- Initial database schema (example)

-- Required for gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE user_status AS ENUM (
    'active',
    'suspended',
    'deleted'
);

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;

CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    access_token_fingerprint VARCHAR(64) NOT NULL,
    refresh_token_fingerprint VARCHAR(64) NOT NULL,
    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'active'
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_access_fingerprint ON sessions(access_token_fingerprint);
CREATE INDEX idx_sessions_refresh_fingerprint ON sessions(refresh_token_fingerprint);

CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    user_id UUID REFERENCES users(id),
    session_id UUID REFERENCES sessions(id),
    ip_address INET,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    details JSONB,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_occurred_at ON audit_log(occurred_at);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

## Step 3: Using DB in the API

In `apps/nursery/crates/api/src/main.rs`:

```rust
use myapp_db::{create_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;
    run_migrations(&pool).await?;

    // build state + router...
    Ok(())
}
```

## Step 4: Environment

In `apps/nursery/.env`:

```bash
DATABASE_URL=postgres://myapp_user:password@localhost:5432/myapp_db
```

## Migration Commands

From `apps/nursery/crates/db`:

```bash
# Create DB
sqlx database create

# Run migrations
sqlx migrate run

# Check status
sqlx migrate info

# Revert last migration
sqlx migrate revert
```

## Testing with a Test Database (Optional)

If you want per-test databases, avoid string-splitting URLs incorrectly. The simplest safe approach is:

- Read `DATABASE_URL`
- Replace only the database name component

Example helper (simplified):

```rust
use sqlx::{Executor, PgPool};

pub async fn create_test_db() -> Result<(PgPool, String), sqlx::Error> {
    let default_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (base, _db_name) = default_url
        .rsplit_once('/')
        .expect("DATABASE_URL must include a database name");

    let test_db_name = format!(
        "test_{}",
        uuid::Uuid::new_v4().to_string().replace('-', "_")
    );

    let admin_url = format!("{}/postgres", base);
    let admin_pool = PgPool::connect(&admin_url).await?;

    admin_pool
        .execute(format!(r#"CREATE DATABASE \"{}\""#, test_db_name).as_str())
        .await?;

    let test_url = format!("{}/{}", base, test_db_name);
    let test_pool = PgPool::connect(&test_url).await?;

    // Run embedded migrations
    sqlx::migrate!("./migrations").run(&test_pool).await?;

    Ok((test_pool, test_db_name))
}
```

## Next Step

Proceed to [060-authentication](./060-authentication.md).
