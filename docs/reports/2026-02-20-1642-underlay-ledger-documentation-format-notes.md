# 2026-02-20 16:42 - Underlay + Ledger Documentation Format Notes

## Purpose

Capture the shared documentation format conventions observed in Underlay and Ledger so future roadmap/process docs stay consistent.

## Underlay docs format observations

- Roadmaps are numbered and ordered in `docs/roadmap/` (`001` onward) with index in `docs/roadmap/README.md`.
- Typical roadmap header pattern:
  - title line `# <number> - <name>`
  - metadata lines: `Status`, `Owner` (sometimes omitted), `Created`, optional `Depends on`
- Common roadmap structure:
  - `Problem`
  - `Goals`
  - `Non-Goals`
  - phased execution sections
  - `Acceptance Criteria`
  - `Risks and Mitigations`
  - `Deliverables`
- Checklist-first execution style is preferred (`- [ ]` / `- [x]`), with phases used as progress anchors.
- Index status values are normalized to `Complete` or `In progress` in the table.

## Ledger docs format observations

- Ledger uses parallel numbered roadmap structure in `ledger/roadmap/` (currently up to `051`).
- Ledger roadmap files also use explicit metadata lines (`Status`, `Owner`, `Created`, `Depends on`) and numbered sections (`## 1)`, `## 2)`, etc.).
- Sweep/report-driven process is explicit:
  - roadmaps often reference scripts under `ledger/scripts/`
  - acceptance criteria are tied to measurable sweep outputs.
- Reports are timestamped artifacts in `ledger/reports/` and should be additive rather than rewriting history.
- AGENTS guidance explicitly requires roadmap progress accuracy and parent checklist updates.

## Cross-repo consistency notes

- Keep roadmap docs execution-oriented with visible checklists and explicit acceptance criteria.
- Prefer clear dependencies (`Depends on`) to signal sequencing.
- Keep index files current immediately after adding a new roadmap.
- When adding new capability tracks, pair:
  - roadmap item (what to build),
  - report/sweep artifact (how to verify it worked).

## Actionable guardrails for future work

- Always add new Underlay roadmap entries using next numeric prefix and update `docs/roadmap/README.md` in the same change.
- Include `Status`, `Created`, and dependency metadata at top of roadmap docs.
- Treat root docs (`AGENTS.md`, roadmap README, key guides) as first-class coordination surfaces for agents.
