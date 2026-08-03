# 192 - Config Model (effigy dev environment, overlays, env/CORS)

The canonical config model for Underlay and its consumers, as converged in
the 2026-08 config convergence (`docs/logs/2026-08/03-104132-config-convergence.md`).
If a consumer deviates from this page, fix the consumer.

Related: `docs/architecture/070-consumer-drift-prevention.md`,
`docs/sweeps/021-consumer-security-convergence.md` §1,
`docs/guides/191-admin-resource-checklist.md`.

## The dev environment is called `effigy`

One identifier fleet-wide: running inside the effigy container dev stack.
Not `local` (ambiguous — host vs container), not per-app names.

- The underlay-effigy-bundle injects `ENVIRONMENT=effigy` into every
  task-rendered process via its env schema (`env/dev.env.schema`), including
  managed dev-session apps and (since effigy `6d958589`) the shell tab.
- `local` stays a legacy alias; app-specific vars (`ACME_ENV`, `CP_ENV`,
  `COMPLI_ENV`, `COMPOSER_ENV`, `ENVIRONMENT_NAME`) are deprecated
  fallbacks and print a one-time warning when they win.
- `Environment` (underlay-observability) has the `Effigy` variant; gates use
  `is_local_dev()` (`Local | Effigy | Test`) for dev-only behavior (seeds,
  CORS mirror, loopback defaults).

## Resolving the environment (apps never parse env vars themselves)

```rust
// Behavior (fail closed: unset/unknown -> Prod):
let env = Environment::resolve("ENVIRONMENT", Some("<APP>_ENV"));

// Config overlay name (raw string; overlay names like uat/production are
// arbitrary and must NOT go through the enum):
let name = Environment::resolve_name("ENVIRONMENT", Some("<APP>_ENV"))
    .unwrap_or_else(|| "dev".to_string());
```

`ENVIRONMENT` is always primary; the app-specific var is the deprecated
fallback. Both helpers read the same vars in the same order, so behavior
and overlay cannot diverge.

## Config file layering

```
config/default.toml        committed, safe defaults for all environments
config/<env>.toml          committed, named overlay (effigy.toml, uat.toml, …)
config/local.toml          gitignored, personal machine-local overrides only
allowlisted env vars       secrets and runtime wiring (env, vault)
```

- `config/effigy.toml` owns the shared dev-stack constants (dev DB URL,
  minio/S3 dev settings, `.test` hostnames, webauthn dev origins). It is
  committed because the container dev environment is identical for every
  operator. Secrets never live here — they are in the effigy vault.
- `config/local.toml` layers **last**, so stale copies silently override
  `effigy.toml`. Keep it to personal tweaks (or empty).
- `ConfigStack` merges `default` → `<env>` (via `resolve_name`) → `local`.
  Do not use the deprecated `with_environment_from_env`.

## CORS (admin APIs)

One construction point in `underlay-http`:

```rust
// Origins from the CORS_ORIGINS env var (comma list), mirror-origin in
// local dev when empty:
let cors = underlay_http::admin_cors_layer_from_env(env);

// Origins from app config instead:
let cors = underlay_http::admin_cors_layer(env, origins_from_config);
```

- Canonical headers: `x-api-version`, `x-csrf-token`, `x-auth-token-mode`,
  `if-match`, `if-none-match` + defaults; credentials on.
- Mirror-origin only in `is_local_dev()`; explicit origins otherwise;
  invalid origins are a boot-time panic.
- Prod/Staging with empty origins logs a boot warning (deny-all is
  fail-closed but usually a misconfiguration).
- App-local `build_cors_layer` / `underlay_env` clones are conformance
  violations (`cors-canonical` check).

## Dev seeds and credentials

- Seeds run only when `env.is_local_dev()` **and** the DB URL is local
  (`underlay_db::is_local_database_url`).
- Shared dev credentials in all `migrations_dev` seeds:
  `admin@example.com` / `UnderlayDev2026!` / TOTP secret
  `UNDERLAYDEVTOTPSECRET234567ABCDE` — one authenticator entry for the
  whole fleet. (farmyard's legacy-dump seed harness is the tracked
  exception — `g09.010`.)
- Seeds upsert by `id` so existing dev DBs migrate the old admin row.

## Bootstrapping a new consumer onto the model

1. Base it on the underlay-effigy-bundle (env schema injects
   `ENVIRONMENT=effigy`; catalog discovery already ignores `config/`).
2. Resolve env via `Environment::resolve` / `resolve_name`; delete any
   local env parsing.
3. CORS via `admin_cors_layer(_from_env)`; delete any local builder.
4. Commit `config/effigy.toml` with the app's dev-stack constants;
   keep `local.toml` personal-only.
5. Seed the shared dev credentials from `migrations_dev/`.
6. `effigy qa:security` (conformance) must pass.
