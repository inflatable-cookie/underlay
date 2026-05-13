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
- `.env.example` and local config file expectations
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
  host
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
3. optional local config override
4. allowlisted env overrides

Rules:

- fail fast on invalid config
- log effective config in redacted form where useful
- reject or warn on unknown app env keys
- env override should be explicit and allowlisted, not open-ended

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

- `.env.example` documents any temporary env bridge keys with placeholders only
- real secrets stay out of committed files
- Effigy-managed apps declare true local secrets under root `[secrets.keys]`
- local secret values are stored in an ignored Effigy vault, not committed files
- optional `config/local.toml` or equivalent stays gitignored

Rules:

- `.env.example` may show placeholders, never live secrets
- `config/local.toml.example` may document local non-secret overrides
- do not commit real JWT keys, OAuth secrets, SMTP passwords, or database
  credentials
- for Effigy-managed apps, `.env` is a compatibility bridge, not the target
  secret authority
- non-secret local values such as ports, hostnames, service names, database
  names, object-store bucket names, regions, and public URL bases should come
  from typed config, bundle defaults, local config, or generated runtime config
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
required = false
targets = ["tasks", "containers"]
description = "Application database connection URL."
```

Use `required = false` while a consumer app still needs legacy `.env`
compatibility. Switch to `required = true` only after bootstrap docs and local
tooling make the vault path normal for all developers.

### Admin and front config rule

Admin and front apps should distinguish public runtime values from app behavior.

Rules:

- browser-visible runtime values use `PUBLIC_*`
- stable app behavior should not be hidden in ad hoc frontend env keys
- frontend apps should document required `PUBLIC_*` keys plainly in
  `.env.example` or equivalent setup docs

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

Do not let non-secret auth behavior drift across `.env` files.

### Compatibility-window rule

Renamed or migrated keys may use a temporary compatibility bridge.

Rules:

- warn first, remove later
- document the replacement field or key explicitly
- remove migrated behavior keys from `.env.example` as soon as typed defaults
  exist
- do not keep behavior fallbacks to legacy env keys indefinitely

## Minimum App Shape

A normal app should have:

- `.env.example`
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
- `.env.example` is short and honest
- legacy key bridges are explicit and temporary

Bad outcomes:

- app behavior hidden in dozens of env keys
- services reading env directly
- unknown env keys silently accepted
- committed files containing live secrets
- config precedence only discoverable from code spelunking

## Next Task

Use this contract when bootstrapping new apps, migrating behavior out of env,
or auditing secret posture across the consumer fleet.
