# g08.030 - Archival Docs Weight Reduction

Status: planned
Owner: repo maintainers
Started:
Completed:

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

## Planned Changes

- [ ] Move closed roadmap generations out of the active tree (archive surface),
  keeping the generation-index history.
- [ ] Collapse `usage/` into `guides/` (or vice versa); remove empty
  subdirectories.
- [ ] Treat `contracts/` as the single normative layer and `guides/` as the only
  narrative layer; de-duplicate overlapping auth coverage.
- [ ] Decide the logs cadence honestly (keep per-batch, or drop the ritual).

## Consumer Upgrade Impact

Impact class: `none` (docs).

## Validation

- [ ] `effigy qa:docs`, `effigy qa:docs:links` still pass after moves
- [ ] active docs tree materially smaller; no dead links

## Stop Conditions

Do not delete generation history; archive, don't destroy. Keep the
generation-index log intact.

## Next Task

Lane E complete. `g08` closeout checkpoint: confirm all lanes closed or rehomed
before considering `g09`.
