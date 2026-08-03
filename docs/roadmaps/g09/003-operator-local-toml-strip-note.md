# g09.003 - Operator local.toml Strip Note

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

`config/local.toml` layers *last* (default → `effigy.toml` → `local.toml`),
so a stale personal file silently overrides the committed shared dev-stack
config. The dev machine used for the convergence was stripped by hand, but
every other operator machine still carries a fat pre-convergence
`local.toml`. That is a per-machine drift vector with no in-repo signal.

## Evidence

- `config/local.toml` (gitignored) in acme, cp, compli
- Audit item 3

## Planned Changes

- [x] Add a short "After pulling the config convergence" note to each
  affected consumer's README (or onboarding doc): strip `config/local.toml`
  to personal overrides only; shared dev config now lives in the committed
  `config/effigy.toml`.
- [x] Repos: underlay-reference, contact-patch, compli-me (songsprout and
  composer have no tracked `local.toml` history; add the note only if their
  READMEs mention `local.toml`).

## Consumer Upgrade Impact

Impact class: `documentation`. Consumer READMEs updated in place.

## Validation

- [x] Notes present and accurate in each affected repo
- [x] `effigy qa:docs` in underlay

## Stop Conditions

None expected.

## Completion Notes

Completed 2026-08-03. Strip notes added to the Config And Secrets Policy section of acme/cp/compli READMEs (layers-last warning + pointer to committed effigy.toml). Also fixed acme README's stale Environment Variables section (old acme-api/config paths + removed .env contract claim).

## Next Task

`g09.004` retire `with_environment_from_env`.
