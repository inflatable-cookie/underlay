# Acowtancy Env Inventory and Classification (20.2.1)

Date: 2026-02-24  
Roadmap: `docs/roadmap/020-configuration-standardization-and-env-reduction.md`

## Scope

This inventory covers the Acowtancy surfaces called out in Phase 20.2.1:

- `../../acowtancy/farmyard`
- `../../acowtancy/dairy`
- `../../acowtancy/cream`
- `../../acowtancy/cattle-grid`
- jobs surface: `../../acowtancy/farmyard/crates/jobs`

It combines:

1. Declared keys in `.env` and `.env.example` files.
2. Direct env reads in Rust/TypeScript/Svelte sources.
3. Indirect env reads via Underlay auth loaders used by Farmyard (`JwtConfig::from_env`, `GoogleOAuthService::from_env`, `OAuthTokenCipher::from_env_optional`).

## Inventory summary

- 67 unique env keys observed.
- Classification totals:
  - `secret`: 8
  - `runtime-env`: 33
  - `app-behavior`: 26

## Mapping table (old env key -> target)

`target` is either an existing typed field, a canonical replacement key, or `keep env` when it should remain env-based.

| Env key | Class | Target / replacement |
| --- | --- | --- |
| AI_ROUTER_ALLOWED_HOSTS | runtime-env | `AppConfig.ai.allowed_hosts` (keep env) |
| AI_ROUTER_API_KEY | secret | `AppConfig.ai.api_key` (keep env secret) |
| AI_ROUTER_BASE_URL | runtime-env | `AppConfig.ai.base_url` (keep env) |
| AI_ROUTER_PROVIDER_NAME | app-behavior | `AppBehaviorConfig.ai.provider_name` |
| AI_RUNTIME_ENABLED | runtime-env | `AppConfig.ai.enabled` (keep env) |
| AI_SCHEDULED_OUTCOME_NOTES_AREA_ID | app-behavior | `AppBehaviorConfig.ai.scheduled_outcome_notes_area_id` |
| AI_SCHEDULED_QA_SOURCE_ID | app-behavior | `AppBehaviorConfig.ai.scheduled_qa_source_id` |
| API_BASE_URL | runtime-env | `API_BASE_URL` (test key, keep env) |
| API_VERSION | app-behavior | `API_VERSION` (test key, keep env for integration tests) |
| ARGON2_ITERATIONS | app-behavior | `AppBehaviorConfig.auth.argon2_iterations` |
| ARGON2_MEMORY_KB | app-behavior | `AppBehaviorConfig.auth.argon2_memory_kb` |
| ARGON2_PARALLELISM | app-behavior | `AppBehaviorConfig.auth.argon2_parallelism` |
| AUTH_ACCESS_TOKEN_LIFETIME_MINUTES | app-behavior | `behavior.auth.jwt.access_token_lifetime_minutes` (next typed target; currently env in `underlay-auth-jwt`) |
| AUTH_GOOGLE_CLIENT_ID | runtime-env | `GoogleOAuthService` client id (keep env) |
| AUTH_GOOGLE_CLIENT_SECRET | secret | `GoogleOAuthService` client secret (keep env secret) |
| AUTH_GOOGLE_REDIRECT_URI | runtime-env | `GoogleOAuthService` redirect URI (keep env) |
| AUTH_JWT_AUDIENCE | app-behavior | `behavior.auth.jwt.audience` (next typed target; currently env in `underlay-auth-jwt`) |
| AUTH_JWT_ISSUER | app-behavior | `behavior.auth.jwt.issuer` (next typed target; currently env in `underlay-auth-jwt`) |
| AUTH_JWT_LEEWAY_SECONDS | app-behavior | `behavior.auth.jwt.leeway_seconds` (next typed target; currently env in `underlay-auth-jwt`) |
| AUTH_JWT_PRIVATE_KEY | secret | `JwtConfig.private_key_b64` (keep env secret) |
| AUTH_JWT_PUBLIC_KEY | secret | `JwtConfig.public_key_b64` (keep env key material) |
| AUTH_OAUTH_SECRET_KEY | secret | `OAuthTokenCipher` key (keep env secret) |
| AUTH_REFRESH_TOKEN_LIFETIME_DAYS | app-behavior | `behavior.auth.jwt.refresh_token_lifetime_days` (next typed target; currently env in `underlay-auth-jwt`) |
| BLOB_STORAGE_DIR | runtime-env | local filesystem path for blob adapter (keep env) |
| CARGO_MANIFEST_DIR | runtime-env | tooling/system env (no app migration) |
| COOKIE_DOMAIN | runtime-env | `AppConfig.cors.cookie_domain` (keep env) |
| COOKIE_SECURE | runtime-env | `AppConfig.cors.cookie_secure` (keep env) |
| CORS_ORIGINS | runtime-env | `AppConfig.cors.allowed_origins` (keep env) |
| DATABASE_URL | secret | `AppConfig.database.url` (keep env secret) |
| EMAIL_ADAPTER | runtime-env | `AppConfig.email.adapter` (keep env) |
| EMAIL_APP_NAME | app-behavior | `AppBehaviorConfig.email.app_name` |
| EMAIL_APP_URL | app-behavior | `AppBehaviorConfig.email.app_url` |
| EMAIL_DEFAULT_FROM | app-behavior | `AppBehaviorConfig.email.default_from` |
| EMAIL_FALLBACK_ADAPTER | runtime-env | `AppConfig.email.dev_capture.fallback_adapter` (keep env) |
| EMAIL_SUPPORT | app-behavior | `AppBehaviorConfig.email.support_email` |
| EMAIL_TEMPLATES_DIR | app-behavior | `AppBehaviorConfig.email.templates_dir` |
| EMAIL_WHITELIST | runtime-env | `AppConfig.email.dev_capture.whitelist` (keep env) |
| ENVIRONMENT | runtime-env | `AppConfig.env` (keep env) |
| FARMYARD_API_VERSION | runtime-env | deprecated test alias -> `API_VERSION` |
| FARMYARD_BASE_URL | runtime-env | deprecated test alias -> `API_BASE_URL` |
| HOST | runtime-env | `AppConfig.http.bind_addr` (keep env) |
| LOG_LEVEL | runtime-env | `AppConfig.logging.level` (keep env) |
| PDF_CHROMIUM_BINARY | runtime-env | `PdfRendererConfig.chromium_binary_path` (keep env) |
| PDF_CHROMIUM_TIMEOUT_SECS | app-behavior | `PdfRendererConfig.chromium_timeout_secs` (candidate typed config file target) |
| PDF_RENDERER_PROVIDER | runtime-env | `PdfRendererConfig.provider` (keep env for infra switching) |
| PDF_THIRD_PARTY_API_KEY | secret | `PdfRendererConfig.third_party_api_key` (keep env secret) |
| PDF_THIRD_PARTY_ENDPOINT | runtime-env | `PdfRendererConfig.third_party_endpoint` (keep env) |
| PDF_THIRD_PARTY_TIMEOUT_SECS | app-behavior | `PdfRendererConfig.third_party_timeout_secs` (candidate typed config file target) |
| PORT | runtime-env | `AppConfig.http.port` (keep env) |
| PUBLIC_API_URL | runtime-env | canonical public API base URL |
| PUBLIC_API_VERSION | app-behavior | canonical public API version |
| PUBLIC_FARMYARD_API_VERSION | runtime-env | deprecated -> `PUBLIC_API_VERSION` |
| PUBLIC_FARMYARD_BASE_URL | runtime-env | deprecated -> `PUBLIC_API_URL` |
| PUBLIC_HOST | runtime-env | `AppConfig.http.public_host` (keep env) |
| RUST_LOG | runtime-env | tracing filter (keep env) |
| SES_CONFIGURATION_SET | runtime-env | `AppConfig.email.ses.configuration_set` (keep env) |
| SES_REGION | runtime-env | `AppConfig.email.ses.region` (keep env) |
| SMTP_HOST | runtime-env | `AppConfig.email.smtp.host` (keep env) |
| SMTP_PASSWORD | secret | `AppConfig.email.smtp.password` (keep env secret) |
| SMTP_PORT | runtime-env | `AppConfig.email.smtp.port` (keep env) |
| SMTP_TLS | runtime-env | `AppConfig.email.smtp.tls_mode` (keep env) |
| SMTP_USERNAME | secret | `AppConfig.email.smtp.username` (keep env credential) |
| VIMEO_ACCESS_TOKEN | secret | `AppConfig.vimeo.access_token` (keep env secret) |
| VIMEO_USER_ID | runtime-env | `AppConfig.vimeo.user_id` (keep env) |
| WEBAUTHN_RP_ID | app-behavior | `AppBehaviorConfig.auth.webauthn_rp_id` |
| WEBAUTHN_RP_NAME | app-behavior | `AppBehaviorConfig.auth.webauthn_rp_name` |
| WEBAUTHN_RP_ORIGIN | app-behavior | `AppBehaviorConfig.auth.webauthn_rp_origin` |

## Notes that unblock 20.2.2

1. Farmyard already has typed behavior structs for 14 app-behavior keys and file-based defaults (`config/default.toml`, `config/local.toml`).
2. Remaining high-value app-behavior env keys still live in auth/JWT loader env reads:
   - `AUTH_ACCESS_TOKEN_LIFETIME_MINUTES`
   - `AUTH_REFRESH_TOKEN_LIFETIME_DAYS`
   - `AUTH_JWT_ISSUER`
   - `AUTH_JWT_AUDIENCE`
   - `AUTH_JWT_LEEWAY_SECONDS`
   - Progress: Underlay now exposes `JwtBehaviorDefaults` + `JwtConfig::from_env_with_defaults` so apps can source these from typed config while keeping JWT keys in env.
3. Frontend env cleanup should remove legacy aliases once apps and tests use canonical names only:
   - `PUBLIC_FARMYARD_BASE_URL` -> `PUBLIC_API_URL`
   - `PUBLIC_FARMYARD_API_VERSION` -> `PUBLIC_API_VERSION`
   - `FARMYARD_BASE_URL` -> `API_BASE_URL`
   - `FARMYARD_API_VERSION` -> `API_VERSION`
4. Shared docs/example now include typed auth bootstrap for JWT/WebAuthn/Argon2/OAuth scopes with env retained for secrets/runtime keys:
   - `docs/guides/120-configuration.md`
   - `docs/guides/code/060-authentication/auth-service-example.rs`
5. Farmyard auth bootstrap now consumes typed JWT behavior defaults directly:
   - `../../acowtancy/farmyard/crates/auth/src/local/mod.rs` now uses `JwtConfig::from_env_with_defaults(...)`
   - `../../acowtancy/farmyard/crates/infra/src/config.rs` + `../../acowtancy/farmyard/config/default.toml` now include `[auth].jwt_*` typed defaults
   - Legacy JWT behavior env keys are still supported as compatibility overrides and now emit deprecation warnings
6. Farmyard auth now uses the same typed-config-first compatibility/deprecation pattern for:
   - WebAuthn behavior env overrides: `WEBAUTHN_RP_ID`, `WEBAUTHN_RP_ORIGIN`, `WEBAUTHN_RP_NAME`
   - Argon2 behavior env overrides: `ARGON2_MEMORY_KB`, `ARGON2_ITERATIONS`, `ARGON2_PARALLELISM`
7. Farmyard `.env.example` no longer lists migrated auth behavior keys; README now points auth behavior config to typed `[auth]` TOML defaults while preserving env-only secret guidance.
8. Farmyard auth startup now logs behavior-source diagnostics per domain (`jwt`, `webauthn`, `argon2`) as `typed_config` vs `legacy_env_override`, with focused helper tests validating override/parse fallback behavior.

## Legacy Key Removal Timeline

Timeline anchor date: **2026-02-24**.

1. **Completed on 2026-02-24**:
   - Typed auth behavior defaults live in `config/default.toml`.
   - Legacy env behavior keys still read for compatibility, with deprecation warnings.
   - Migrated auth behavior keys removed from `farmyard/.env.example`.
2. **Target by 2026-03-10**:
   - Local/dev operators stop setting deprecated auth behavior env keys.
   - Any remaining deployment env manifests updated to typed TOML behavior defaults.
3. **Target by 2026-03-24**:
   - Remove compatibility reads for these legacy auth behavior keys in Farmyard auth bootstrap:
     - `AUTH_ACCESS_TOKEN_LIFETIME_MINUTES`
     - `AUTH_REFRESH_TOKEN_LIFETIME_DAYS`
     - `AUTH_JWT_ISSUER`
     - `AUTH_JWT_AUDIENCE`
     - `AUTH_JWT_LEEWAY_SECONDS`
     - `WEBAUTHN_RP_ID`
     - `WEBAUTHN_RP_ORIGIN`
     - `WEBAUTHN_RP_NAME`
     - `ARGON2_MEMORY_KB`
     - `ARGON2_ITERATIONS`
     - `ARGON2_PARALLELISM`
4. **Target by 2026-04-07**:
   - Verify no migrated auth behavior keys remain in Acowtancy `.env` docs/manifests.
   - Mark corresponding roadmap acceptance items complete for this migration slice.

## Evidence pointers

- Farmyard app config loading and behavior overlays:
  - `../../acowtancy/farmyard/crates/infra/src/config.rs`
- Farmyard auth behavior overrides and argon/webauthn wiring:
  - `../../acowtancy/farmyard/crates/auth/src/local/mod.rs`
- Farmyard jobs config entrypoint:
  - `../../acowtancy/farmyard/crates/jobs/src/main.rs`
- PDF renderer env loader:
  - `../../acowtancy/farmyard/crates/pdf-renderer/src/lib.rs`
- Cream public API canonicalization + legacy warnings:
  - `../../acowtancy/cream/src/lib/config/public-api.ts`
- Dairy public env usage:
  - `../../acowtancy/dairy/src/hooks.server.ts`
- Cattle-grid legacy test aliases:
  - `../../acowtancy/cattle-grid/tests/integration-learning.test.ts`
- Underlay env-backed auth loaders used by Farmyard:
  - `../../underlay/rust/crates/underlay-auth-jwt/src/config.rs`
  - `../../underlay/rust/crates/underlay-auth-oauth/src/google.rs`
  - `../../underlay/rust/crates/underlay-auth-oauth/src/token_cipher.rs`
- Farmyard typed JWT behavior integration (pilot implementation):
  - `../../acowtancy/farmyard/crates/infra/src/config.rs`
  - `../../acowtancy/farmyard/crates/auth/src/local/mod.rs`
  - `../../acowtancy/farmyard/config/default.toml`
  - `../../acowtancy/farmyard/.env.example`
