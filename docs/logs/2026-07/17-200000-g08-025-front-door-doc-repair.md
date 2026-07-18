# 2026-07-17 - g08.025 front-door doc repair

## Context

The docs a new consumer hits first were actively misleading: the quickstart was a
pre-`g06` fossil, the README carried a dead CI badge, the crate count was wrong,
and several "Next Task" pointers named closed milestones that QA checked for
presence but not truth.

## Changes

- **Quickstart 060.** Was a 1200-line step-by-step for a monorepo layout no
  consumer uses (`apps/bloom`, `libs/petal`, a `legacy/libraries` symlink),
  pinning `axum 0.7` (workspace is 0.8) and `bun 9+`, with a GitHub Actions CI
  the repo lacks. Reduced to a short "superseded" pointer at contract 024 (new
  app bootstrap) and `guides/020-project-structure`. No inbound links existed
  except this card, so the collapse is safe.
- **README CI badge.** Removed the `decodelabs/underlay` `rust.yml` badge - there
  is no `.github/` anywhere in the repo, so it rendered permanently broken.
  Minimal-CI creation stays with the blocked `g08.019` (Postgres CI need).
- **Crate count.** `010-package-map.md` and `000-overview.md` both said "31";
  the workspace has 36. Fixed the count and added the four omitted crates -
  `underlay-config`, `underlay-http-client`, `underlay-query` to Core;
  `underlay-aws` to Infrastructure. Verified the table now matches
  `rust/crates/*` exactly by set-diff (36/36, no drift either direction).
- **Stale next-action pointers.** Vision `001` said "Open `g01.042`"; logs README
  aligned the window to `g02.001`; `g01/README.md` carried a live "use `g02`"
  instruction. All three named closed generations. Repointed vision and logs at
  the active `g08` front door; fixed the g01 live instruction to defer to the
  roadmaps front door while keeping the historical closeout (g01 genuinely rolled
  into g02) intact. Added the `g08` front door to `qa:docs:links` so the active
  generation's links are checked; `qa:northstar:g01` validates g01's permanent
  archival headings and is retained.
- **Envelope doc drift.** `015` pointed at `ts/src/client/types.ts` for envelope
  types; they live in `envelopes.ts` (re-exported via `types.ts`). Fixed both
  references and documented `PagedListResponse<T>` (`{ data, total, hasMore }`),
  noting the Rust wire shape is produced by `underlay-http`'s `Paginated<T>`
  (`dto.rs` holds only `SingleResponse`/`ListResponse`).

## Validation

- `effigy qa:docs`: link check, docs index (vision), forbidden check, and
  next-action (vision) all pass. The next-action policy rejected a `Track ...`
  lead verb; rephrased to `Open ...`.
- `effigy qa:northstar`: vision / roadmaps / g01 heading checks pass.
- `effigy qa:docs:links`: pass (now including `docs/roadmaps/g08/README.md`).
- Surviving quickstart pointer resolves real paths: contract 024 and
  `guides/020-project-structure.md` both exist.

## Consumer Upgrade Notes

Impact class **none** (docs only). No code, contract behavior, or exported
surface changed.

## Next

`g08.026` committed-artifact cleanup.
