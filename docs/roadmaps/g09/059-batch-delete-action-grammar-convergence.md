# g09.059 - Batch Delete Action Grammar Convergence

Status: complete
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

## Merge Evidence

- Underlay Reference PR9 merged reviewed head
  `dc866aa4762e5e142299fdc23a452e9af1f844c4` as
  `0109b906272c7ea39e5e84bb4034ff08d0043f48`
- Compli Me PR8 merged reviewed head
  `d7b46b8287f65b33b39dc773460a3fd569b3d80d` as
  `a290d2a783bdfbe1deac52c96a1fd5264e46d624`
- both remote `main` tips matched those merge commits during closeout

## Next Task

No further `g09.059` work remains. `g09.058` is also complete; re-enter planning
before opening another roadmap.
