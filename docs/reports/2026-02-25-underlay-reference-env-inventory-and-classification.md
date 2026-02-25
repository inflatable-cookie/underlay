# Underlay Reference Env Inventory and Classification (20.3.1)

Date: 2026-02-25  
Roadmap: `docs/roadmap/020-configuration-standardization-and-env-reduction.md`

## Scope

Inventory and migration verification for the reference surfaces in `../underlay-reference`:

- `acme-api`
- `acme-admin`
- `acme-front`

Inputs:

1. Declared keys in `.env.example` files.
2. Typed config defaults and bootstrap wiring in `acme-api`.
3. Focused env-key sweep for migrated app-behavior keys in reference setup manifests.

## Inventory summary

- 25 unique env keys in reference `.env.example` files after migration.
- Classification totals:
  - `secret`: 5
  - `runtime-env`: 20
  - `app-behavior`: 0 in env manifests for migrated auth/email behavior set

## Migrated behavior mapping (reference)

These migrated behavior keys are now typed-config sourced in `acme-api/config/default.toml` and no longer listed in reference `.env.example` setup:

| Legacy env key | Typed replacement |
| --- | --- |
| `EMAIL_DEFAULT_FROM` | `AppBehaviorConfig.email.default_from` |
| `EMAIL_APP_NAME` | `AppBehaviorConfig.email.app_name` |
| `EMAIL_APP_URL` | `AppBehaviorConfig.email.app_url` |
| `EMAIL_SUPPORT` | `AppBehaviorConfig.email.support_email` |
| `EMAIL_TEMPLATES_DIR` | `AppBehaviorConfig.email.templates_dir` |
| `AUTH_ACCESS_TOKEN_LIFETIME_MINUTES` | `AppBehaviorConfig.auth.jwt_access_token_lifetime_minutes` |
| `AUTH_REFRESH_TOKEN_LIFETIME_DAYS` | `AppBehaviorConfig.auth.jwt_refresh_token_lifetime_days` |
| `AUTH_JWT_ISSUER` | `AppBehaviorConfig.auth.jwt_issuer` |
| `AUTH_JWT_AUDIENCE` | `AppBehaviorConfig.auth.jwt_audience` |
| `AUTH_JWT_LEEWAY_SECONDS` | `AppBehaviorConfig.auth.jwt_leeway_seconds` |
| `WEBAUTHN_RP_ID` | `AppBehaviorConfig.auth.webauthn_rp_id` |
| `WEBAUTHN_RP_ORIGIN` | `AppBehaviorConfig.auth.webauthn_rp_origin` |
| `WEBAUTHN_RP_NAME` | `AppBehaviorConfig.auth.webauthn_rp_name` |
| `ARGON2_MEMORY_KB` | `AppBehaviorConfig.auth.argon2_memory_kb` |
| `ARGON2_ITERATIONS` | `AppBehaviorConfig.auth.argon2_iterations` |
| `ARGON2_PARALLELISM` | `AppBehaviorConfig.auth.argon2_parallelism` |

## Implementation notes

1. `acme-api` typed auth behavior now includes JWT defaults (`jwt_*`) in `AppBehaviorConfig.auth` and `config/default.toml`.
2. Local auth bootstrap now uses typed JWT behavior defaults via `JwtConfig::from_values(...)` while retaining env-only key material (`AUTH_JWT_PRIVATE_KEY`, `AUTH_JWT_PUBLIC_KEY`).
3. Legacy env behavior overrides were removed from reference bootstrap paths for:
   - Email branding behavior (`EMAIL_*` behavior keys)
   - JWT behavior tuning (`AUTH_*` behavior tuning keys)
   - WebAuthn relying-party behavior (`WEBAUTHN_RP_*`)
   - Argon2 tuning (`ARGON2_*`)
4. Focused infra config test verifies migrated behavior env keys do not override typed config values.
5. Reference bootstrap now emits compatibility warnings when migrated legacy behavior env keys are present, including typed replacement field names.

## Evidence pointers

- Reference env setup manifests:
  - `../../underlay-reference/acme-api/.env.example`
  - `../../underlay-reference/acme-admin/.env.example`
  - `../../underlay-reference/acme-front/.env.example`
- Typed behavior defaults and bootstrap:
  - `../../underlay-reference/acme-api/config/default.toml`
  - `../../underlay-reference/acme-api/crates/infra/src/config.rs`
  - `../../underlay-reference/acme-api/crates/auth/src/local/mod.rs`
- Legacy compatibility warning layer:
  - `../../underlay-reference/acme-api/crates/infra/src/config.rs` (`warn_legacy_behavior_env_keys`, `collect_set_legacy_behavior_env_keys`)
