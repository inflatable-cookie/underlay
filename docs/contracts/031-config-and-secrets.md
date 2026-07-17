# Contract: Config and Secrets

Status: active
Owner: repo maintainers
Depends on: `024-new-app-bootstrap-and-bring-up.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `028-runtime-surface-and-openapi-maturity-levels.md`, `030-auth-and-session-systems.md`

## Purpose

Define the shared config and secret posture for normal Underlay apps.

This contract covers:

- config classes and where they belong
- env naming and grouping
- canonical config load order
- typed config versus env ownership
- local dev secret posture
- app-local config package and file expectations

It does not define app-domain settings themselves. It defines where those
settings should live and how they should be loaded.

## Sources of Truth

Primary guidance and rollout evidence:

- [`docs/guides/120-configuration.md`](/Users/tom/Dev/projects/underlay/docs/guides/120-configuration.md)
- [`docs/guides/121-consumer-config-rollout-kit.md`](/Users/tom/Dev/projects/underlay/docs/guides/121-consumer-config-rollout-kit.md)
- [`docs/guides/140-local-development.md`](/Users/tom/Dev/projects/underlay/docs/guides/140-local-development.md)
- [`docs/contracts/024-new-app-bootstrap-and-bring-up.md`](/Users/tom/Dev/projects/underlay/docs/contracts/024-new-app-bootstrap-and-bring-up.md)
- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)

Reference consumer evidence:

- `underlay-reference/acme-api`
- `acowtancy/farmyard`
- `compli-me/api`
- `contact-patch/cp-api`
- `songsprout/nursery`
- `loophole/composer/composer-api`

If these diverge, the contract plus the clearest modern posture win.

Promoted implementation reference:

- `acowtancy/farmyard`

For runtime assembly and local bring-up, Acowtancy is the promoted
implementation. Other consumers should converge on its config posture unless a
documented exception is promoted here first.

## Contract Goal

Underlay should make app config boring.

A normal app team should not have to rediscover:

- which values belong in env
- which values belong in typed config files
- how config should override across local and deployed environments
- where secrets should live in local dev
- where direct env reads are allowed

The goal is one declared config model instead of six local habits.

## Scope Boundary

In scope:

- API config ownership
- admin/front runtime-public config posture
- env naming and grouping
- env manifest and local config file expectations
- secret versus non-secret classification
- config bootstrap boundary

Out of scope:

- deployment secret manager implementation
- app-domain config schema details
- release rollout policy
- migration-specific SQL/config interplay

## Shared Boundary

### Config class rule

Every setting must be classified before it is added.

Classes:

- `secret`
- `runtime-env`
- `app-behavior`

Rules:

- `secret` values live in environment or secret manager only
- `runtime-env` values live in environment because they truly vary by deploy or
  host, but for local Effigy-managed apps they should be injected by Effigy
  rather than stored in `.env` files
- `app-behavior` values live in typed config with committed defaults
- if a value is not secret and does not need per-environment drift, it should
  not live in `.env`

### Typed-config rule

Normal Underlay APIs should own a typed config layer for stable behavior.

Expected posture:

- a typed config crate or module
- committed defaults
- explicit config sections such as:
  - server
  - auth
  - email
  - storage
  - app-domain behavior

Rules:

- behavior knobs, retries, limits, and default timeouts belong in typed config
- auth behavior belongs in typed config unless the value is secret or truly
  deployment-specific
- do not scatter direct `std::env::var` reads through handlers and services

### Config load-order rule

Canonical precedence is lowest to highest:

1. typed defaults
2. committed default config file
3. named environment config file
4. optional local config override
5. allowlisted env overrides

Rules:

- named environment overlays should use deploy-aligned names such as `dev`,
  `uat`, and `production`
- `local.toml` is not an environment; it is a gitignored personal non-secret
  patch
- runtime selection should use `ENVIRONMENT_NAME`, defaulting to `dev` when
  unset; deployed services should set it to the matching environment name
- fail fast on invalid config
- log effective config in redacted form where useful
- reject or warn on unknown app env keys
- env override should be explicit and allowlisted, not open-ended

### Runtime and browser-surface rule

Normal Underlay API apps must keep local HTTP runtime and browser auth surface
settings in typed config, not ad hoc `.env` files.

Required typed sections:

- `[<app>.runtime]`
- `[<app>.cors]`
- `[public_api]`
- `[<app>.auth]`
- `[<app>.email]`
- `[<app>.database]`

Rules:

- `[<app>.runtime]` owns local bind host, bind port, and public API hostname
- `[<app>.cors]` owns allowed origins, cookie domain, and cookie secure posture
- safe production CORS posture: an explicit `allowed_origins` list built with
  `CorsConfig::with_origins`/`try_with_origins`, credentials only alongside
  that explicit list. `CorsConfig::default()` allows no cross-origin access;
  wildcard (`with_any_origin`) is an explicit opt-in for credential-free
  internal services only. Mirror-origin with credentials is a local-dev-only
  posture: layers are built via `cors_layer_for_env`, which refuses that
  combination outside `Environment::Local`/`Test`
- `[public_api]` owns browser-visible API/front/admin base URLs
- local WebAuthn RP values belong in typed auth config, not runtime env
- local email app URL belongs in typed email config, not runtime env
- the API bootstrap must read cookie/domain/port/host values from the loaded
  typed config, with env only as an explicit allowlisted override path
- Effigy bundles must not rely on generating repo `.env` files for normal
  Underlay app bring-up

Promoted local shape:

```toml
[public_api]
base_url = "https://api.example.test"
front_url = "https://example.test"
admin_url = "https://admin.example.test"

[app.runtime]
host = "127.0.0.1"
port = 3000
public_host = "localhost"

[app.cors]
allowed_origins = []
cookie_secure = false

[app.database]
# url is optional in committed defaults

[app.email]
app_url = "https://example.test"

[app.auth]
webauthn_rp_id = "localhost"
webauthn_rp_origin = "http://localhost:4174"
webauthn_rp_name = "Example"
```

Promoted local override shape:

```toml
[app.runtime]
host = "0.0.0.0"
port = 41001
public_host = "api.example.test"

[app.cors]
allowed_origins = ["https://example.test", "https://admin.example.test"]
cookie_domain = ".example.test"
cookie_secure = true

[app.email]
app_url = "https://admin.example.test"

[app.auth]
webauthn_rp_id = "admin.example.test"
webauthn_rp_origin = "https://admin.example.test"
webauthn_rp_name = "Example"
```

### Bootstrap boundary rule

Environment should be read at config/bootstrap time, not throughout the app.

Rules:

- direct env reads belong in config bootstrap only
- downstream services should receive typed config, not read env themselves
- local compatibility bridges for renamed keys may live in bootstrap during a
  deprecation window, not in handlers

### Env naming rule

Use the shared naming posture for keys that stay in env.

Rules:

- generic infrastructure keys stay unprefixed where the shared meaning is clear
  enough:
  - `DATABASE_URL`
  - `HOST`
  - `PORT`
- auth keys use `AUTH_`
- frontend public keys use `PUBLIC_`
- app branding or domain-specific public messaging may use an app prefix where
  that improves clarity

Do not invent app-local prefixes for generic shared infrastructure keys unless a
clear conflict requires it.

### Local-dev secret posture

Local development should keep secret handling simple but explicit.

Expected posture:

- real secrets stay out of committed files
- Effigy-managed apps declare true local secrets under root `[secrets.keys]`
- local secret values are stored in an ignored Effigy vault, not committed files
- optional `config/local.toml` or equivalent stays gitignored
- `config/env-manifest.txt` documents the allowed env surface for runtime and
  validation tooling

Rules:

- do not keep `.env`, `.env.local`, or `.env.example` files in the target
  posture for Effigy-managed Underlay apps
- `config/local.toml.example` may document local non-secret overrides
- do not commit real JWT keys, OAuth secrets, SMTP passwords, or database
  credentials
- non-secret local values such as ports, hostnames, service names, database
  names, object-store bucket names, regions, and public URL bases should come
  from typed config, bundle defaults, `config/dev.toml`, optional
  `config/local.toml`, or generated runtime config
- true secrets should be declared with explicit Effigy targets such as
  `tasks`, `containers`, `state`, `artifacts`, and `deploy`
- required sibling mounts and local setup expectations belong in bootstrap docs,
  not hidden inside env folklore

Effigy local vault shape:

```toml
[secrets]
backend = "effigy-vault"

[secrets.vault]
path = ".effigy/secrets/local.vault"
identity = "passphrase"
unlock = "passphrase"

[secrets.keys.database_url]
required = true
targets = ["tasks", "containers"]
description = "Application database connection URL."
```

Use `required = true` once the app posture is fully on Effigy vault plus typed
config. If a consumer repo is still migrating, keep the temporary bridge local
to that repo and remove it before calling the migration complete.

### Admin and front config rule

Admin and front apps should distinguish public runtime values from app behavior.

Rules:

- browser-visible runtime values use `PUBLIC_*`
- stable app behavior should not be hidden in ad hoc frontend env keys
- frontend apps should document required `PUBLIC_*` keys plainly in
  `config/env-manifest.txt`, setup docs, or generated runtime-config docs

### Auth-config split rule

Auth settings must respect the secret versus behavior split.

Keep in env:

- signing keys
- client secrets
- encrypted-token keys
- other true secrets

Move to typed config:

- token lifetimes
- issuer and audience defaults
- WebAuthn RP behavior
- Argon2 tuning
- provider scopes and other non-secret auth behavior

Do not let non-secret auth behavior drift across ad hoc env setup.

### Compatibility-window rule

Renamed or migrated keys may use a temporary compatibility bridge.

Rules:

- warn first, remove later
- document the replacement field or key explicitly
- remove migrated behavior keys from `config/env-manifest.txt` as soon as typed
  defaults exist
- do not keep behavior fallbacks to legacy env keys indefinitely

## Minimum App Shape

A normal app should have:

- `config/env-manifest.txt`
- committed default config file for API behavior where applicable
- optional local config example for non-secret local overrides
- one typed config bootstrap layer
- one allowlisted env surface

Reference posture:

- `config/default.toml`
- `config/local.toml.example`
- app-local config crate or module

## What Good Looks Like

Good outcomes:

- secrets and runtime-only values stay in env
- stable behavior defaults live in typed config
- startup fails fast on invalid config
- env usage is concentrated in bootstrap
- the env manifest is short and honest
- legacy key bridges are explicit and temporary

Bad outcomes:

- app behavior hidden in dozens of env keys
- services reading env directly
- unknown env keys silently accepted
- committed files containing live secrets
- config precedence only discoverable from code spelunking

## Next Task

Use this contract when bootstrapping new apps, migrating behavior out of env,
or auditing secret posture across the consumer fleet. The current rollout after
`acowtancy` is `underlay-reference`, `contact-patch`, `compli-me`,
`songsprout`, and `loophole/composer`, with each root treated as a workspace
boundary that includes all affected child packages.
