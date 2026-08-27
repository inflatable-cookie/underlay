# 2026-08-27 22:38:23 - g09.061-062 Doctor Error Promotion

## Outcome

Promoted the two Underlay doctor error families into independent ready
roadmaps. The operator chose doctor exit success with advisory warnings retained,
not a zero-finding threshold sweep.

## Exact Baseline

Discovery ran from exact pushed `main` commit `60ff292b`:

- attention markers: five errors and one warning
- god files: one error and fourteen warnings
- comment ratio: one warning
- graph index: stale warning, refreshed successfully

The attention findings are public deprecation metadata plus ordinary prose. The
god-file error is `ts/src/tools/workspace-shape.ts` at 559 code lines.

## Promoted Runway

- `g09.061` owns the committed attention-marker policy in `effigy.toml`
- `g09.062` owns internal workspace-shape modularization behind its unchanged
  public facade
- the lanes have no shared implementation or lane-evidence files and may run in
  parallel
- shared roadmap front doors remain orchestrator-owned after both merges

## Boundaries

- do not delete deprecated APIs to satisfy a scanner
- do not retune god-file thresholds or suppress the workspace checker
- do not split fourteen warning-level files merely to reach zero findings
- do not change consumers, releases, package exports, CLI behavior, or stable
  workspace-shape diagnostics

Current Compli Me `origin/main` still uses both deprecated pagination aliases.
Their retirement needs its own compatibility and release sequence.

## Validation

- fresh `effigy doctor --verbose` and JSON evidence
- fresh Effigy graph index and workspace-shape ownership query
- six-consumer exact `origin/main` caller scan for deprecated APIs
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Task

Publish one worker handoff per ready roadmap and dispatch `g09.061` and
`g09.062` in parallel. Review each PR independently; merge only with explicit
operator authorisation.
