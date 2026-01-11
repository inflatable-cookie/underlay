# 050 - Database & Migrations

> **Reference Implementation**: This guide includes patterns from Acowtancy, a production application built with Underlay. These serve as working examples of best practices.

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

-- Note: PostgreSQL 13+ has gen_random_uuid() built-in.
-- For PostgreSQL \u003c13, uncomment the following line:
-- CREATE EXTENSION IF NOT EXISTS pgcrypto;

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

## Development Seeds

For local development, you may want test data separate from production migrations.

### Pattern: Separate Dev Migrations

Create a separate directory for development-only seeds:

```
apps/nursery/crates/db/
├── migrations/           # Production migrations
│   ├── 001_create_users.sql
│   └── 002_create_articles.sql
└── migrations_dev/       # Development seeds (never run in prod)
    ├── 001_seed_users.sql
    └── 002_seed_articles.sql
```

### Seed Files

`migrations_dev/001_seed_users.sql`:

```sql
-- Development seed data for users
INSERT INTO auth.users (id, email, display_name, role, status) VALUES
    ('01933f9a-7b1e-7c9f-8f3d-1a2b3c4d5e6f', 'admin@example.com', 'Admin User', 'admin', 'active'),
    ('01933f9a-7b1e-7c9f-8f3d-2b3c4d5e6f7g', 'user@example.com', 'Test User', 'user', 'active'),
    ('01933f9a-7b1e-7c9f-8f3d-3c4d5e6f7g8h', 'editor@example.com', 'Editor User', 'editor', 'active')
ON CONFLICT (id) DO NOTHING;
```

### Run Dev Seeds

Create a helper function in `apps/nursery/crates/db/src/lib.rs`:

```rust
use sqlx::PgPool;

/// Run development seeds (local only).
///
/// These are simple SQL statements that insert test data.
/// Never run in production!
pub async fn run_dev_seeds(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Check environment
    let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());
    if env == "production" {
        eprintln!("WARNING: Refusing to run dev seeds in production");
        return Ok(());
    }

    tracing::info!("Running development seeds...");

    // Run each seed file
    let seed_files = [
        include_str!("../migrations_dev/001_seed_users.sql"),
        include_str!("../migrations_dev/002_seed_articles.sql"),
    ];

    for (idx, sql) in seed_files.iter().enumerate() {
        tracing::debug!("Running seed file {}", idx + 1);
        sqlx::raw_sql(sql).execute(pool).await?;
    }

    tracing::info!("Development seeds completed");
    Ok(())
}
```

### Usage in Main

```rust
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    
    let pool = create_pool(&database_url).await?;
    run_migrations(&pool).await?;
    
    // Run dev seeds in development
    if std::env::var("ENVIRONMENT").unwrap_or_default() == "development" {
        run_dev_seeds(&pool).await?;
    }
    
    // Start server...
    Ok(())
}
```

### Reset Script

Create a reset script for development:

`apps/nursery/crates/db/src/bin/reset_dev_db.rs`:

```rust
use anyhow::Result;
use myapp_db::{create_pool, drop_database, run_migrations, run_dev_seeds};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    
    let env = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());
    if env != "development" {
        anyhow::bail!("Can only reset database in development environment");
    }
    
    let database_url = std::env::var("DATABASE_URL")?;
    
    println!("Dropping and recreating database...");
    drop_database(&database_url).await?;
    
    let pool = create_pool(&database_url).await?;
    
    println!("Running migrations...");
    run_migrations(&pool).await?;
    
    println!("Running development seeds...");
    run_dev_seeds(&pool).await?;
    
    println!("✓ Database reset complete");
    Ok(())
}
```

Add to `Cargo.toml`:

```toml
[[bin]]
name = "reset_dev_db"
path = "src/bin/reset_dev_db.rs"
```

**Run it:**

```bash
cargo run --bin reset_dev_db
```

### Benefits

- **Separation** - Dev data separate from production migrations
- **Safety** - Environment check prevents running in prod
- **Speed** - Quick database reset during development
- **Consistency** - All developers start with same test data

### Acowtancy Pattern

Acowtancy uses this exact pattern:
- `farmyard/crates/db/migrations/` - Production migrations
- `farmyard/crates/db/migrations_dev/` - Development seeds
- `farmyard/crates/db/src/lib.rs` - `run_dev_seeds()` function
- `farmyard/crates/db/src/bin/reset_dev_db.rs` - Reset script

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

    // Note: CREATE DATABASE cannot use bind parameters in PostgreSQL.
    // The database name contains only alphanumeric + underscores (UUID-based) to prevent injection.
    // In production code, consider additional validation.\n    sqlx::query(\u0026format!(r#\"CREATE DATABASE \\\"{}\\\"\"#, test_db_name))\n        .execute(\u0026admin_pool)\n        .await?;

    let test_url = format!("{}/{}", base, test_db_name);
    let test_pool = PgPool::connect(&test_url).await?;

    // Run embedded migrations
    sqlx::migrate!("./migrations").run(&test_pool).await?;

    Ok((test_pool, test_db_name))
}
```

## CRITICAL: Schema Qualification in Migrations

**Rule**: **NEVER** use `SET search_path` in SQL migrations. Always fully qualify schema names.

### Why This Matters

Migration runners (including SQLx) do not reliably preserve `search_path` settings across different execution contexts. This can cause:
- Migrations failing in CI/CD but working locally
- Objects created in the wrong schema
- Silent failures that only surface in production

### ❌ WRONG - Using search_path

```sql
-- ❌ DON'T DO THIS
SET search_path TO auth, public;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL
);

-- Which schema is this in? Depends on search_path!
INSERT INTO users (id, email) VALUES (...);
```

### ✅ CORRECT - Fully Qualified Names

```sql
-- ✅ ALWAYS DO THIS
CREATE TABLE auth.users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL
);

CREATE INDEX idx_users_email ON auth.users(email);

INSERT INTO auth.users (id, email) VALUES (...);

-- Foreign keys
CREATE TABLE auth.sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE
);

-- Cross-schema references
CREATE TABLE content.articles (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES auth.users(id)
);
```

### Schema Organization Example

```sql
-- Migration: 001_create_schemas.sql

CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS content;
CREATE SCHEMA IF NOT EXISTS learning;
CREATE SCHEMA IF NOT EXISTS platform;

-- Migration: 002_create_auth_tables.sql

CREATE TYPE auth.user_status AS ENUM ('active', 'suspended', 'deleted');

CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    status auth.user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE auth.credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    secret_encrypted TEXT NOT NULL,
    metadata JSONB,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

CREATE INDEX idx_credentials_user_id ON auth.credentials(user_id);
CREATE INDEX idx_credentials_type ON auth.credentials(type);

-- Migration: 003_create_content_tables.sql

CREATE TABLE content.articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES auth.users(id),
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_articles_author ON content.articles(author_id);
CREATE INDEX idx_articles_published ON content.articles(published_at) WHERE published_at IS NOT NULL;
```

### In Rust Code

Always use qualified names in raw SQL:

```rust
// ✅ CORRECT
let user = sqlx::query_as::<_, User>(
    r#"
    SELECT id, email, display_name, status, created_at, updated_at
    FROM auth.users
    WHERE email = $1
    "#
)
.bind(email)
.fetch_optional(&pool)
.await?;

// ✅ CORRECT - Cross-schema query
let articles = sqlx::query_as::<_, Article>(
    r#"
    SELECT a.id, a.title, a.content, a.published_at,
           u.display_name as author_name
    FROM content.articles a
    JOIN auth.users u ON u.id = a.author_id
    WHERE a.published_at IS NOT NULL
    ORDER BY a.published_at DESC
    LIMIT $1
    "#
)
.bind(limit)
.fetch_all(&pool)
.await?;
```

### Exception: Dynamic SQL (Rare)

If you absolutely must use dynamic schema names (rare), validate them:

```rust
use underlay_db::validate_schema_name;

// Only allows alphanumeric and underscores
validate_schema_name(schema)?;

let query = format!(
    "SELECT * FROM {}.users WHERE id = $1",
    schema  // Already validated
);
```

**But prefer**: Hard-code schema names in your application. Use configuration for database names, not schema names.

### Checklist for Every Migration

Before committing a migration:

- [ ] No `SET search_path` statements
- [ ] All table references include schema prefix (e.g., `auth.users`)
- [ ] All type references include schema prefix (e.g., `auth.user_status`)
- [ ] All index definitions use qualified table names
- [ ] All foreign keys use qualified table names
- [ ] All cross-schema references are explicit

### Benefits of Schema Qualification

1. **Reliability**: Works the same in all environments
2. **Clarity**: Easy to see which schema owns each object
3. **Safety**: No ambiguity about object location
4. **Maintenance**: Easier to reorganize schemas later
5. **CI/CD**: No environment-specific behavior

## See Also

**Related Guides:**
- **[060-authentication.md](./060-authentication.md)** - Auth schema, JWT setup, credentials table
- **[040-rust-backend.md](./040-rust-backend.md)** - Database pool setup, SQL queries in handlers
- **[070-api-handlers.md](./070-api-handlers.md)** - Using database in API handlers
- **[130-testing.md](./130-testing.md)** - Database testing patterns, test fixtures, integration tests

**Key Topics:**
- Migration best practices: Never use `SET search_path`
- Schema organization: Group related tables (auth, content, etc.)
- Dev seeds pattern: Separate test data from production migrations
- Connection pooling: Configure for production workloads
- Test database setup: Clean database for integration tests

**Migration Checklist:**
1. Create schema (`CREATE SCHEMA IF NOT EXISTS ...`)
2. Fully qualify all object names
3. Add indexes for foreign keys and query patterns
4. Use `gen_random_uuid()` for UUIDs (Postgres 13+)
5. Add constraints (NOT NULL, CHECK, UNIQUE)
6. Test migration in clean database
7. Test rollback if applicable

## Next Steps

Proceed to [060-authentication](./060-authentication.md).

