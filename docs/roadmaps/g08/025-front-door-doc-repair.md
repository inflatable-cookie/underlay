# g08.025 - Front-Door Doc Repair

Status: done
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Repair the actively-misleading front-door docs a new consumer hits first.
Quickstart 060 is a pre-g06-reset fossil: it describes a monorepo layout
(`apps/bloom`, `libs/petal`, symlink story) no consumer uses, pins `axum 0.7`
(workspace is 0.8), says "bun 9+", references paths that don't exist, and
documents a GitHub Actions CI the repo lacks. The README shows a
`decodelabs/underlay` `rust.yml` CI badge with no `.github/` anywhere. The
package map says "31 crates" (actually 35, omitting aws/config/http-client). The
vision doc and logs README carry stale "Next Task" pointers that QA checks for
presence, not truth.

## Evidence

- `docs/architecture/060-new-project-quickstart.md` (fossil; overlaps
  `docs/contracts/024`)
- `README.md:3` (dead CI badge); no `.github/`
- `docs/architecture/010-package-map.md:5`, `000-overview.md` (crate count)
- `docs/vision/001-underlay-foundation-vision.md` ("Open g01.042"),
  `docs/logs/README.md` (g02 window)
- envelope doc filename drift `docs/architecture/015-error-and-envelopes.md:19,80`
  (points at `types.ts`, now `envelopes.ts`)

## Governing References

- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)
- [024 New app bootstrap and bring-up](../../contracts/024-new-app-bootstrap-and-bring-up.md)

## Planned Changes

- [x] Reduced quickstart 060 (1200-line pre-`g06` monorepo fossil) to a thin
  "superseded" pointer at contract 024 + `guides/020-project-structure`. No
  inbound links existed except this card.
- [x] Removed the dead `decodelabs/underlay` `rust.yml` CI badge from the README
  (no `.github/` in the repo). Minimal-CI creation stays with blocked `g08.019`.
- [x] Corrected the package-map + overview crate count `31 -> 36` and added the
  four omitted crates (`config`, `http-client`, `query` to Core; `aws` to
  Infrastructure). Table now matches `rust/crates/*` exactly (36/36, verified by
  diff).
- [x] Refreshed the vision and logs-README next-action pointers (were `g01.042`
  / `g02.001`, now defer to the `g08` front door) and the stale live instruction
  in `g01/README.md` ("use g02" -> defer to the roadmaps front door; historical
  closeout kept). Added the `g08` front door to `qa:docs:links`; the
  `qa:northstar:g01` archival-heading check is valid and retained.
- [x] Fixed the `015` envelope filename reference (`types.ts` ->
  `envelopes.ts`, re-exported via `types.ts`) and added `PagedListResponse`
  (`{ data, total, hasMore }`; Rust wire shape produced by `underlay-http`
  `Paginated<T>`, not `dto.rs`).

## Consumer Upgrade Impact

Impact class: `none` (docs).

## Validation

- [x] `effigy qa:docs` (links, index, forbidden, next-action), `effigy
  qa:northstar` (vision/roadmaps/g01 heading checks), `effigy qa:docs:links` -
  all pass.
- [x] The surviving quickstart pointer resolves real paths: contract 024 and
  `guides/020-project-structure.md` both exist.

## Stop Conditions

None.

## Next Task

`g08.026` committed-artifact cleanup.
