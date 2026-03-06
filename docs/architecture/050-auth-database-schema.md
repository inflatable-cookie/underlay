# Auth Database Schema

This document details the database schema for the Underlay authentication system. The schema is designed to be app-agnostic while supporting all auth methods (password, TOTP, PassKey, OAuth).

## Schema Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           auth_users                                         │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │ id (UUID), email, display_name, status, created_at, updated_at        │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                    │                                         │
│          ┌─────────────────────────┼─────────────────────────┐              │
│          ▼                         ▼                         ▼              │
│  ┌───────────────┐        ┌───────────────┐        ┌───────────────┐       │
│  │auth_credentials│       │auth_sessions  │       │auth_audit_log │       │
│  └───────────────┘        └───────────────┘        └───────────────┘       │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Core Tables

### auth_users

Stores user accounts.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | User identifier |
| email | VARCHAR(255) | NOT NULL UNIQUE | User email address |
| display_name | VARCHAR(255) | NOT NULL | User's display name |
| status | VARCHAR(50) | NOT NULL DEFAULT 'active' | Account status (active, suspended, deleted) |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Creation timestamp |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update timestamp |

### auth_credentials

Stores authentication methods for each user.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | Credential identifier |
| user_id | UUID | NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE | Owning user |
| type | VARCHAR(50) | NOT NULL CHECK (type IN ('password', 'totp', 'passkey', 'oauth_google')) | Credential type |
| secret_encrypted | TEXT | NOT NULL | Encrypted credential data |
| metadata | JSONB | NOT NULL DEFAULT '{}' | Type-specific metadata |
| verified | BOOLEAN | NOT NULL DEFAULT FALSE | Whether credential is verified |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Creation timestamp |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update timestamp |
| last_used_at | TIMESTAMPTZ | NULL | Last usage timestamp |

### auth_sessions

Stores active sessions.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | Session identifier |
| user_id | UUID | NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE | Owning user |
| access_token_fingerprint | VARCHAR(64) | NOT NULL | Hash of access token for lookup |
| refresh_token_fingerprint | VARCHAR(64) | NOT NULL | Hash of refresh token for lookup |
| access_token_expires_at | TIMESTAMPTZ | NOT NULL | Access token expiration |
| refresh_token_expires_at | TIMESTAMPTZ | NOT NULL | Refresh token expiration |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Creation timestamp |
| last_used_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last activity timestamp |
| ip_address | INET | NULL | Client IP address |
| user_agent | TEXT | NULL | Client user agent |
| revoked | BOOLEAN | NOT NULL DEFAULT FALSE | Whether session is revoked |
| revocation_reason | VARCHAR(100) | NULL | Reason for revocation |
| revoked_at | TIMESTAMPTZ | NULL | Revocation timestamp |

### auth_audit_log

Stores auth events for security review.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | Log entry identifier |
| event_type | VARCHAR(100) | NOT NULL | Event type code |
| user_id | UUID | NULL REFERENCES auth_users(id) ON DELETE SET NULL | Related user |
| session_id | UUID | NULL REFERENCES auth_sessions(id) ON DELETE SET NULL | Related session |
| ip_address | INET | NULL | Client IP address |
| user_agent | TEXT | NULL | Client user agent |
| success | BOOLEAN | NOT NULL | Whether event succeeded |
| details | JSONB | NOT NULL DEFAULT '{}' | Event-specific details |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Event timestamp |

### auth_rate_limits

Rate limiting for brute force protection.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| key | VARCHAR(255) | PRIMARY KEY | Rate limit key |
| count | INTEGER | NOT NULL DEFAULT 0 | Request count in window |
| window_start | TIMESTAMPTZ | NOT NULL | Window start time |
| expires_at | TIMESTAMPTZ | NOT NULL | Key expiration |

## Optional Tables

### auth_backup_codes

One-time recovery codes for TOTP.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | Code identifier |
| user_id | UUID | NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE | Owning user |
| code_hash | VARCHAR(255) | NOT NULL UNIQUE | Hashed backup code |
| used_at | TIMESTAMPTZ | NULL | When code was used |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Creation timestamp |

### auth_oauth_connections

External OAuth provider connections.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY DEFAULT gen_random_uuid() | Connection identifier |
| user_id | UUID | NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE | Owning user |
| provider | VARCHAR(50) | NOT NULL CHECK (provider IN ('google')) | OAuth provider |
| provider_user_id | VARCHAR(255) | NOT NULL | Provider's user ID |
| access_token_encrypted | TEXT | NOT NULL | Encrypted access token |
| refresh_token_encrypted | TEXT | NULL | Encrypted refresh token |
| token_expires_at | TIMESTAMPTZ | NULL | Token expiration |
| scopes | TEXT[] | NULL | Granted scopes |
| connected_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Connection timestamp |
| last_used_at | TIMESTAMPTZ | NULL | Last usage timestamp |

## Credential Storage

### Password

- `type`: `'password'`
- `secret_encrypted`: Argon2id hash
- `metadata`: `{ algorithm: 'argon2id', memory_kb: 65536, iterations: 3, parallelism: 4 }`

### TOTP

- `type`: `'totp'`
- `secret_encrypted`: AES-256-GCM encrypted secret
- `metadata`: `{ issuer: 'AppName', algorithm: 'SHA1', digits: 6, period: 30 }`

### PassKey

- `type`: `'passkey'`
- `secret_encrypted`: CBOR-encoded credential data
- `metadata`: `{ credential_id: 'base64...', transports: ['platform'], last_counter: 0 }`

### OAuth

- Stored in `auth_oauth_connections` with tokens
- `auth_credentials` has `type: 'oauth_google'` as marker

## Audit Event Types

| Event Type | Description |
|------------|-------------|
| `auth.register` | New user registration |
| `auth.login.attempt` | Login attempt |
| `auth.login.password` | Password login |
| `auth.login.totp` | TOTP verification |
| `auth.login.passkey` | PassKey authentication |
| `auth.login.oauth` | OAuth authentication |
| `auth.logout` | User logged out |
| `auth.session.refresh` | Token refresh |
| `auth.session.revoke` | Session revoked |
| `auth.2fa.enable` | 2FA enabled |
| `auth.2fa.disable` | 2FA disabled |
| `auth.passkey.register` | PassKey registered |
| `auth.passkey.delete` | PassKey deleted |
| `auth.oauth.connect` | OAuth connected |
| `auth.oauth.disconnect` | OAuth disconnected |
| `auth.password.change` | Password changed |
| `auth.rate_limit.exceeded` | Rate limit triggered |

## Rate Limiting Keys

- `login:{email}` - Login attempts per email
- `login:{ip}` - Login attempts per IP
- `register:{ip}` - Registration attempts per IP
- `2fa:{user_id}` - 2FA verification attempts
- `password_reset:{email}` - Password reset requests

## Indexes

| Table | Index | Purpose |
|-------|-------|---------|
| auth_credentials | idx_auth_credentials_user_id | Find credentials by user |
| auth_sessions | idx_auth_sessions_user_id | Find sessions by user |
| auth_sessions | idx_auth_sessions_access_fingerprint | Validate access token |
| auth_sessions | idx_auth_sessions_refresh_fingerprint | Refresh session |
| auth_sessions | idx_auth_sessions_expires_at | Clean expired sessions |
| auth_audit_log | idx_auth_audit_log_user_id | Audit by user |
| auth_audit_log | idx_auth_audit_log_ip_address | Audit by IP |
| auth_audit_log | idx_auth_audit_log_created_at | Audit by time |

## Security Considerations

1. Use Argon2id for password hashing (memory: 64MB, iterations: 3, parallelism: 4)
2. Encrypt TOTP secrets and OAuth tokens with AES-256-GCM
3. Store only token fingerprints (hashes), never plain tokens
4. Use `TIMESTAMPTZ` for all timestamps (timezone-aware)
5. Support both IPv4 and IPv6 with `INET` type
6. Consider audit log retention policy (90 days active, 1 year archived)

## Application Extensions

Apps can extend tables with additional columns:

```sql
-- Songsprout: Link users to artists
ALTER TABLE auth_users ADD COLUMN artist_id UUID REFERENCES artists(id);

-- Acowtancy: Add role column
ALTER TABLE auth_users ADD COLUMN role VARCHAR(50) DEFAULT 'student';
```

Underlay provides the base schema; apps own their domain-specific extensions.

## Migrations

See `docs/roadmaps/g01/004-underlay-auth-system-roadmap.md` for migration SQL files.
