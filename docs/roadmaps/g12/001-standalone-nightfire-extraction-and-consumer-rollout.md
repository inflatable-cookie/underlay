# g12.001 - Standalone Nightfire Extraction And Consumer Rollout

Status: active
Owner: repo maintainers and Acowtancy cross-repo Coordinator
Created: 2026-09-04
Depends on: operator choice of standalone repository
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/070-nightfire-and-migration-systems.md`

## Problem

Froyo uses only Underlay's Nightfire renderer, editor, registry, validation,
and markdown subpaths, but its package dependency brings the entire Underlay
JavaScript package into downstream installation and distribution. Bovine
Desktop then has to freeze, resolve, and package Underlay despite consuming no
other Underlay surface through Froyo.

## Invariant

Froyo and Bovine Desktop can consume the generic Nightfire runtime without an
installed, bundled, frozen, or source-linked `@inflatable-cookie/underlay`
package. Existing Underlay Nightfire callers keep working through one
temporary compatibility facade over the same released implementation.

## Package Boundary

- repository: `github.com/inflatable-cookie/nightfire`
- local sibling checkout: `../nightfire`
- root package: `@inflatable-cookie/nightfire`
- distribution: immutable Git tags; no private registry prerequisite
- package shape: one root package with focused subpaths, not independently
  versioned sibling packages
- ownership: generic TypeScript protocol types, registries, validation,
  strategies, renderer, editor, markdown, and media helpers
- exclusions: Underlay, SvelteKit, Vite, templates, auth, client/runtime
  helpers, product block types, Acowtancy schemas, and the Rust
  `underlay-nightfire` crate

## Ordered Delivery

1. Market Card 272 creates the standalone repository, extracts the exact
   TypeScript/Svelte implementation and tests, freezes subpaths and dependency
   policy, and proves browser/SSR/Tauri-safe rendering without Underlay.
2. Market Card 273 cuts and validates `v0.1.0` only after Card 272 acceptance
   and explicit operator release confirmation.
3. Market Card 274 repoints Underlay to the released tag, deletes the internal
   TypeScript implementation, and retains byte/behavior-compatible historical
   subpath re-exports.
4. Market Card 275 repoints Froyo directly to the released tag and proves its
   package and bundle graphs contain no Underlay edge.
5. Market Card 276 refreshes Bovine Desktop's private candidate, frozen
   consumer graph, resolvers, locks, and package evidence with Nightfire in
   place of Underlay.

Cards 274 and 275 may run in parallel. Card 276 depends on Card 275, not on an
Underlay release. A later Underlay release and compatibility retirement need
their own release authorization and caller proof.

## Review Oracle

| Invariant | Smallest counterexample | Required response |
| --- | --- | --- |
| Nightfire is independent. | Its manifest imports or peers on Underlay or framework-only packages. | Reject the extraction. |
| One TS authority exists. | Underlay retains an editable implementation after Card 274. | Delete it; keep re-exports only. |
| Wire behavior stays aligned. | Rust and TS conformance fixtures disagree. | Stop before release. |
| Sanitization is not weakened. | Malicious markdown/HTML differs across browser, SSR, or Tauri. | Stop and retain the proven sanitizer path. |
| Froyo is clean. | Its installed or bundled graph still contains Underlay. | Card 275 cannot close. |
| Desktop is clean. | Candidate receipts, locks, Vite resolution, or package contents still mention Underlay. | Card 276 cannot close. |
| Existing callers are safe. | An historical Underlay Nightfire subpath resolves differently. | Repair the compatibility facade before merge. |

## Consumer Upgrade Impact

Impact class: `deprecation`. New TypeScript consumers import the standalone
package. Existing Underlay imports remain temporarily valid. No product schema,
block payload, persisted envelope, or Rust API changes.

## Stop Conditions

- repository visibility, licence, or package ownership cannot match Underlay;
- the package cannot be installed from an immutable Git tag without registry
  publication;
- extracting the sanitizer introduces unsafe or environment-divergent output;
- a product-specific block or schema must enter the generic package;
- Rust movement becomes necessary;
- release authorization is absent when Card 273 becomes ready.

## Next Task

Run Acowtancy Market Card 272. Do not tag a release yet.
