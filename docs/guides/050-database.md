# 050 - Database & Migrations

> **Reference Implementation**: This guide includes patterns from a production application built with Underlay. These serve as working examples of best practices.

This document covers setting up the database layer using SQLx, including connection pool management, migration handling, and the docs-first schema development workflow.

## Docs-First Schema Development

**Schema documentation is the source of truth for design decisions.** Migrations are the mechanical implementation of that design.

### Why Docs-First?

The traditional approach—write a migration, then update docs—leads to:
- Documentation that's perpetually out of date
- Schema changes reviewed at the SQL level, not the design level
- Difficulty understanding the intended state vs. the current state
- No clear reference when something goes wrong

The docs-first approach inverts this:
1. **Design in documentation**: Describe the intended schema in human-readable docs
2. **Review the design**: Catch issues before writing SQL
3. **Implement mechanically**: The migration becomes a translation of reviewed docs
4. **Stay synchronized**: Docs are always current because they're written first

### The Docs-First Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. UPDATE DOCUMENTATION                                        │
│     Edit table docs with intended columns, constraints, indexes │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. REVIEW THE DESIGN                                           │
│     Verify constraints, relationships, and naming are correct   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. WRITE THE MIGRATION                                         │
│     Translate documentation into SQL                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. VERIFY ALIGNMENT                                            │
│     Confirm migration matches documentation exactly             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. TEST & DEPLOY                                               │
│     Run migration on dev, staging, then production              │
└─────────────────────────────────────────────────────────────────┘
```

### Documentation Structure

Organize schema documentation by schema and table:

```
docs/database/
├── 000-index.md              # Schema overview and conventions
├── 001-schema-issues.md      # Known issues and planned fixes
├── auth/
│   ├── users.md
│   ├── credentials.md
│   └── sessions.md
├── content/
│   ├── articles.md
│   └── media.md
└── learning/
    ├── courses.md
    └── lessons.md
```

### Table Documentation Template

Each table document should include:

```markdown
# schema.table_name

Brief description of what this table stores.

## Columns

| Column | Type | Nullable | Default | Description |
|--------|------|----------|---------|-------------|
| `id` | `uuid` | NO | - | Primary key (UUIDv7) |
| `name` | `text` | NO | - | Display name |
| `created_at` | `timestamptz` | NO | `now()` | Creation timestamp |

## Primary Key

\`\`\`sql
PRIMARY KEY (id)
\`\`\`

## Unique Constraints

\`\`\`sql
CONSTRAINT table_name_unique UNIQUE (name)
\`\`\`

## Check Constraints

| Column | Constraint |
|--------|------------|
| `name` | `char_length(name) <= 100` |

## Foreign Keys

| Column | References | On Delete |
|--------|------------|-----------|
| `user_id` | `auth.users(id)` | `CASCADE` |

## Indexes

\`\`\`sql
CREATE INDEX idx_table_user ON schema.table (user_id);
\`\`\`

## Soft Delete

✅ Supported via `deleted_at` and `delete_batch_id`.

## Notes

- Important behavioral notes
- Edge cases
- Migration considerations
```

### Example: Adding a New Column

**Step 1: Update documentation first**

Edit `docs/database/content/articles.md`:

```diff
 ## Columns
 
 | Column | Type | Nullable | Default | Description |
 |--------|------|----------|---------|-------------|
 | `id` | `uuid` | NO | - | Primary key |
 | `title` | `text` | NO | - | Article title |
+| `subtitle` | `text` | YES | - | Optional subtitle |
 | `created_at` | `timestamptz` | NO | `now()` | Creation timestamp |
```

**Step 2: Review the design**

- Is `subtitle` the right name?
- Should it be nullable or have a default?
- Are there length constraints needed?

**Step 3: Write migration**

```sql
-- migrations/202601211200__add_article_subtitle.sql
ALTER TABLE content.articles
ADD COLUMN subtitle TEXT NULL
CHECK (subtitle IS NULL OR char_length(subtitle) <= 200);
```

**Step 4: Verify alignment**

Confirm the migration matches what's documented.

### Checklist for Schema Changes

Before committing any schema change:

- [ ] Documentation updated in `docs/database/` FIRST
- [ ] Design reviewed (constraints, indexes, relationships)
- [ ] Migration written to match documentation
- [ ] Migration tested on local dev database
- [ ] Migration verified with `cargo sqlx prepare` (if applicable)
- [ ] Documentation and migration reviewed together
- [ ] Applied to staging, then production

## Database Crate Structure

Keep migrations in a standard `migrations/` folder at the crate root (this matches `sqlx migrate` conventions):

```
apps/api/crates/db/
├── Cargo.toml
├── migrations/
│   └── 20250101000000_init.sql
└── src/
    └── lib.rs
```

## Step 1: Create Database Crate

Create `apps/api/crates/db/Cargo.toml`:

```toml
[package]
name = "myapp-db"
version.workspace = true
edition.workspace = true

[dependencies]
sqlx = { workspace = true }
tokio = { workspace = true }
```

Create `apps/api/crates/db/src/lib.rs`:

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

From `apps/api/crates/db`:

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

In `apps/api/crates/api/src/main.rs`:

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

In `apps/api/.env`:

```bash
DATABASE_URL=postgres://myapp_user:password@localhost:5432/myapp_db
```

## Migration Commands

From `apps/api/crates/db`:

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
apps/api/crates/db/
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

Create a helper function in `apps/api/crates/db/src/lib.rs`:

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

`apps/api/crates/db/src/bin/reset_dev_db.rs`:

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

### Example Pattern

Your project might use this exact pattern:
- `apps/api/crates/db/migrations/` - Production migrations
- `apps/api/crates/db/migrations_dev/` - Development seeds
- `apps/api/crates/db/src/lib.rs` - `run_dev_seeds()` function
- `apps/api/crates/db/src/bin/reset_dev_db.rs` - Reset script

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
        uuid::Uuid::now_v7().to_string().replace('-', "_")
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

- [ ] Documentation updated in `docs/database/` FIRST
- [ ] Design reviewed (constraints, indexes, relationships)
- [ ] No `SET search_path` statements
- [ ] All table references include schema prefix (e.g., `auth.users`)
- [ ] All type references include schema prefix (e.g., `auth.user_status`)
- [ ] All index definitions use qualified table names
- [ ] All foreign keys use qualified table names
- [ ] All cross-schema references are explicit
- [ ] Migration matches documentation exactly

### Benefits of Schema Qualification

1. **Reliability**: Works the same in all environments
2. **Clarity**: Easy to see which schema owns each object
3. **Safety**: No ambiguity about object location
4. **Maintenance**: Easier to reorganize schemas later
5. **CI/CD**: No environment-specific behavior

## Rich Text Field Conventions

When storing rich text content in the database, follow these conventions based on the PostgreSQL column type:

### TEXT Columns → Plain Markdown

Use `TEXT` columns for simple rich text that can be represented as plain Markdown:

```sql
CREATE TABLE learning.module (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    learning_aims TEXT,       -- Plain Markdown text
    key_takeaways TEXT,       -- Plain Markdown text
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

- **Storage**: Plain text with Markdown formatting
- **Frontend editor**: `MarkdownEditor` component (simple textarea with preview)
- **API type**: `String` / `Option<String>` in Rust, `string | null` in TypeScript
- **Use case**: Learning aims, key takeaways, simple notes, summaries without complex structure

### JSONB Columns → Nightfire Structured Content

Use `JSONB` columns for complex structured content that requires block-based editing:

```sql
CREATE TABLE learning.module (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description JSONB,        -- Nightfire structured content
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

- **Storage**: Nightfire JSON with schema, blocks, versions, and content hashes
- **Frontend editor**: `NightfireEditor` component (block-based editor)
- **API type**: `serde_json::Value` / `Option<serde_json::Value>` in Rust, `NightfireValue | null` in TypeScript
- **Use case**: Descriptions, article bodies, complex content with multiple block types (paragraphs, headings, images, etc.)

### Decision Guide

| Scenario | Column Type | Editor |
|----------|-------------|--------|
| Simple text with basic formatting (bold, italic, lists) | `TEXT` | `MarkdownEditor` |
| Single-purpose content (e.g., learning aims) | `TEXT` | `MarkdownEditor` |
| Complex content with multiple block types | `JSONB` | `NightfireEditor` |
| Content requiring validation strategies | `JSONB` | `NightfireEditor` |
| Content with embedded media or custom blocks | `JSONB` | `NightfireEditor` |

### Example: Module Fields

```sql
-- learning.module table
learning_aims TEXT,       -- Markdown: simple bulleted list of aims
key_takeaways TEXT,       -- Markdown: simple bulleted list of takeaways  
description JSONB,        -- Nightfire: rich description with paragraphs, headings, etc.
```

**Key principle**: If the content is fundamentally a simple text blob with formatting, use `TEXT` and Markdown. If it requires structured blocks, validation, or complex editing, use `JSONB` and Nightfire.

## Underlay Database Utilities (`underlay-db`)

The `underlay-db` crate provides common database utilities to reduce boilerplate.

### ExistsCheck Builder (Recommended)

The `ExistsCheck` builder provides flexible existence checks with support for composite constraints:

```rust
use underlay_db::ExistsCheck;

// Simple: check if slug exists
let exists = ExistsCheck::new("content", "summary_item")
    .value("slug", "my-slug")
    .check(&pool)
    .await?;

// For updates: exclude current record
let exists = ExistsCheck::new("content", "summary_item")
    .value("slug", "my-slug")
    .excluding(current_id)
    .check(&pool)
    .await?;

// Composite: slug + nullable year (pathway)
let exists = ExistsCheck::new("learning", "pathway")
    .value("slug", slug)
    .nullable_value("year", year)  // uses IS NOT DISTINCT FROM
    .check(&pool)
    .await?;

// Multi-scope: slug + pathway_id + start_year (module)
let exists = ExistsCheck::new("learning", "module")
    .value("slug", slug)
    .scope("pathway_id", pathway_id)
    .value_i32("start_year", start_year)
    .excluding(current_id)
    .check(&pool)
    .await?;
```

#### ExistsCheck Methods

| Method | Description |
|--------|-------------|
| `value(column, value)` | Add string equality condition |
| `value_i32(column, value)` | Add integer equality condition |
| `scope(column, uuid)` | Add UUID equality condition (FK scope) |
| `nullable_value(column, Option<i32>)` | Add nullable int with `IS NOT DISTINCT FROM` |
| `excluding(id)` | Exclude a specific record (for updates) |
| `include_deleted()` | Skip `deleted_at IS NULL` filter (for tables without soft-delete) |
| `check(&pool)` | Execute and return `Result<bool, sqlx::Error>` |

#### Including Deleted Records

By default, `ExistsCheck` filters out soft-deleted records (`deleted_at IS NULL`). For tables without soft-delete or when you need to check all records:

```rust
// Check existence including deleted records
let exists = ExistsCheck::new("learning", "area")
    .value("slug", slug)
    .include_deleted()
    .check(&pool)
    .await?;
```

### Legacy Helper Functions

For simple cases, convenience functions are also available:

```rust
use underlay_db::{value_exists, value_exists_excluding};

// Check if slug is taken
let exists = value_exists(&pool, "content", "summary_item", "slug", "my-slug").await?;

// Excluding current record (for updates)
let exists = value_exists_excluding(
    &pool, "content", "summary_item", "slug", "my-slug", current_id
).await?;
```

#### Scoped Uniqueness

For values unique within a parent scope:

```rust
use underlay_db::{value_exists_in_scope, value_exists_in_scope_excluding};

// Check if section label exists within module
let exists = value_exists_in_scope(
    &pool, "learning", "section", "label", "Introduction", "module_id", module_id
).await?;
```

#### Integer Column Variants

```rust
use underlay_db::{number_exists_in_scope, number_exists_in_scope_excluding};

// Check if area number exists within section
let exists = number_exists_in_scope(
    &pool, "learning", "area", "number", 5, "section_id", section_id
).await?;
```

### Safety Note

These helpers use format strings for table/column names. Only pass known-good values from your application code - never user input directly.

## See Also

**Related Guides:**
- **[060-authentication.md](./060-authentication.md)** - Auth schema, JWT setup, credentials table
- **[040-rust-backend.md](./040-rust-backend.md)** - Database pool setup, SQL queries in handlers
- **[070-api-handlers.md](./070-api-handlers.md)** - Using database in API handlers
- **[130-testing.md](./130-testing.md)** - Database testing patterns, test fixtures, integration tests
- **[076-nightfire.md](./076-nightfire.md)** - Nightfire structured content system

**Key Topics:**
- **Docs-first workflow**: Document schema changes before writing migrations
- Migration best practices: Never use `SET search_path`
- Schema organization: Group related tables (auth, content, etc.)
- Dev seeds pattern: Separate test data from production migrations
- Connection pooling: Configure for production workloads
- Test database setup: Clean database for integration tests

**Migration Checklist:**
1. Update documentation in `docs/database/` FIRST
2. Review the design in documentation
3. Create schema (`CREATE SCHEMA IF NOT EXISTS ...`)
4. Fully qualify all object names
5. Add indexes for foreign keys and query patterns
6. Use `gen_random_uuid()` for UUIDs (Postgres 13+)
7. Add constraints (NOT NULL, CHECK, UNIQUE)
8. Verify migration matches documentation
9. Test migration in clean database
10. Test rollback if applicable

## Next Steps

Proceed to [060-authentication](./060-authentication.md).

