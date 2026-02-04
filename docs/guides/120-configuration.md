# 120 - Configuration

This document covers configuration management for Underlay applications. Environment variables follow a standard naming convention to ensure consistency across projects.

## Naming Convention

Underlay defines a set of **standard environment variable names** that all consuming applications should use. This enables:

- Consistent configuration across projects
- Documentation that applies to any Underlay app
- Shared tooling that works without app-specific knowledge

**Rules:**

1. **Generic infrastructure** uses unprefixed names: `PORT`, `DATABASE_URL`, `SMTP_HOST`
2. **Auth variables** use `AUTH_` prefix: `AUTH_JWT_PRIVATE_KEY`, `AUTH_GOOGLE_CLIENT_ID`
3. **Frontend public vars** use `PUBLIC_` prefix: `PUBLIC_API_URL`, `PUBLIC_APP_NAME`
4. **App-specific branding** can use app prefix: `MYAPP_EMAIL_SUPPORT`, `MYAPP_THEME`

When migrating from app-prefixed names, the API should accept both the generic name (preferred) and the legacy app-prefixed name as fallback.

---

## API Environment Variables

### Server

| Variable | Default | Description |
|----------|---------|-------------|
| `ENVIRONMENT` | `local` | Runtime environment: `local`, `development`, `staging`, `production`, `test` |
| `HOST` | `127.0.0.1` (local) or `0.0.0.0` | IP address to bind the server |
| `PORT` | `3000` | Port number for the HTTP server |
| `LOG_LEVEL` | `info` | Application log level (also see `RUST_LOG`) |

### Database

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | — | PostgreSQL connection string (required) |

Example: `postgres://user:pass@localhost:5432/myapp`

### CORS & Cookies

| Variable | Default | Description |
|----------|---------|-------------|
| `CORS_ORIGINS` | — | Comma-separated allowed origins for CORS |
| `COOKIE_DOMAIN` | — | Optional domain for auth cookies (e.g., `.example.com`) |
| `COOKIE_SECURE` | `true` in prod | Whether cookies require HTTPS |

### Logging & Tracing

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Rust `tracing` filter directive |
| `RUST_LOG_FORMAT` | `pretty` | Log format: `pretty`, `json`, `compact` |

---

## Authentication

### JWT (underlay-auth-jwt)

Ed25519 keys for signing JWTs. Generate using the script in `docs/guides/code/060-authentication/generate-jwt-env.rs`.

| Variable | Default | Description |
|----------|---------|-------------|
| `AUTH_JWT_PRIVATE_KEY` | — | Base64 PKCS#8 DER private key (required) |
| `AUTH_JWT_PUBLIC_KEY` | — | Base64url raw 32-byte public key (required) |
| `AUTH_JWT_ISSUER` | `underlay` | JWT `iss` claim |
| `AUTH_JWT_AUDIENCE` | — | JWT `aud` claim (optional) |
| `AUTH_ACCESS_TOKEN_LIFETIME_MINUTES` | `15` | Access token expiry |
| `AUTH_REFRESH_TOKEN_LIFETIME_DAYS` | `30` | Refresh token expiry |
| `AUTH_JWT_LEEWAY_SECONDS` | `30` | Clock skew tolerance |

### OAuth (underlay-auth-oauth)

For storing encrypted OAuth refresh tokens:

| Variable | Default | Description |
|----------|---------|-------------|
| `AUTH_OAUTH_SECRET_KEY` | — | Base64 AES-256-GCM key for encrypting refresh tokens |

Google OAuth provider:

| Variable | Default | Description |
|----------|---------|-------------|
| `AUTH_GOOGLE_CLIENT_ID` | — | Google OAuth client ID |
| `AUTH_GOOGLE_CLIENT_SECRET` | — | Google OAuth client secret |
| `AUTH_GOOGLE_REDIRECT_URI` | — | OAuth callback URL |

### WebAuthn

| Variable | Default | Description |
|----------|---------|-------------|
| `WEBAUTHN_RP_ID` | `localhost` | Relying party ID (hostname) |
| `WEBAUTHN_RP_ORIGIN` | `http://localhost:4174` | Relying party origin (full URL) |
| `WEBAUTHN_RP_NAME` | — | Human-readable RP name |

### Password Hashing (Argon2)

| Variable | Default | Description |
|----------|---------|-------------|
| `ARGON2_MEMORY_KB` | `65536` | Memory cost in KB (64 MiB) |
| `ARGON2_ITERATIONS` | `3` | Time cost (iterations) |
| `ARGON2_PARALLELISM` | `4` | Parallelism degree |

---

## Email

| Variable | Default | Description |
|----------|---------|-------------|
| `EMAIL_ADAPTER` | `noop` | Email backend: `noop`, `dev_capture`, `smtp`, `ses` |
| `EMAIL_DEFAULT_FROM` | — | Default sender address |
| `EMAIL_APP_NAME` | — | Application name in email templates |
| `EMAIL_APP_URL` | — | Application URL in email templates |
| `EMAIL_SUPPORT` | — | Support email address |
| `EMAIL_TEMPLATES_DIR` | `templates/emails` | Path to email templates |

### SMTP (when `EMAIL_ADAPTER=smtp`)

| Variable | Default | Description |
|----------|---------|-------------|
| `SMTP_HOST` | `localhost` | SMTP server hostname |
| `SMTP_PORT` | `587` | SMTP server port |
| `SMTP_USERNAME` | — | SMTP authentication username |
| `SMTP_PASSWORD` | — | SMTP authentication password |
| `SMTP_TLS` | `opportunistic` | TLS mode: `required`, `opportunistic`, `none` |

### AWS SES (when `EMAIL_ADAPTER=ses`)

| Variable | Default | Description |
|----------|---------|-------------|
| `SES_REGION` | `eu-west-1` | AWS region |
| `SES_CONFIGURATION_SET` | — | Optional SES configuration set |

### Dev Capture (when `EMAIL_ADAPTER=dev_capture`)

| Variable | Default | Description |
|----------|---------|-------------|
| `EMAIL_WHITELIST` | — | Comma-separated addresses to also deliver |
| `EMAIL_FALLBACK_ADAPTER` | — | Adapter for whitelisted addresses |

---

## Frontend Environment Variables

Frontend apps (SvelteKit) use `PUBLIC_` prefixed variables that are exposed to the browser.

### Web/Admin (.env)

| Variable | Default | Description |
|----------|---------|-------------|
| `PUBLIC_API_URL` | — | Backend API base URL |
| `PUBLIC_API_VERSION` | — | API version sent as `X-Api-Version` header (date format) |
| `PUBLIC_APP_NAME` | — | Application display name |

### API Versioning (Date Format)

Underlay uses **date-based API versioning** in the format `YYYY-MM-DD` (e.g., `2025-01-01`). This approach offers several advantages over traditional major version numbers like `v1`, `v2`:

1. **Granular changes** — Each API change can be tied to a specific date, allowing incremental updates rather than large breaking version bumps
2. **Clear timeline** — The version date indicates when the API contract was established, making it easy to understand compatibility
3. **Gradual deprecation** — Old date versions can be deprecated with clear sunset dates (e.g., "versions before 2024-06-01 will be removed on 2025-01-01")
4. **No version inflation** — Avoids the awkwardness of `v47` or deciding what constitutes a "major" version

The version is sent to the API via the `X-Api-Version` header. The API can use this to:
- Return responses in the expected format for that version
- Apply version-specific validation rules
- Log which versions are still in use for deprecation planning

**Convention:** Use the date when the API contract was defined or last changed in a breaking way. For new projects, use the project start date or a recent date like `2025-01-01`.

Example:

```bash
PUBLIC_API_URL=http://localhost:3000
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

---

## Example .env Files

### API (Development)

```bash
# Server
ENVIRONMENT=local
HOST=127.0.0.1
PORT=3000

# Database
DATABASE_URL=postgres://root@127.0.0.1:5432/myapp

# Logging
RUST_LOG=debug

# Auth - JWT
AUTH_JWT_PRIVATE_KEY=MC4CAQAwBQYDK2VwBCIEI...
AUTH_JWT_PUBLIC_KEY=a0tbP2NAEjG6vM3fVf36C1...

# Email (dev mode - capture to database)
EMAIL_ADAPTER=dev_capture
```

### API (Production)

```bash
# Server
ENVIRONMENT=production
HOST=0.0.0.0
PORT=3000

# Database
DATABASE_URL=postgres://user:pass@db.example.com:5432/myapp

# CORS
CORS_ORIGINS=https://app.example.com,https://admin.example.com
COOKIE_DOMAIN=.example.com
COOKIE_SECURE=true

# Auth - JWT (use strong production keys!)
AUTH_JWT_PRIVATE_KEY=...
AUTH_JWT_PUBLIC_KEY=...
AUTH_JWT_ISSUER=myapp
AUTH_JWT_AUDIENCE=myapp-api

# Email (production SMTP)
EMAIL_ADAPTER=smtp
EMAIL_DEFAULT_FROM=noreply@example.com
EMAIL_APP_NAME=MyApp
EMAIL_APP_URL=https://app.example.com
EMAIL_SUPPORT=support@example.com
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=smtp-user
SMTP_PASSWORD=smtp-pass
SMTP_TLS=required
```

### Frontend (Development)

```bash
PUBLIC_API_URL=http://localhost:3000
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

### Frontend (Production)

```bash
PUBLIC_API_URL=https://api.example.com
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

---

## Migration from App-Prefixed Names

If your application previously used app-prefixed names (e.g., `MYAPP_DATABASE_URL`), you can migrate gradually:

1. Update your config loading to check the generic name first, then fall back to the legacy name
2. Update your `.env` files to use the generic names
3. Update documentation to reference the generic names
4. Eventually remove the fallback code

Example fallback pattern (Rust):

```rust
fn env_var_with_fallback(primary: &str, fallbacks: &[&str]) -> Option<String> {
    std::env::var(primary).ok().or_else(|| {
        for fallback in fallbacks {
            if let Ok(val) = std::env::var(fallback) {
                return Some(val);
            }
        }
        None
    })
}

// Usage
let db_url = env_var_with_fallback("DATABASE_URL", &["MYAPP_DATABASE_URL"]);
```

---

## Configuration Validation

See code examples in `/code/120-configuration/` for validation patterns.

## Next Steps

- [130-testing.md](./130-testing.md)
