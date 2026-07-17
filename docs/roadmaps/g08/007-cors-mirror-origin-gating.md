# g08.007 - CORS Mirror-Origin Gating

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Prevent a wildcard-with-credentials CORS posture in production.
`AllowOrigin::mirror_request()` echoes any Origin while `allow_credentials(true)`
is set, so any site can make credentialed reads for a logged-in user. It is
documented "for local dev" but nothing gates it to non-prod. Separately, the
default config is allow-any-origin with `AllowMethods::any()`, which invites
permissive prod configs even though credentials are off by default.

## Evidence

- `rust/crates/underlay-http/src/cors.rs:199-201,233-234` (mirror + credentials)
- `rust/crates/underlay-http/src/cors.rs:54-68,224` (allow-any default)

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [031 Config and secrets](../../contracts/031-config-and-secrets.md)

## Planned Changes

- [x] Gate `with_mirror_origin` behind `Environment::Local`/`Test`, or error when
  mirror + credentials are combined outside those environments.
- [x] Change `CorsConfig::default()` to an empty explicit origin list; make
  wildcard an explicit opt-in.
- [x] Document the safe prod CORS configuration in `031`.

## Consumer Upgrade Impact

Impact class: `configuration`. Consumers relying on the permissive default must
declare explicit origins. Requires six-consumer proof per `023`.

## Validation

- [x] test: mirror + credentials rejected/panics outside Local/Test; default
  config allows no cross origin
- [x] `cargo test -p underlay-http`
- [x] `effigy validate`

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-07-17. `CorsConfig::default()` now allows no cross-origin
access (wildcard is explicit opt-in via `with_any_origin`). New
`try_cors_layer_for_env`/`cors_layer_for_env` refuse mirror+credentials
outside `Environment::Local`/`Test`; env-less `cors_layer` panics on that
combination with a pointer to the env-gated builder. Contract `031` records
the safe prod posture; guides `066`/`070` updated off the stale (and
dangerous) mirror-fallback pattern. Validated: `cargo test -p underlay-http`
green.

## Next Task

`g08.008` distributed rate-limit backend (Batch 3).
