# g09.059 - Batch Delete Action Grammar Convergence

Status: ready - dispatched
Owner: repo maintainers
Contract: `029`
Depends on: `g09.057`

## Purpose

Give Underlay Reference and Compli Me one canonical batch-delete grammar per
API without changing batch-delete behavior.

## Decision

The operator settled the grammar and compatibility posture on 2026-08-27:

- `:batch-delete` is canonical in both target APIs;
- treat the supported fleet caller set as closed-world;
- provide no external compatibility window;
- move every in-repo caller and retire slash-suffix routes atomically;
- require negative route proof after retirement.

A worker must still stop if current source disproves the assessment or exposes a
caller outside the declared fleet.

## Planned Lanes

### Underlay Reference

- make nested task batch-delete use `:batch-delete`
- move Acme Client and active route tests atomically
- update OpenAPI/comments/inventory and add old-path absence proof

### Compli Me

- move business, people, and compliment batch-delete paths to `:batch-delete`
- move API Client and active route inventory atomically
- update OpenAPI/docs and add old-path absence proof

The two target lanes may run independently.

## Acceptance

- each API exposes one batch-delete suffix grammar
- collection semantics and `POST` payloads remain unchanged
- clients move before aliases retire unless the caller set is proved local
- no lifecycle, envelope, access, or audit redesign enters the batch
- target-owned Effigy validation and focused route/client tests pass
- one fleet closeout records exact merged tips

## Stop Conditions

Stop if the suffix change would alter action semantics or current source
disproves the closed-world caller inventory.

## Consumer Upgrade Impact

- Impact class: compatibility retirement
- Affected consumers: Underlay Reference, Compli Me
- Required action: move in-repo callers to `:batch-delete` and retire slash
  routes atomically
- Compatibility window: none; the supported caller set is closed-world

## Dispatch Evidence

- Underlay Reference handoff
  `docs/handoffs/20260827-181956-g09-059-task-batch-delete-colon.md` pushed in
  target commit `ff5f14389ce8ba71cf148da56336d8eabb1ac427`
- Compli Me handoff
  `docs/handoffs/20260827-181956-g09-059-batch-delete-colon.md` pushed in target
  commit `8c6d6d9c22d339f26ad454bba3da36a746136d4f`
- both target docs and Northstar QA gates passed before push

## Next Task

Launch the two workers. Review each PR at exact head; merge only with explicit
operator authorisation.
