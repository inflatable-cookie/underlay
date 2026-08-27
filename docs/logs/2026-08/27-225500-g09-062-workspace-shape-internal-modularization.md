# 2026-08-27 22:55:00 - g09.062 Workspace-Shape Internal Modularization

## Outcome

Split the high-severity `workspace-shape.ts` god file into cohesive internal
modules behind the unchanged public facade. Doctor no longer reports a
workspace-shape god-file error. The attention-marker error remains for
`g09.061`.

## Worktree

- root: `/Users/tom/.t3/worktrees/underlay/t3code-0036d3d4`
- branch: `t3code/workspace-shape-modularization`
- base: `origin/main` at `493aa4cbd8bb0981d56ba2d575eae5bf22205dbf`

## What Changed

`ts/src/tools/workspace-shape.ts` stays the public export/bin target. Behavior
moved into:

| Module | Responsibility |
| --- | --- |
| `workspace-shape/model.ts` | rule IDs, types, constants, sort/push helpers |
| `workspace-shape/fs.ts` | path, manifest, lock, and membership discovery |
| `workspace-shape/topology.ts` | Git, root manifest, workspace path, membership, lock checks |
| `workspace-shape/dependencies.ts` | internal edges and released shared `file:` checks |
| `workspace-shape/check.ts` | `checkWorkspaceShape()` orchestration |
| `workspace-shape/report.ts` | `formatWorkspaceShapeReport()` |
| `workspace-shape/cli.ts` | `runWorkspaceShapeCli()` |

Preserved exactly: `WORKSPACE_SHAPE_RULE_IDS`, `WorkspaceShapeRuleId`,
`WorkspaceShapeViolation`, `checkWorkspaceShape()`,
`formatWorkspaceShapeReport()`, `runWorkspaceShapeCli()`, the
`@inflatable-cookie/underlay/tools/workspace-shape` export, and the
`underlay-workspace-shape` bin.

Added one formatter-copy assertion in `ts/tests/tools/workspace-shape.test.ts`.
Existing fixtures were retained.

## God-file Scan

Baseline on this checkout before the split: 559 code lines / 701 total, the
only high-severity finding, plus fourteen advisory warnings.

After the split: `high=0`, `critical=0`, `warning=14`. No workspace-shape path
appears in the finding list. Largest new module is `topology.ts` at 216 total
lines.

## Partial Doctor State

`effigy doctor --verbose` after the split:

- error: `scan.attention-markers` (five errors, one warning) — `g09.061`
- warning: `scan.god-files` (fourteen advisory files, zero errors)
- warning: `scan.comment-ratio`
- warning: `graph.index` stale after the source split
- summary: `ok:16  warn:3  err:1`

This lane removed the god-file error without hiding the attention-marker
error or the advisory inventory.

Shared front doors and `docs/logs/README.md` were not edited.

## Validation

- `effigy check:workspace-shape` — 15 tests passed
- `effigy test:unit ts/tests/tools/workspace-shape.test.ts` — 15 tests passed
- `effigy check:types` — pass
- `effigy scan god-files --json` — no high/critical workspace-shape finding
- `effigy doctor --verbose` — remaining error is attention-markers
- `effigy health` — pass
- `effigy validate` — pass (800 unit, 49 component)
- `effigy qa:docs` — pass
- `effigy qa:northstar` — pass
- `git diff --check` — pass

## Next Task

Stop for orchestrator review of the implementation PR. Do not merge.
