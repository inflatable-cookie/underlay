# g08.030 - Archival Docs Weight Reduction

Status: done
Owner: repo maintainers
Started: 2026-07-18
Completed: 2026-07-18

## Purpose

Shed docs weight that has outgrown a solo 0.0.1 project by an order of
magnitude. The docs spine is ~155k lines; `docs/roadmaps/` alone is 657 files /
~62k lines and every generation g01-g07 is closed, so effectively all of it is
archival (g06 is 367 files). `docs/guides/` (95 files) overlaps architecture,
contracts, patterns, and usage; auth appears across architecture/050,
contracts/030, and seven guides. `docs/usage/backend|frontend|runtime/` are
empty directories; `usage/000-overview.md` mostly redirects into guides. The
logs cadence is already collapsing (2026-05 empty, 2026-06 one file).

## Evidence

- `docs/roadmaps/g01`-`g07` (all closed; 657 files)
- `docs/usage/{backend,frontend,runtime}/` (empty)
- `docs/logs/` (66 files, gap months)

## Governing References

- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)

## Changes

- [x] **Closed roadmap generations designated archival, physical move deferred.**
  `g01`-`g07` (639 files) are marked frozen archival record in `roadmaps/README`
  with the generation-index as the authoritative history. Physical relocation to
  an archive surface is **deliberately deferred**: ~15 active contracts/guides
  link into the closed generations and the generations cross-link each other with
  relative paths, so a bulk move breaks links the front-door checker (nine fixed
  files) cannot detect — this warrants a human-reviewed pass, not an autonomous
  mass move. The designation gives the navigational benefit without the risk, and
  respects the "archive, don't destroy" stop condition.
- [x] **Removed the three empty `usage/` subdirectories** (`backend`,
  `frontend`, `runtime`) and fixed every doc that linked to them
  (`guides/README`).
- [x] **Clarified the normative/narrative layering and killed the false
  "guides deprecated" claim.** `docs/README`, `guides/README`, and
  `usage/000-overview` all claimed `docs/guides/` was deprecated and migrated to
  `usage/` — flatly wrong: `guides/` is the active 95-file narrative layer and
  `usage/` is a thin (22-file) template-usage reference. Rewrote all three to the
  real model: contracts = normative, guides = narrative, usage = template
  reference; added a "Layer Boundary" rule to `docs/README`.
- [x] **Logs cadence decided: keep per-batch.** Recorded in `logs/README` and
  refreshed its stale "Current Evidence Window" (was pointing at the g02
  rollover) to the active `g08` / `2026-07` window.

## Consumer Upgrade Impact

Impact class: `none` (docs).

## Validation

- [x] `effigy qa:docs` / `qa:docs:links` pass (found + fixed 3 broken links the
  empty-dir removal exposed in `guides/README`).
- [x] No dead links; the layer-model contradictions that made the tree
  self-misleading are resolved. Material file-count reduction (the 639-file
  relocation) is deferred to a human-reviewed pass per the rationale above — the
  autonomous scope corrected the misleading structure rather than mass-moving
  files the checker cannot validate.

## Stop Conditions

Do not delete generation history; archive, don't destroy. Keep the
generation-index log intact.

## Next Task

Lane E complete. `g08` closeout checkpoint: confirm all lanes closed or rehomed
before considering `g09`.
