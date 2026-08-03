# Config convergence — `effigy` dev environment, canonical env/CORS, shared dev credentials

Date: 2026-08-03
Scope: underlay, underlay-effigy-bundle, effigy, all six consumers
Governing refs: `docs/architecture/070-consumer-drift-prevention.md` (B3),
`docs/sweeps/021-consumer-security-convergence.md` §1

Trigger: dev CORS broke fleet-wide when the July sweep made an unset
`ENVIRONMENT` fail closed to prod — the dev stack never set it anywhere
(the old `.env` path was removed in May's root-stack config move). Root
cause was structural: no canonical place for non-secret dev env, five cloned
CORS/env builders, three env-var naming schemes, dev config hand-copied
between operators' untracked `local.toml` files.

## The model

- **One dev environment, named `effigy`** — unambiguous: "running in the
  effigy container dev stack". `local` stays a legacy alias.
- **One injection point** — the bundle's `env/dev.env.schema`
  (`ENVIRONMENT=effigy`, plain/non-secret). Effigy folds schema plain env
  into every rendered task command, including managed dev-session apps.
  Secrets stay vault-only.
- **One env/CORS implementation** — `underlay_observability::Environment`
  (`Effigy` variant, `Environment::resolve`, `is_local_dev`) and
  `underlay_http::admin_cors_config/admin_cors_layer` replace the per-app
  clones. `ENVIRONMENT` is primary everywhere; app-specific vars
  (`ACME_ENV`, `CP_ENV`, `COMPLI_ENV`, `COMPOSER_ENV`, `ENVIRONMENT_NAME`)
  are deprecated fallbacks.
- **One config overlay shape** — committed `config/effigy.toml` holds the
  shared dev-stack constants (dev DB URL, minio, `.test` hosts, webauthn);
  loaders name the overlay from the same `Environment::resolve` call that
  gates behavior. `local.toml` stays as the personal gitignored overlay.
- **One dev credential set** — `admin@example.com` / `UnderlayDev2026!` /
  TOTP secret `UNDERLAYDEVTOTPSECRET2345678ABCD` in all dev seeds (one
  authenticator entry for the fleet).

## What shipped

- **underlay** `c8c6a6c4`, `6c88b90c`, `b07797b5` — `Environment::Effigy`
  (+ parse/display/gates), `resolve`, `is_local_dev` gating CORS
  mirror-origin + loopback defaults, `admin_cors_config/layer` (incl.
  `x-auth-token-mode`), `cors-canonical` conformance check (17/17 in all
  six repos).
- **effigy** `07d49a80` — child catalogs inherit the nearest ancestor
  catalog's `[env_schema]` (without it the bundle's env never reached app
  processes — every app is a child-catalog task). Earlier same-day:
  `4874b17e` (install-before-probe, volume chown wiring, containers
  ancestor fallback).
- **bundle** `7685e8d`, `fbec0cd` — `env/dev.env.schema` +
  `[env_schema]` wiring; `[catalog.discovery] ignore = ["config"]` after
  `config/effigy.toml` collided with catalog manifest discovery (every
  task failed parse).
- **consumers** — env/CORS convergence + committed `config/effigy.toml` +
  shared-credential seeds: acme (`f7f25df`, `50fcbd4`, `47f7e94`), cp
  (`f8feef2`, `8606e1d`, `df5ba1f`, `ac8933c`), compli (`cb1b1e2`,
  `b59af08`, `10ede0e`, `e50c431`), songsprout (`439d2c0`, `5b0a273`,
  `7bf9c3c`), composer (`ec0e187`, `68476a6`), farmyard (`b30049a8` +
  parent `d092ce5`).
- **seed upsert fix** — seeds upsert by `id` (not `email`) so existing dev
  DBs migrate the old admin row instead of violating the pkey.

## Verification

- Fold proven at render level: root + child-catalog tasks render
  `env 'ENVIRONMENT=effigy' sh -c '…'`.
- cp-api boot: `env: Effigy`, `cors_origins: 2` (overlay loaded), seeds
  clean on the existing dev DB.
- The original failure, fixed: `GET /v1/book/navigation` with
  `Origin: https://contact-patch.test` → 200 with
  `access-control-allow-origin: https://contact-patch.test` and
  credentials; preflight returns the canonical header set.
- Conformance 17/17 in all six consumers.

## Variants and follow-ups

- **farmyard** keeps its TOML-overlay env model and `"dev"` as its local
  identifier (gates accept `Effigy` too); aligning it to the fleet
  identifier means renaming its env value + `dev.toml` and is parked.
- **farmyard dev credentials** come from the legacy-dump seed-bundle
  harness — shared-credential alignment needs a designed hook, not a
  drive-by. Accepted variant for now.
- **songsprout** has no overlay seam (dev constants live in committed
  `config/default.toml`; its two hand-rolled loaders hardcode a vestigial
  `dev.toml` layer). Works today; a real env-named overlay + moving dev
  constants out of `default.toml` is a designed follow-up. Its
  `ENVIRONMENT_NAME`-primary precedence is inverted vs the fleet but
  harmless (never set in-stack).
- **Effigy shell tab / lifecycle processes** don't get schema env (not
  task references) — manual `cargo run` there needs `ENVIRONMENT=effigy`
  prefixed.
- **Behavior deltas to know**: prod with no explicit CORS origins now
  denies cross-origin silently (empty allowlist) instead of boot-panicking;
  invalid `CORS_ORIGINS` entries are now a boot-time panic; mirror-origin
  requires `is_local_dev()` (`ENVIRONMENT=dev` no longer mirrors);
  composer gained `x-api-version`/`x-csrf-token` and lost `if-none-match`
  from its custom header list.
- **Env schema applies to all task runs** including builds — nothing may
  read `ENVIRONMENT` at build time (front/admin public config comes from
  the config stack).
