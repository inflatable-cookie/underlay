# 050 - Database and Migrations

This document covers setting up the database layer using SQLx, including connection pool management and migration handling.

## Database Crate Structure

```
apps/nursery/crates/db/
├── Cargo.toml
└── src/
    ├── lib.rs              # Public exports, pool creation
    └── migrations/         # SQL migration files
        ├── 000_init.sql
        ├── 001_add_users.sql
        └── ...
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
underlay-db = { path = "../../../libs/underlay/rust/crates/underlay-db" }
underlay-core = { path = "../../../libs/underlay/rust/crates/underlay-core" }
tokio = { workspace = true }
```

Create `apps/nursery/crates/db/src/lib.rs`:

```rust
//! Database utilities and connection management.

pub use sqlx::PgPool;

use underlay_db::Migrations;

/// Create a PostgreSQL connection pool.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Run database migrations.
///
/// Migrations are stored in the `migrations/` directory.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    // Run migrations from the compiled binary's embedded migrations
    sqlx::migrate!("migrations").run(pool).await
}

/// Run migrations from a specific directory.
///
/// Use this when migrations are not embedded in the binary.
pub async fn run_migrations_from(
    pool: &PgPool,
    migrations_path: &str,
) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!(migrations_path).run(pool).await
}

/// Transaction helper for atomic operations.
#[macro_export]
macro_rules! transaction {
    ($pool:expr, |$tx:ident| $body:expr) => {{
        let mut tx = $pool.begin().await?;
        match $body {
            Ok(result) => {
                tx.commit().await?;
                Ok(result)
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }};
}
```

## Step 2: Create Initial Migration

Create the migrations directory and first migration:

```bash
mkdir -p apps/nursery/crates/db/src/migrations
cd apps/nursery/crates/db
sqlx migrate add init
```

Edit `apps/nursery/crates/db/src/migrations/000_init.sql`:

```sql
-- Initial database schema
-- This follows Underlay auth database patterns

-- === Enums ===

CREATE TYPE user_status AS ENUM (
    'active',
    'suspended',
    'deleted'
);

-- === Tables ===

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    access_token_fingerprint VARCHAR(64) NOT NULL,
    refresh_token_fingerprint VARCHAR(64) NOT NULL,
    access_token_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    refresh_token_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
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
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_occurred_at ON audit_log(occurred_at);

-- === Soft Delete Support ===

ALTER TABLE users ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP WITH TIME ZONE;

CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;

-- === Updated At Trigger ===

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

## Step 3: Create Additional Migrations

Create `apps/nursery/crates/db/src/migrations/001_add_artists.sql`:

```sql
-- Add artist-specific tables

CREATE TABLE artists (
    id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    bio TEXT,
    website_url VARCHAR(500),
    social_links JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

## Step 4: Database Pool in Application

Update `apps/nursery/crates/api/src/main.rs` to include database setup:

```rust
use myapp_db::{create_pool, run_migrations};
use myapp_infra::tracing::init_tracing;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    init_tracing();

    // Load configuration
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    // Create database pool
    let pool = create_pool(&database_url).await?;

    // Run migrations
    run_migrations(&pool).await?;
    tracing::info!("Database migrations completed");

    // Create application state
    let state = AppState { pool };

    // Build and start the router
    // ... rest of main.rs
    Ok(())
}
```

## Step 5: Repository Pattern

Create `apps/nursery/crates/core/src/repositories.rs`:

```rust
//! Repository trait definitions for data access.
//!
//! Following Underlay patterns, define repository traits that
//! implementations will fulfill.

use async_trait::async_trait;
use underlay_core::Uuid;

use crate::{Artist, ArtistId};

/// Result type for repository operations.
pub type RepoResult<T> = Result<T, RepoError>;

/// Repository errors.
#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Repository trait for artist operations.
#[async_trait]
pub trait ArtistRepository: Send + Sync {
    /// Find an artist by ID.
    async fn find_by_id(&self, id: ArtistId) -> RepoResult<Option<Artist>>;

    /// Find an artist by user ID.
    async fn find_by_user_id(&self, user_id: Uuid) -> RepoResult<Option<Artist>>;

    /// Create a new artist.
    async fn create(&self, artist: &Artist) -> RepoResult<Artist>;

    /// Update an artist.
    async fn update(&self, artist: &Artist) -> RepoResult<Artist>;

    /// Delete an artist (soft delete).
    async fn delete(&self, id: ArtistId) -> RepoResult<()>;
}

/// In-memory implementation for testing.
#[derive(Debug, Default)]
pub struct InMemoryArtistRepository {
    artists: std::sync::Mutex<std::collections::HashMap<ArtistId, Artist>>,
}

#[async_trait]
impl ArtistRepository for InMemoryArtistRepository {
    async fn find_by_id(&self, id: ArtistId) -> RepoResult<Option<Artist>> {
        Ok(self.artists.lock().unwrap().get(&id).cloned())
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> RepoResult<Option<Artist>> {
        // Implementation for in-memory lookup
        Ok(None)
    }

    async fn create(&self, artist: &Artist) -> RepoResult<Artist> {
        let mut artists = self.artists.lock().unwrap();
        artists.insert(artist.id, artist.clone());
        Ok(artist.clone())
    }

    async fn update(&self, artist: &Artist) -> RepoResult<Artist> {
        let mut artists = self.artists.lock().unwrap();
        artists.insert(artist.id, artist.clone());
        Ok(artist.clone())
    }

    async fn delete(&self, _id: ArtistId) -> RepoResult<()> {
        Ok(())
    }
}
```

## Step 6: Database Configuration

Create `apps/nursery/.env`:

```bash
# Database
DATABASE_URL=postgres://myapp_user:password@localhost:5432/myapp_db

# For migrations via sqlx-cli
DATABASE_URL=postgres://myapp_user:password@localhost:5432/myapp_db
```

## Migration Commands

```bash
# Create a new migration
cd apps/nursery/crates/db
sqlx migrate add migration_name

# Run migrations locally
sqlx database create
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

## Testing with Test Database

Create `apps/nursery/crates/db/src/test.rs`:

```rust
//! Test utilities for database operations.

use sqlx::{Executor, PgPool};

/// Create a test database with a unique name.
pub async fn create_test_db() -> Result<PgPool, sqlx::Error> {
    let test_db_name = format!("test_{}", uuid::Uuid::new_v4().to_string().replace('-', "_"));

    // Connect to default database to create test DB
    let default_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let base_url = default_url.rsplit('/').next().unwrap();
    let creator_url = format!("{}/postgres", base_url);

    let pool = sqlx::PgPool::connect(&creator_url).await?;

    // Create test database
    pool.execute(&format!(r#"CREATE DATABASE "{}""#, test_db_name))
        .await?;

    // Connect to the new test database
    let test_url = format!("{}/{}", base_url, test_db_name);
    let test_pool = sqlx::PgPool::connect(&test_url).await?;

    // Run migrations
    sqlx::migrate!("migrations").run(&test_pool).await?;

    Ok(test_pool)
}

/// Clean up test database.
pub async fn drop_test_db(pool: &PgPool, db_name: &str) {
    let _ = sqlx::query(&format!(r#"DROP DATABASE "{}" WITH (FORCE)"#, db_name))
        .execute(pool)
        .await;
}
```

## Next Step

With the database layer configured, proceed to [060-authentication](./060-authentication.md) to implement the authentication system.
