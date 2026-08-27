# g09.062 - Workspace-Shape Internal Modularization

Status: complete
Owner: Underlay maintainers
Contract: `024`, `120`
Depends on: `g09.060` (`complete`)

## Purpose

Split the high-severity `workspace-shape.ts` god file into cohesive internal
modules without changing its public checker, CLI, diagnostics, or consumer
contract.

## Decision

The operator chose a green-doctor finish line. The 559-code-line workspace
checker is the only high-severity god-file finding. Its current responsibilities
already expose stable internal seams: model/constants, filesystem and manifest
discovery, topology checks, dependency checks, report formatting, and CLI
dispatch.

Keep `ts/src/tools/workspace-shape.ts` as the stable public facade. Place
implementation modules under `ts/src/tools/workspace-shape/`. Prefer a small
cohesive split; do not create one file per function. Keep every new code file
below the 250-code-line advisory threshold where the natural seams allow it.

## Scope

- preserve these exports exactly:
  - `WORKSPACE_SHAPE_RULE_IDS`
  - `WorkspaceShapeRuleId`
  - `WorkspaceShapeViolation`
  - `checkWorkspaceShape()`
  - `formatWorkspaceShapeReport()`
  - `runWorkspaceShapeCli()`
- preserve the published `@inflatable-cookie/underlay/tools/workspace-shape`
  export and `underlay-workspace-shape` bin
- extract internal model/helper/check groups into cohesive modules
- preserve deterministic rule IDs, paths, details, sort order, help text, and
  exit behavior
- retain and strengthen focused fixture coverage if the split exposes a missing
  seam assertion
- update this roadmap and one lane execution log

## Out Of Scope

- changing Contract `024` topology or adding new conformance rules
- changing consumer repositories, manifests, locks, or pins
- folding env-authority or security conformance into workspace-shape
- changing package exports, CLI syntax, output copy, or stable diagnostics
- retuning god-file thresholds or suppressing the checker path
- refactoring the fourteen warning-level god files
- editing shared roadmap front doors; orchestrator closeout owns them after both
  doctor lanes merge

## Acceptance

- the public facade and package/bin surfaces are unchanged
- all existing workspace-shape fixtures pass without diagnostic drift
- extracted modules follow cohesive responsibilities and do not form circular
  imports
- `effigy scan god-files --json` has no high or critical workspace-shape
  finding and introduces no new warning-level workspace-shape file
- this lane removes the god-file doctor error without hiding other findings

## Validation

- `effigy check:workspace-shape`
- `effigy test:unit ts/tests/tools/workspace-shape.test.ts`
- `effigy check:types`
- `effigy scan god-files --json`
- `effigy doctor --verbose` — expected to retain the attention-marker error
  until `g09.061` merges
- `effigy health`
- `effigy validate`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Evidence

- exact `main` doctor report at discovery commit `60ff292b`
- `docs/triage/20260827-223450-underlay-doctor-scan-backlog.md`
- Contract `120` consumer workspace-shape checker boundary
- current workspace-shape fixture and package-compatibility coverage
- fresh Effigy graph identifying `workspace-shape.ts` as the primary owner

## Execution Evidence

Worker branch `t3code/workspace-shape-modularization` in
`/Users/tom/.t3/worktrees/underlay/t3code-0036d3d4`. Public facade remains
`ts/src/tools/workspace-shape.ts`. Internal modules:

- `workspace-shape/model.ts` — rule IDs, types, constants
- `workspace-shape/fs.ts` — filesystem and manifest discovery
- `workspace-shape/topology.ts` — Git, manifest, membership, lock checks
- `workspace-shape/dependencies.ts` — internal edges and shared `file:` checks
- `workspace-shape/check.ts` — `checkWorkspaceShape()` orchestration
- `workspace-shape/report.ts` — `formatWorkspaceShapeReport()`
- `workspace-shape/cli.ts` — `runWorkspaceShapeCli()`

Package export and `underlay-workspace-shape` bin are unchanged. Existing
fixtures still pass. One report-copy assertion was added for the extracted
formatter seam.

`effigy scan god-files --json` after the split: `high=0`, `critical=0`,
`warning=14`. No workspace-shape file remains in the finding list.

`effigy doctor --verbose` on this branch: `ok:16 warn:3 err:1`. The remaining
error is `scan.attention-markers` (five errors, one warning), owned by
`g09.061`. God-files is warning-only. Comment-ratio and a stale graph index
are the other warnings. This lane did not hide those findings.

Lane log:
`docs/logs/2026-08/27-225500-g09-062-workspace-shape-internal-modularization.md`

## Merge Evidence

- PR: `https://github.com/inflatable-cookie/underlay/pull/15`
- reviewed head: `056c379e11ebee13bd14376f1cc6a8e7ca8fea35`
- merge commit: `c55a6fe6a7786853186e966fead012fd396e61ec`
- both required GitHub checks passed
- exact-head full validation passed: 800 unit and 49 component tests
- combined post-merge doctor: `ok:18 warn:2 err:0`

## Stop Conditions

Stop if the split changes a stable rule, output, CLI, export, package boundary,
or consumer obligation; requires a new architecture decision; or grows into
warning-level refactors outside the checker family.

## Consumer Upgrade Impact

- Impact class: internal refactor
- Affected consumers: none
- Required action: none
- Compatibility window: none; public behavior and exports stay unchanged

## Dispatch Evidence

- planning base: `049fae4dd5f326bfbb08bc97b5e6ef7bfcd6c8b5`
- handoff:
  `docs/handoffs/20260827-224035-g09-062-workspace-shape-internal-modularization.md`
- topology: parallel with `g09.061`; no shared implementation or lane-evidence
  files
- shared front doors remain orchestrator-owned after both reviewed merges

## Next Task

No further work in this roadmap. Re-enter generation planning before promoting
another lane.
