# 2026-07-18 - g08.030 archival docs weight reduction

## Context

The docs spine (~155k lines) has outgrown a solo project. `docs/roadmaps/` is 688
md files; `g01`-`g07` are all closed (639 files, `g06` alone 367). `docs/usage/`
had three empty subdirectories and a front door that redirected into guides. The
docs front doors carried a **false and self-contradictory** claim that
`docs/guides/` was deprecated and migrated to `usage/` — while `guides/` is in
fact the active 95-file narrative layer and `usage/` is a thin (22-file) template
reference. The logs cadence pointer was stale (aimed at the g02 rollover).

## Changes

- **Archival designation over physical move.** `g01`-`g07` are marked frozen
  archival record in `roadmaps/README` (generation-index remains the
  authoritative history). Physical relocation is **deliberately deferred**: ~15
  active contracts/guides link into the closed generations, which also cross-link
  each other relatively, so a bulk move breaks links the nine-file front-door
  checker cannot catch. That is a human-reviewed operation, not an autonomous
  mass move. The designation delivers the navigational benefit without the risk
  and honours "archive, don't destroy."
- **Empty `usage/` subdirs removed** (`backend`, `frontend`, `runtime`) and the
  `guides/README` links that pointed at them fixed (they surfaced as 3 broken
  links in the link check — found and cleared).
- **Layer model corrected across all three front doors.** `docs/README`,
  `guides/README`, and `usage/000-overview` each claimed `guides/` was deprecated.
  Rewrote them to the real model — contracts = normative, guides = narrative,
  usage = admin template reference — and added a "Layer Boundary" rule to
  `docs/README` (contracts own guarantees; guides explain and link to them).
- **Logs cadence: keep per-batch.** Recorded in `logs/README` and refreshed its
  stale "Current Evidence Window" to the active `g08` / `2026-07` window.

## Validation

- `effigy qa:docs` / `qa:docs:links` pass (the empty-dir removal exposed 3 broken
  `guides/README` links, now fixed).
- No dead links; the self-misleading layer contradictions are resolved. The
  639-file relocation (the only "material file-count" lever) is deferred with
  rationale rather than executed blind.

## Consumer Upgrade Notes

Impact class **none** (docs). No code, contract behaviour, or exported surface
changed.

## Next

Lane E complete. `g08` closeout checkpoint.
