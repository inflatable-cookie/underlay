# 2026-08-27 23:24:00 - g09.061-062 Doctor Error Closeout

## Outcome

Closed both doctor-error roadmaps after their reviewed heads merged. Underlay's
repo-owned attention-marker policy now ignores compatibility metadata and
ordinary prose while retaining action labels. The workspace-shape checker now
uses cohesive internal modules behind its unchanged public facade.

## Exact Merge Evidence

| Roadmap | PR | Reviewed head | Merge commit |
| --- | --- | --- | --- |
| `g09.061` | Underlay PR14 | `f1666f2e4ae1267860f994d7b57fe4ff22084d20` | `5129356b8a3f22a135bcc958a516d64657f0ee4b` |
| `g09.062` | Underlay PR15 | `056c379e11ebee13bd14376f1cc6a8e7ca8fea35` | `c55a6fe6a7786853186e966fead012fd396e61ec` |

Both PRs were clean and mergeable at their reviewed heads. Both required
GitHub `build + test (with Postgres)` checks passed before merge.

## Doctor Verdict

Refreshed exact `main` at `c55a6fe6`:

- attention markers: zero findings
- god files: fourteen warnings, zero errors
- comment ratio: one warning, zero errors
- graph index: refreshed and current
- summary: `ok:18 warn:2 err:0`

The selected green-doctor finish line is complete. Advisory threshold findings
remain visible and unpromoted.

## Compatibility Verdict

- public Rust and TypeScript deprecations remain intact
- workspace-shape exports, rule IDs, report/CLI copy, diagnostics, package
  export, and bin entry remain stable
- no consumer repository or release version changed
- no consumer action or compatibility window is required

Deprecated API retirement remains a later compatibility candidate. It needs
caller migration and release planning before promotion.

## Planning State

Posture is `strict-paused`. `g09.001`–`g09.062` are complete and no roadmap is
ready. This closeout does not open or roll a generation.

## Next Task

At the next planning checkpoint, decide whether to promote one bounded roadmap
inside `g09` or close the generation. Do not open a later generation without
explicit operator direction.
