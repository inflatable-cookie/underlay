# 005 – Auth Database Migrations

Create SQL migrations for the auth system. Each migration is a numbered SQL file that products can include in their migration directories.

## Migration Checklist

- [ ] Create `YYYYMMDDHHMMSS__create_auth_tables.sql`
  - `auth_users` table
  - `auth_credentials` table
  - `auth_sessions` table
  - `auth_audit_log` table
  - `auth_rate_limits` table
  - Indexes for performance
  - `updated_at` trigger function

- [ ] Create `YYYYMMDDHHMMSS__create_auth_backup_codes.sql`
  - `auth_backup_codes` table
  - Indexes

- [ ] Create `YYYYMMDDHHMMSS__create_auth_oauth_connections.sql`
  - `auth_oauth_connections` table
  - Indexes

- [ ] Create `YYYYMMDDHHMMSS__seed_test_data.sql` (optional, for dev)
  - Test user with password credential
  - Sample audit log entries

## Migration Naming Convention

Format: `YYYYMMDDHHMMSS__description.sql`

Example: `20260108120000__create_auth_tables.sql`

## Running Migrations

Products should include these migrations in their existing migration directories:
- Songsprout: `nursery/migrations/`
- Acowtancy: `farmyard/migrations/`

Reference: `docs/architecture/050-auth-database-schema.md` for schema details.
