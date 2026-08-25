# g10.004 — Acowtancy evidence repair

Date: 2026-08-25
Card: `docs/roadmaps/g10/batch-cards/004-acowtancy-evidence-repair.md`
Consumer PR: [acowtancy/market#57](https://github.com/acowtancy/market/pull/57)

## Summary

Aligned Acowtancy's active dependency documentation with its already-compliant
monorepo manifests. Sibling Underlay/Poodle mounts are now described as Effigy
QA/tooling and optional local co-development inputs, not committed application
dependency sources.

## Changed Consumer Surfaces

| Surface | Change |
| --- | --- |
| root `README.md` | Released application dependency boundary and optional local Cargo link |
| root `AGENTS.md` | QA/tooling-only sibling-mount boundary |
| `packages/cattle-grid/README.md` | Internal `workspace:*` and root-install workflow |

No manifests, locks, task wiring, runtime code, workflows, or historical
evidence changed.

## Review And Merge Evidence

The Acowtancy-side orchestrator approved worker commit
`dec01128d87ae6e6d7384bf1f95d59a15e933054`. PR #57 merged to Acowtancy
`main` as `b995fad517783ee09b00e384f903988dccbb2b79` on 2026-08-25. The
canonical approval record is the
[review comment](https://github.com/acowtancy/market/pull/57#issuecomment-5417592324).

Independent closeout inspection confirmed the merged diff contains only the
three carded active Markdown files. Current Acowtancy `main` still passes docs
QA and the live Underlay workspace-shape checker. The PR records full
`effigy health` passing with the existing 29 Dairy Svelte warnings and existing
Vite configuration warning.

## Validation

```text
effigy docs/qa:docs                                                 pass
effigy docs/qa:northstar                                            pass
effigy health                                                       pass (worker evidence)
bun /Users/tom/Dev/projects/underlay/ts/bin/underlay-workspace-shape.ts .  pass
git diff --check                                                    pass
```

## Underlay Reference Readiness

Underlay Reference `main` is clean at
`3354803ebc484d4d611878a8e60e356ab92e206e` with no open PR. Its released
Underlay/Poodle boundary is already correct after its v0.9.4 adoption. The
remaining migration baseline is four child Bun locks, four internal `file:`
edges, no root lock, and no root `packageManager` or workspaces. The
workspace-shape checker reports those seven expected violations.

The target's docs QA passes. Doctor retains pre-existing schema, scan, and
built-in docs-reference findings; these do not change the migration contract.

## Next Task

Execute `g10.005` — Underlay Reference normalization — through a fresh
orchestrator-dispatched worker handoff. No other card is ready.
