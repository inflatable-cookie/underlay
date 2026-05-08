# Auth Database Schema

Status: active reference
Owner: repo maintainers

This document describes the live shared auth persistence model.

The canonical source of truth is the migration:

- [rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/migrations/0001_create_auth_tables.sql:1)

The shared Rust type surface in
[rust/crates/underlay-auth/src/types.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/types.rs:1)
is the matching application model.

If this doc drifts from either of those, the migration and shared types win.

## Boundary

`auth` owns authentication mechanics:

- user account row
- credentials
- sessions
- multi-step auth state
- TOTP replay/backup-code support

It does not own full identity and personalization. That belongs in
`account.user_profile`; see
[055-account-database-schema.md](./055-account-database-schema.md).

## Schema Overview

```
auth.users
  ├─ auth.credentials
  ├─ auth.sessions
  └─ auth.auth_state

auth.credentials
  └─ auth.totp_credential
```

## Core Tables

### auth.users

Stores the shared authentication account row.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | `UUID` | PK | Shared user identifier |
| `email` | `TEXT` | NOT NULL, UNIQUE | Login email |
| `role` | `TEXT` | NOT NULL, checked enum | Coarse primary role |
| `status` | `TEXT` | NOT NULL, checked enum | `active`, `suspended`, `deleted` |
| `created_at` | `TIMESTAMPTZ` | NOT NULL | Creation time |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL | Last update time |

Notes:

- there is no `display_name` column in the live shared auth user table
- `role` exists in the canonical migration even though higher auth logic also
  supports multi-role principals and session role snapshots
- richer profile and naming data belongs in `account.user_profile`

### auth.credentials

Stores user authentication methods.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | `UUID` | PK | Credential identifier |
| `user_id` | `UUID` | NOT NULL, FK -> `auth.users` | Owning user |
| `type` | `TEXT` | NOT NULL, checked enum | `password`, `totp`, `passkey`, `oauth_google` |
| `secret_encrypted` | `TEXT` | NOT NULL | Hash or encrypted blob |
| `metadata` | `JSONB` | NOT NULL | Shared `CredentialMetadata` payload |
| `verified` | `BOOLEAN` | NOT NULL | Verification state |
| `display_name` | `TEXT` | NULL | Optional user-facing label, mainly for devices |
| `created_at` | `TIMESTAMPTZ` | NOT NULL | Creation time |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL | Last update time |
| `last_used_at` | `TIMESTAMPTZ` | NULL | Last use time |

Rules:

- password, TOTP, and OAuth Google are unique per user
- passkeys allow multiple credentials per user
- passkey credential IDs are globally unique through the metadata index

### auth.sessions

Stores active and historical shared session state.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | `UUID` | PK | Session identifier |
| `user_id` | `UUID` | NOT NULL, FK -> `auth.users` | Owning user |
| `roles` | `JSONB` | NOT NULL | Session role snapshot |
| `is_active` | `BOOLEAN` | NOT NULL | Fast active flag |
| `access_token_fingerprint` | `TEXT` | NOT NULL | Access-token fingerprint |
| `refresh_token_fingerprint` | `TEXT` | NOT NULL | Refresh-token fingerprint |
| `refresh_token_id` | `UUID` | NOT NULL | Current refresh token id |
| `refresh_token_version` | `INTEGER` | NOT NULL | Rotation version counter |
| `access_token_expires_at` | `TIMESTAMPTZ` | NOT NULL | Access expiry |
| `refresh_token_expires_at` | `TIMESTAMPTZ` | NOT NULL | Refresh expiry |
| `created_at` | `TIMESTAMPTZ` | NOT NULL | Creation time |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL | Last update time |
| `last_used_at` | `TIMESTAMPTZ` | NOT NULL | Last use time |
| `ip_address` | `TEXT` | NULL | Client IP |
| `user_agent` | `TEXT` | NULL | Client user agent |
| `status` | `TEXT` | NOT NULL, checked enum | `active`, `revoked`, `expired` |
| `revocation_reason` | `TEXT` | NULL | Revocation reason |
| `revoked_at` | `TIMESTAMPTZ` | NULL | Revocation time |

Notes:

- the live shared session model includes both `status` and `is_active`
- refresh rotation state is first-class in the schema
- role snapshots live on sessions, not only on users

### auth.auth_state

Stores short-lived workflow state for multi-step auth flows.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `id` | `UUID` | PK | State identifier |
| `user_id` | `UUID` | NULL, FK -> `auth.users` | Related user when present |
| `state_type` | `TEXT` | NOT NULL | Workflow state kind |
| `state` | `JSONB` | NOT NULL | Opaque workflow payload |
| `created_at` | `TIMESTAMPTZ` | NOT NULL | Creation time |
| `expires_at` | `TIMESTAMPTZ` | NOT NULL | Expiry time |

Purpose:

- OAuth state
- WebAuthn start/finish state
- other expiring cross-request auth workflow state

### auth.totp_credential

Stores TOTP replay and recovery support alongside a TOTP credential.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `credential_id` | `UUID` | PK, FK -> `auth.credentials` | TOTP credential id |
| `last_counter` | `BIGINT` | NOT NULL | Last accepted counter |
| `backup_code_hashes` | `JSONB` | NOT NULL | Stored backup-code hashes |

## Shared Types Mapping

The live shared Rust types are similar but not identical to raw schema rows.

Important differences:

- `User.display_name` is optional in the shared type even though the canonical
  auth table no longer stores it directly; apps may project profile data into
  the type
- `Session` in shared types is a cleaner app-facing shape and does not expose
  every low-level rotation column directly
- `CredentialMetadata` is the app-facing contract for the `metadata` JSONB
  column

## Security Model

- store only token fingerprints, never raw session tokens
- refresh rotation state is durable and replay-sensitive
- secrets and provider tokens live in encrypted or hashed form inside
  `secret_encrypted`
- timestamps are UTC `TIMESTAMPTZ`

## Extension Rule

Apps may add app-local tables or columns, but they should not fork the meaning
of the shared core tables silently.

Preferred extension pattern:

- keep `auth` focused on authentication mechanics
- put identity/personalization into `account`
- put product/domain state in app-local schemas
