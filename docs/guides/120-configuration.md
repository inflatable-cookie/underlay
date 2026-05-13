# 120 - Configuration

This document defines the canonical configuration model for Underlay applications.

Primary rule: keep stable app behavior settings in typed Rust config structures
backed by committed `config/*.toml` files. Keep true secrets in a declared
secret manager. For Effigy-managed Underlay apps, the local default is the
Effigy `[secrets]` vault, not loose `.env` files.

Rollout/enforcement assets for consumers are in `docs/guides/121-consumer-config-rollout-kit.md`.

## Configuration Classes

Classify every setting before adding it:

| Class | Where it belongs | Examples |
|---|---|---|
| `secret` | Secret manager only; Effigy `[secrets]` vault for local Effigy-managed apps | API keys, JWT private keys, SMTP passwords |
| `runtime-env` | Environment (deployment specific) | `DATABASE_URL`, `HOST`, `PORT`, public base URLs |
| `app-behavior` | Typed Rust config + committed defaults | pagination limits, retries, feature knobs, timeout defaults |

If a value is not secret and does not need to vary per environment, it should not live in `.env`.

For local Effigy-managed apps, split values further:

- true secrets are declared under root `[secrets.keys]` and stored with
  `effigy secrets`
- local ports, hostnames, service names, bucket names, regions, and public URLs
  should come from bundle defaults, typed config, `config/dev.toml`, optional
  `config/local.toml`, or generated `.effigy/runtime/` files
- `.env` may remain as a compatibility bridge while an app still reads env
  directly, but it should not be the long-term source of truth for true secrets

Recommended Effigy vault declaration:

```toml
[secrets]
backend = "effigy-vault"

[secrets.vault]
path = ".effigy/secrets/local.vault"
identity = "passphrase"
unlock = "passphrase"

[secrets.keys.database_url]
required = false
targets = ["tasks", "containers"]
description = "Application database connection URL."
```

Target meanings:

| Target | Use |
|---|---|
| `tasks` | Host task process env injection |
| `containers` | Compose/runtime process env injection |
| `state` | State apply/capture hook injection |
| `artifacts` | Artifact workflow injection |
| `deploy` | Deploy provider script injection |
| `rhai` | Explicit Rhai script `secrets::get(...)` access |

Set `required = false` during transition while legacy `.env` bridges still
exist. Set `required = true` once the vault path is normal for all developers
and runtime surfaces.

## Canonical Load Order

Use this precedence (lowest to highest):

1. Rust struct defaults
2. `config/default.toml` (committed)
3. `config/<environment>.toml` (committed where shared)
4. `config/local.toml` (optional personal patch, gitignored)
5. Environment overrides (allowlisted keys only)

`<environment>` should match the named Effigy environment where one exists:

- `dev.toml`
- `uat.toml`
- `production.toml`

Use `UNDERLAY_ENV` to select the named overlay at runtime. Deployed services
should set `UNDERLAY_ENV` to the same name used by `[deploy.<environment>]`.
Local development should normally use `UNDERLAY_ENV=dev`.

`local.toml` is not an environment. It is a developer-local last-mile override
for non-secret values that should not be shared.

Recommended behavior:

- Fail fast at startup on invalid config
- Emit clear error messages with field names
- Log effective config in redacted form (never print secrets)

## Recommended Project Layout

```text
apps/api/
  config/
    default.toml
    dev.toml
    uat.toml
    production.toml
    local.toml.example
  crates/
    app-config/
      src/lib.rs
```

`app-config` should expose typed sections such as `ServerConfig`, `AuthConfig`, `EmailConfig`, and app-domain sections for behavior toggles/limits.

Apps may use `underlay-config` for file stacking:

```rust
use underlay_config::ConfigStack;

let config: AppConfig = ConfigStack::from_project_root(".")
    .with_environment_from_env()
    .with_optional_local_overlay("local")
    .load()?;
```

`underlay-config` only owns TOML stacking. Apps still own typed structs,
validation, redacted diagnostics, and explicit env override allowlists.

## Environment Key Policy

- Keep an explicit allowlist of supported env keys per app
- Reject or warn on unknown app env keys
- Do not read `std::env::var` throughout handlers/services; read env in config bootstrap only
- Keep deprecation windows for renamed keys (warn first, remove later)

## Per-App Migration Checklist (Reusable)

For each consuming app:

1. Inventory all env keys currently read.
2. Classify each key (`secret`, `runtime-env`, `app-behavior`).
3. Move `app-behavior` keys into typed Rust config with defaults.
4. Add compatibility bridge for legacy env keys with deprecation warnings.
5. Add or update `config/dev.toml`, `config/uat.toml`, and
   `config/production.toml` where shared environment behavior differs.
6. Declare true local secrets in root `[secrets.keys]` with the right Effigy
   targets.
7. Update docs and `.env.example` to remove migrated keys.
8. Enforce allowlist/guardrails in CI.
9. Remove deprecated keys after the agreed transition window.

### Reference Migration Feedback (2026-02-25)

Applied from `underlay-reference/acme-api` migration:

1. Remove migrated behavior keys from `.env.example` as soon as typed defaults exist in `config/default.toml`.
2. Keep JWT key material in env, but source JWT behavior from typed config using `JwtConfig::from_values(...)`.
3. Add startup warnings for legacy behavior env keys that are now ignored, with typed replacement field names.
4. Keep focused bootstrap tests that assert migrated behavior env keys do not override typed config.

## Rollout PR Template (Copy/Paste)

```md
## Config Migration Scope
- App: <name>
- Phase: <inventory|typed-structs|deprecation|cleanup>

## Env Key Changes
- Moved to typed config: <keys>
- Remaining env keys (secret/runtime-env): <keys>
- Deprecated keys (warning only): <keys>

## Validation
- Startup validation result:
- Redacted config diagnostics reviewed:

## Follow-ups
- Deprecation removal target release:
```

---

## Standard Environment Variables

The sections below list commonly used env keys that remain valid for `secret` and `runtime-env` usage.

## Naming Convention

Underlay defines a set of **standard environment variable names** that all consuming applications should use. This enables:

- Consistent configuration across projects
- Documentation that applies to any Underlay app
- Shared tooling that works without app-specific knowledge

**Rules:**

1. **Generic infrastructure** uses unprefixed names: `PORT`, `DATABASE_URL`, `SMTP_HOST`
2. **Auth variables** use `AUTH_` prefix: `AUTH_JWT_PRIVATE_KEY`, `AUTH_GOOGLE_CLIENT_ID`
3. **Frontend public vars** use `PUBLIC_` prefix: `PUBLIC_API_BASE_URL`, `PUBLIC_API_VERSION`, `PUBLIC_APP_NAME`
4. **App-specific branding** can use app prefix: `MYAPP_EMAIL_SUPPORT`, `MYAPP_THEME`

When migrating from app-prefixed names, the API should accept both the generic name (preferred) and the legacy app-prefixed name as fallback.

---

## API Environment Variables

### Server

| Variable | Default | Description |
|----------|---------|-------------|
| `ENVIRONMENT` | `local` | Runtime environment: `local`, `development`, `staging`, `production`, `test` |
| `HOST` | `127.0.0.1` (local) or `0.0.0.0` | IP address to bind the server socket |
| `PORT` | `3000` | Port number for the HTTP server |
| `PUBLIC_HOST` | `HOST` value | Hostname for constructing URLs (e.g., `localhost`, `api.example.com`) |
| `LOG_LEVEL` | `info` | Application log level (also see `RUST_LOG`) |

**Note on HOST vs PUBLIC_HOST:**

`HOST` must be a valid IP address for socket binding (e.g., `127.0.0.1`, `0.0.0.0`). `PUBLIC_HOST` is used when constructing URLs that need to be accessible from browsers or external clients. For local development, set `HOST=localhost` in `.env` if you want browser-compatible URLs (the socket will bind to `127.0.0.1` by default).

Common patterns:
- **Local dev:** `HOST` unset (defaults to `127.0.0.1`), `PUBLIC_HOST=localhost` for browser URLs
- **Containers/proxies:** `HOST=0.0.0.0` (bind all interfaces), `PUBLIC_HOST=api.example.com`
- **Production:** `HOST=0.0.0.0`, `PUBLIC_HOST` set to the public domain name

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

Preferred migration pattern: keep JWT key material in env, and move behavior defaults (`issuer`, lifetimes, audience, leeway) to typed config.

```rust
use underlay_auth_jwt::{JwtBehaviorDefaults, JwtConfig, JwtService};

let jwt_defaults = JwtBehaviorDefaults {
    access_token_lifetime_minutes: 15,
    refresh_token_lifetime_days: 30,
    issuer: "myapp-api".to_string(),
    audience: Some("myapp-clients".to_string()),
    leeway_seconds: 30,
};

let private_key_b64 = std::env::var("AUTH_JWT_PRIVATE_KEY")?;
let public_key_b64 = std::env::var("AUTH_JWT_PUBLIC_KEY")?;
let jwt_config = JwtConfig::from_values(private_key_b64, public_key_b64, jwt_defaults);
let jwt_service = JwtService::new(jwt_config)?;
```

This keeps these keys as env-only secrets:

- `AUTH_JWT_PRIVATE_KEY`
- `AUTH_JWT_PUBLIC_KEY`

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

### Auth Behavior Migration Pattern

Use one typed auth behavior config at bootstrap so non-secret auth behavior does not drift across env files.

Keep in env (`secret` / `runtime-env`):

- `AUTH_JWT_PRIVATE_KEY`
- `AUTH_JWT_PUBLIC_KEY`
- `AUTH_GOOGLE_CLIENT_ID`
- `AUTH_GOOGLE_CLIENT_SECRET`
- `AUTH_GOOGLE_REDIRECT_URI`

Move to typed behavior config (`app-behavior`):

- JWT tuning: issuer, audience, lifetimes, leeway
- WebAuthn RP settings: id, origin, name
- Argon2 params: memory, iterations, parallelism
- OAuth provider scopes

Recommended bootstrap shape:

```rust
use underlay_auth::hashing::Argon2Hasher;
use underlay_auth_jwt::{JwtBehaviorDefaults, JwtConfig, JwtService};
use underlay_auth_oauth::{GoogleOAuthConfig, GoogleOAuthService};
use underlay_auth_webauthn::{WebAuthnConfig, WebAuthnService};

let jwt_defaults = JwtBehaviorDefaults {
    access_token_lifetime_minutes: 15,
    refresh_token_lifetime_days: 30,
    issuer: "myapp-api".to_string(),
    audience: Some("myapp-clients".to_string()),
    leeway_seconds: 30,
};

let jwt_private_key = std::env::var("AUTH_JWT_PRIVATE_KEY")?;
let jwt_public_key = std::env::var("AUTH_JWT_PUBLIC_KEY")?;
let jwt = JwtService::new(JwtConfig::from_values(
    jwt_private_key,
    jwt_public_key,
    jwt_defaults,
))?;

let argon2 = Argon2Hasher::with_params(65536, 3, 4);
let webauthn = WebAuthnService::new(WebAuthnConfig {
    rp_id: "myapp.com".to_string(),
    rp_origin: "https://myapp.com".to_string(),
    rp_name: "My App".to_string(),
})?;

let oauth = GoogleOAuthService::new(GoogleOAuthConfig {
    client_id: std::env::var("AUTH_GOOGLE_CLIENT_ID")?,
    client_secret: std::env::var("AUTH_GOOGLE_CLIENT_SECRET")?,
    redirect_uri: std::env::var("AUTH_GOOGLE_REDIRECT_URI")?,
    scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
})?;
```

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
| `PUBLIC_API_BASE_URL` | — | Canonical backend API base URL |
| `PUBLIC_API_VERSION` | — | API version sent as `X-Api-Version` header (date format) |
| `PUBLIC_APP_NAME` | — | Application display name |

Legacy migration fallbacks (deprecated):

- `PUBLIC_API_URL`
- `VITE_API_URL`

Use these only during transition windows. New projects should read
`PUBLIC_API_BASE_URL` directly.

Recommended resolver pattern:

```ts
const baseUrl =
  env.PUBLIC_API_BASE_URL ??
  env.PUBLIC_API_URL ??
  env.VITE_API_URL ??
  "http://127.0.0.1:3000";

const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";
```

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
PUBLIC_API_BASE_URL=http://localhost:3000
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

---

## Example .env Files

### API (Development)

```bash
# Server
ENVIRONMENT=local
PORT=3000
PUBLIC_HOST=localhost  # For browser-compatible URLs

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
PUBLIC_HOST=api.example.com  # Public domain for URLs

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
PUBLIC_API_BASE_URL=http://localhost:3000
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

### Frontend (Production)

```bash
PUBLIC_API_BASE_URL=https://api.example.com
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
