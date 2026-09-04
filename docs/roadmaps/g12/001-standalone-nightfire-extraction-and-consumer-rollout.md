# g12.001 - Standalone Nightfire Extraction And Consumer Rollout

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Created: 2026-09-04
Corrected: 2026-09-04 — operator confirmed dual-language ownership
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/070-nightfire-and-migration-systems.md`

## Problem

Nightfire is one generic content system implemented across Rust and
TypeScript/Svelte. Its Rust crate and browser/editor runtime currently have
different repository authorities. Froyo's dependency also brings the complete
Underlay JavaScript package into downstream installation and distribution.
Both splits are accidental package boundaries.

## Invariant

Rust and TypeScript consumers can use the generic Nightfire system from one
immutable standalone release. Froyo and Bovine Desktop need no installed,
bundled, frozen, or source-linked `@inflatable-cookie/underlay` package.
Existing Underlay Nightfire callers keep working through temporary Rust and
TypeScript compatibility facades over the same released implementations.

## Package Boundary

- repository: `github.com/inflatable-cookie/nightfire`
- local sibling checkout: `../nightfire`
- root npm package: `@inflatable-cookie/nightfire`
- root Cargo workspace with crate: `nightfire`
- distribution: immutable Git tags; no private registry prerequisite
- repository shape: TypeScript/Svelte implementation under `ts/`, Rust
  implementation under `rust/`, and shared root wire fixtures; root
  `package.json` remains the npm Git-install entry
- ownership: generic Rust protocol/registries/strategies/validation/hashing/
  IDs/media locators and generic TypeScript protocol types, registries,
  validation, strategies, renderer, editor, markdown, and media helpers
- exclusions: Underlay, SvelteKit, Vite, templates, auth, client/runtime
  helpers, product block types, Acowtancy schemas, Underlay media traversal,
  and validation-to-HTTP adapters

## Ordered Delivery

1. Market Card 272 created the standalone repository and extracted only the
   TypeScript/Svelte implementation. It is closed incomplete after correction
   of the mistaken Rust exclusion.
2. Market Card 278 reshapes the repository into explicit `ts/` and `rust/`
   tranches, extracts the exact Rust `nightfire` crate and tests, and proves
   both Git-consumer paths plus shared conformance without Underlay.
3. Market Card 273 cuts and validates `v0.1.0` only after Card 278 acceptance
   and explicit operator release confirmation.
4. Market Card 274 repoints Underlay to the released tag, deletes both internal
   implementations, and retains behavior-compatible historical TypeScript
   subpaths and Rust `underlay-nightfire` crate-name facades.
5. Market Card 275 repoints Froyo directly to the released tag and proves its
   package and bundle graphs contain no Underlay edge.
6. Market Card 279 repoints Farmyard's generic Rust core dependency directly to
   the released `nightfire` crate without moving app-local blocks or schemas.
7. Market Card 276 refreshes Bovine Desktop's private candidate, frozen
   consumer graph, resolvers, locks, and package evidence with Nightfire in
   place of Underlay.

Cards 274, 275, and 279 may run in parallel. Card 276 depends on Card 275, not
on an Underlay release. A later Underlay release and compatibility retirement
need their own release authorization and caller proof.

## Review Oracle

| Invariant | Smallest counterexample | Required response |
| --- | --- | --- |
| Nightfire is independent. | Either manifest imports or peers on Underlay or framework-only packages. | Reject the extraction. |
| One dual-language authority exists. | Underlay retains an editable Rust or TS implementation after Card 274. | Delete it; keep facades only. |
| Wire behavior stays aligned. | Rust and TS conformance fixtures disagree. | Stop before release. |
| Sanitization is not weakened. | Malicious markdown/HTML differs across browser, SSR, or Tauri. | Stop and retain the proven sanitizer path. |
| Farmyard uses Rust authority directly. | Its generic core still resolves through Underlay. | Card 279 cannot close. |
| Froyo is clean. | Its installed or bundled graph still contains Underlay. | Card 275 cannot close. |
| Desktop is clean. | Candidate receipts, locks, Vite resolution, or package contents still mention Underlay. | Card 276 cannot close. |
| Existing callers are safe. | A historical Rust crate name or TS subpath resolves differently. | Repair the compatibility facade before merge. |

## Consumer Upgrade Impact

Impact class: `deprecation`. New Rust and TypeScript consumers import the
standalone packages. Existing Underlay paths remain temporarily valid. No
product schema, block payload, persisted envelope, or generic Rust/TS API
changes are authorized.

## Stop Conditions

- repository visibility, licence, or package ownership cannot match Underlay;
- either package cannot be installed from an immutable Git tag without registry
  publication;
- extracting the sanitizer introduces unsafe or environment-divergent output;
- a product-specific block, schema, media traversal, or HTTP adapter must enter
  the generic repository;
- release authorization is absent when Card 273 becomes ready.

## Next Task

Run Acowtancy Market Card 278. Do not tag a release yet.
