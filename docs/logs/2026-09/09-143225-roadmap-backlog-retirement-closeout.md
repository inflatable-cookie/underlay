# Roadmap Backlog Retirement Closeout

Date: 2026-09-09
Status: complete, merged
Task: `93ef8609-52d6-4444-9620-7d58f27913c2`
Handoff: [`docs/handoffs/20260909-151020-retire-roadmap-backlog.md`](../../handoffs/20260909-151020-retire-roadmap-backlog.md)

## Outcome

The one-time Northstar roadmap-backlog retirement landed in Underlay PR
[#28](https://github.com/inflatable-cookie/underlay/pull/28). The reviewed
worker head was
`81a81d11757fccea4c8f11d74a3245b9c1786680`; Northstar Queue merged it as
`6be9d71dfad1eb62eeca8394685b5560700e95de` on 2026-09-09.

All 14 items under `docs/roadmaps/backlog/` were dispositioned 1:1 into the
single timestamped triage note
[`docs/triage/20260909-170000-roadmap-backlog-retirement.md`](../../triage/20260909-170000-roadmap-backlog-retirement.md)
as non-authoritative deferred candidates with sources, constraints, open
questions, and promotion conditions. None was approved executable work: no
owning `gNN.NNN` task authorized any item and the active `g11`/`g12`
frontiers do not include them. Migration is not approval.

The change touched 27 files (+251/−1413): 15 backlog files deleted,
`docs/roadmaps/README.md`, `docs/contracts/001-working-rules.md`,
`docs/contracts/README.md`, `docs/contracts/122-rust-public-api-inventory.md`,
`docs/guides/190-upgrade-compatibility.md`, `AGENTS.md`, `README.md`, two
research indexes, and two broken-link repairs
(`docs/roadmaps/archive/g09.md`, the flattened-task switchover manifest).

Current state on merged `main`:

- `find docs -type d -name backlog -print` returns nothing.
- No backlog templates, aliases, moved-to stubs, or empty directories remain.
- Inbound links resolve to the single canonical triage note.
- Roadmaps contain only promoted executable tasks; triage stays
  non-authoritative.
- Remaining `backlog` word uses are pre-existing and unrelated (doctor-scan
  triage note, jobs-sweep wording, `g06` historical roll-up, a `2026-03`
  historical log) or provenance inside the manifest and closed handoffs —
  not live roadmap doctrine.
- The approved frontier is unchanged: `g11.002` ready (Underlay Reference
  first), `g11.003` blocked on `g11.002`, `g12.001` ready for Acowtancy
  Market Card 278.

## Review And Validation

The accepted exact-head review is the
[Northstar review comment](https://github.com/inflatable-cookie/underlay/pull/28#issuecomment-5603541942).
It found no required changes and approved the head for merge.

Post-merge, the clean integration checkout matched local and remote `main`
at `6be9d71dfad1eb62eeca8394685b5560700e95de`. These focused checks passed:

- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:docs:links`
- `effigy qa:docs:agent-defaults`
- `git diff --check`

The worker's full `effigy qa` code gates could not run in its throwaway
worktree because `node_modules` was not installed — a pre-existing
environment condition, not a regression. This change is docs-only (no
product code, workflows, dependencies, or release state), so no
implementation repair is included in this closeout. No failures are
deferred except that already-documented environment note.

## Next Task

Normal dispatch resumes with `g11.002` (Underlay Reference first) while
Acowtancy Market Card 278 runs under `g12.001`. No new planning decision is
needed.
