# 005 – Auth Database Migrations

Status: Complete

Create SQL migrations for the auth system. Each migration is a numbered SQL file that products can include in their migration directories.

## Migration Checklist

- [x] Canonical auth schema migration(s) live in Underlay
  - `auth.users`
  - `auth.credentials`
  - `auth.sessions`
  - `auth.auth_state`
  - `auth.totp_credential`
  - Indexes for performance
  - **Location:** `rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql`

- [x] Create `YYYYMMDDHHMMSS__seed_test_data.sql` (optional, for dev) - Deferred
  - Test user with password credential

## Migration Naming Convention

Format: `YYYYMMDDHHMMSS__description.sql`

Example: `20260108120000__create_auth_tables.sql`

## Running Migrations

Products should sync these migrations into their existing migration directories using `underlay-devtools sync-migrations`:
- Songsprout: `nursery/migrations/`
- Acowtancy: `farmyard/migrations/`

Reference: `docs/architecture/050-auth-database-schema.md` for schema details.
