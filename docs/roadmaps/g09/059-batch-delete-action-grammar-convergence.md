# g09.059 - Batch Delete Action Grammar Convergence

Status: planned - decision gated
Owner: repo maintainers
Contract: `029`
Depends on: `g09.057`

## Purpose

Give Underlay Reference and Compli Me one canonical batch-delete grammar per
API without changing batch-delete behavior.

## Decision Gate

Before this roadmap becomes ready, settle the canonical suffix and
compatibility window for each target:

- Underlay Reference currently uses `:batch-delete` for categories, projects,
  and media, but `/batch-delete` for nested project tasks. The implementation
  comment already names the colon form as canonical.
- Compli Me currently uses `/batch-delete` for businesses, people, and
  compliments, but `:batch-delete` for media. Choose one app-wide grammar;
  contract `029` prefers the established explicit colon suffix.

For each target, decide whether in-repo caller proof permits an atomic cutover
or whether a temporary same-handler alias is required. Do not infer an external
compatibility window from source search.

## Planned Lanes

### Underlay Reference

- make nested task batch-delete match the chosen app grammar
- move Acme Client and active route tests first or atomically only with proved
  local callers
- update OpenAPI/comments/inventory and add old-path absence proof

### Compli Me

- move business, people, and compliment batch-delete paths or media according
  to the chosen app-wide grammar
- move API Client and active route inventory first
- update OpenAPI/docs and add old-path absence proof

The two target lanes may run independently after their own decision closes.

## Acceptance

- each API exposes one batch-delete suffix grammar
- collection semantics and `POST` payloads remain unchanged
- clients move before aliases retire unless the caller set is proved local
- no lifecycle, envelope, access, or audit redesign enters the batch
- target-owned Effigy validation and focused route/client tests pass
- one fleet closeout records exact merged tips

## Stop Conditions

Stop if the chosen suffix would change action semantics or if an external
caller window remains unowned.

## Consumer Upgrade Impact

- Impact class: compatibility retirement
- Affected consumers: Underlay Reference, Compli Me
- Required action: decision dependent
- Compatibility window: unresolved per target

## Next Task

Settle the two canonical suffix/window decisions, then promote only the cleared
target lanes.
