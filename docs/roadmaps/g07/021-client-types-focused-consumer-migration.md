# g07.021 - Client Types Focused Consumer Migration

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Move the known consumer imports from aggregate
`@decodelabs/underlay/client/types` to the focused client subpaths added in
`g07.020`.

## Governing References

- [010 foundation primitives and envelopes](../../contracts/010-foundation-primitives-and-envelopes.md)
- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [190 upgrade compatibility matrix](../../guides/190-upgrade-compatibility.md)
- [020 client types subpath split](020-client-types-subpath-split.md)

## Changes

- [x] Move `SingleResponse` imports in health clients to
  `@decodelabs/underlay/client/envelopes`.
- [x] Move restore-blocker DTO and guard imports to
  `@decodelabs/underlay/client/restore`.
- [x] Refresh affected Bun file-dependency installs so newly-created client
  files are materialized in consumer `node_modules`.
- [x] Fix `contact-patch/cp-client` `ignoreDeprecations` from `6.0` to `5.0`
  so its TypeScript 5.9 check remains runnable.

## Consumer Upgrade Impact

Impact class: `additive`.

No behavior changed. Existing aggregate `client/types` imports remain valid for
unknown consumers. The known consumer roots now use the focused client subpaths
in live source.

## Consumer Proof

- `underlay-reference`: `acme-client` health response envelope import moved.
- `contact-patch`: `cp-client` health response envelope import moved.
- `compli-me`: `api-client` health response envelope import moved.
- `acowtancy`: `cattle-grid` restore DTO/guard imports moved.
- `acowtancy`: `dairy` restore presentation/helper imports moved.
- `songsprout`: no live `client/types` imports found.
- `loophole/composer`: no live `client/types` imports found.

Final source scan found no remaining live aggregate
`@decodelabs/underlay/client/types` imports in the six-consumer family.

## Validation

- [x] `underlay-reference`: `effigy acme-client/check`
- [x] `contact-patch`: `effigy cp-client/check`
- [x] `compli-me`: `effigy api-client/check`
- [x] `acowtancy`: `effigy cattle-grid/check`
- [x] `acowtancy`: `effigy dairy/check`
- [x] `underlay`: `effigy qa:docs`
- [x] `underlay`: `effigy qa:northstar`

## Next Task

Keep `client/types` as the aggregate compatibility path unless a later
retirement card proves no unknown consumer risk remains.
