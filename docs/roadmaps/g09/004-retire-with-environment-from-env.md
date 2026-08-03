# g09.004 - Retire with_environment_from_env

Status: ready
Owner: repo maintainers

## Purpose

`ConfigStack::with_environment_from_env` has zero callers after the
convergence, and its baked-in defaults (`ENVIRONMENT_NAME` var, `dev`
fallback) actively contradict the new model (`ENVIRONMENT` primary via
`resolve_name`). Dead machinery that still looks like the blessed path.

## Evidence

- `rust/crates/underlay-config/src/stack.rs:43-52`
- `rust/crates/underlay-config/src/constants.rs` (`DEFAULT_ENV_VAR`,
  `DEFAULT_ENVIRONMENT`)
- Audit item 4

## Planned Changes

- [ ] Mark `with_environment_from_env` `#[deprecated]` with a note pointing
  at `Environment::resolve_name`, or remove it outright if no external
  users exist (verify against all consumers).
- [ ] If removed, also retire `DEFAULT_ENV_VAR`/`DEFAULT_ENVIRONMENT` if
  they have no other callers.
- [ ] Update any doc references to the old pattern.

## Consumer Upgrade Impact

Impact class: `deprecation`. No consumer uses the method post-convergence
(verified by grep across all six); removal is safe but should still ship as
a deprecation first if any doubt remains.

## Validation

- [ ] `cargo test -p underlay-config`
- [ ] `effigy validate`
- [ ] grep confirms zero callers in underlay + all six consumers

## Stop Conditions

None expected.

## Next Task

`g09.005` `admin_cors_layer_from_env`.
