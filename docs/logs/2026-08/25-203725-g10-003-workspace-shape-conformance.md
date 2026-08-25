# g10.003 — workspace-shape conformance

Date: 2026-08-25
Card: `docs/roadmaps/g10/batch-cards/003-workspace-shape-conformance.md`
Branch: `t3code/workspace-shape-conformance`
Worktree: `/Users/tom/.t3/worktrees/underlay/t3code-d49b3a65`

## Summary

Added a distributable consumer workspace-shape checker separate from security
conformance, with fixture coverage, Effigy self-tests, contract/artifact
alignment, and consumer integration guidance.

## Changed surfaces

| Surface | Change |
| --- | --- |
| `ts/src/tools/workspace-shape.ts` | Generic checker + CLI for contract `024` topology |
| `package.json` | Export `@inflatable-cookie/underlay/tools/workspace-shape` |
| `effigy.toml` | `check:workspace-shape` fixture self-test in `health` |
| `ts/tests/fixtures/workspace-shape/*` | Compliant and failing workspace fixtures |
| `ts/tests/tools/workspace-shape.test.ts` | Focused checker, fixture, and published-bin tests |
| `docs/contracts/120-tooling-testing-and-contract-artifacts.md` | Workspace-shape checker ownership |
| `docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md` | Mechanical bootstrap check rule |
| `docs/contracts/app-review/underlay-app-review-checklist.json` | `workspace-shape` mechanical check on bootstrap domain |
| `docs/guides/030-underlay-integration.md` | Consumer `qa:workspace-shape` Effigy task pattern |

## Review follow-up (2026-08-25)

Addressed orchestrator changes requested on PR #8:

1. Removed workstation-local Acowtancy path from `health` tests; fixture-only
   self-tests remain in `check:workspace-shape`.
2. Added workspace membership and outside-root enforcement with new fixtures.
3. Published `underlay-workspace-shape` bin entry and documented consumer
   invocation through `bunx underlay-workspace-shape .`.

### Re-review follow-up (2026-08-25)

4. Workspace containment now uses filesystem `realpath` so in-root symlink
   targets outside the Git root emit `workspace-path-outside-root`; fixture
   `workspace-symlink-outside-root` covers the bypass.

## Review And Merge Evidence

Final orchestrator re-review approved
`d1baca3d0d715bd235e8dbd6fd01b5887a3a639d` after the symlink-escape fixture,
ordinary `effigy validate`, packed-package bin smoke, and Acowtancy live proof
all passed. PR [#8](https://github.com/inflatable-cookie/underlay/pull/8)
merged to `main` as `dc5d26668b554d00520a404dd6d17137f3663f74`
on 2026-08-25. The canonical approval record is the
[final review comment](https://github.com/inflatable-cookie/underlay/pull/8#issuecomment-5416874040).


- Fixture suite: compliant workspace passes; nested Git, manifest drift,
  wildcard/unresolved workspaces, child locks, internal `file:`, and non-
  `workspace:*` internal edges fail with stable rule ids.
- Live proof (review evidence, not CI): orchestrator re-review recorded
  `bun ts/bin/underlay-workspace-shape.ts /Users/tom/Dev/projects/acowtancy`
  with zero violations on 2026-08-25. Underlay `health`/`validate` stay fixture-
  only via `check:workspace-shape`.
- Underlay does not run the consumer topology check against its own foundation
  root; `check:workspace-shape` runs fixture self-tests only.

## Validation

```text
effigy check:workspace-shape          pass
effigy qa:docs                        pass
effigy qa:northstar                   pass
effigy validate                       pass
git diff --check                      pass
```

## Stop boundaries respected

- No edits to `scripts/check-consumer-conformance.sh`
- No consumer-repository edits
- No Effigy schema changes
- No `.github/workflows/` edits
- `g10.004` is ready after review and merge

## Next task

Execute `g10.004` — Acowtancy evidence repair — through a fresh
orchestrator-dispatched Acowtancy worker handoff. No other card is ready.
