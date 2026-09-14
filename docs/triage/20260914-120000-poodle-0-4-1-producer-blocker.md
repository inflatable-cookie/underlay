# Poodle 0.4.1 producer blocker — Underlay has no ready update task

Status: open
Captured: 2026-09-14
Authority: operator directive to move every project pinning Poodle to published 0.4.1; Acowtancy `g05.070` blocked handoff `20260914-095635-g05-070-poodle-0-4-1-consumer-adoption.md` (`status: blocked`) whose prerequisite is an accepted Underlay release with updated Poodle 0.4.1 metadata.

## Observation

Underlay `v0.9.8` still declares Poodle 0.3.0. There is no Underlay task file owning the 0.4.1 producer bump, so the planning rubric is not met and no committed ready handoff was promoted.

## Why it matters

Market cannot submit its `g05.070` worker until the exact accepted Underlay tag, commit, and package bytes declaring Poodle 0.4.1 exist. Market's handoff owns only the three Market manifests plus root `bun.lock` and must not edit Underlay, so the producer side has to move first.

## Known (evidence, no inference beyond cited paths)

- Underlay committed pin scope: root `package.json` `dependencies.@inflatable-cookie/poodle-svelte` exact `0.3.0`; `bun.lock` records `@inflatable-cookie/poodle-svelte@0.3.0` plus transitive `@inflatable-cookie/poodle-core@0.3.0`. `Cargo.toml` workspace has no Poodle edge. `ts/tests/fixtures/**/package.json` hits are test fixtures, not live manifests. `docs/guides/020-project-structure.md` shows a `0.2.2` example and `contracts/ui/*poodle*` snapshots are historical (read by no live check per contract 120) — docs-only, not pin scope.
- Published Poodle 0.4.1 contract: `poodle/package.json` `0.4.1`; `packages/core/package.json` `0.4.1` public; `packages/svelte/components/package.json` `0.4.1` public with `dependencies.@inflatable-cookie/poodle-core` exact `0.4.1`, new ordinary deps (CodeMirror 6.x, TipTap 3.31.3, `@lezer/highlight`), peers `svelte >=5.56.8 <6` and optional `marked ^18.0.9`. `docs/release-notes/0.4.1.md`: 0.4.1 itself has no breaking API change; Core+Svelte are the publication set, React stays private, Cargo/native unchanged.
- Crossing warning: `0.3.0` → `0.4.1` crosses the `0.4.0` breaking minor (Slider `appearance`→`variant`, Markdown safe-by-default where `renderHtml` no longer implies trust). Underlay callers needing a census: `ts/src/nightfire/markup/MarkdownEditor.svelte` and `MarkdownEditorSurface.svelte` pass `renderHtml={renderSafeMarkdownPreview}` (already sanitizing — compatible in principle, unproved); no direct Poodle `Slider` imports found (only Nightfire-internal `SUMMARY_IMAGE_SLIDER_TYPE` strings in `summary-transform.ts`); remaining broad Poodle imports (Pagination, Tabs, etc.) not yet censused against 0.4.0 removals.
- Underlay planning: `g11.002` is `ready` but explicitly excludes new Underlay releases; `g12.001` is `active` serial behind Card 278; generation-index Next Task names only those two lanes. No Poodle 0.4.1 file exists under `docs/roadmaps/g11/` or `g12/`.
- Release contract (`docs/contracts/023-release-and-compatibility-rollout.md`): Underlay ships via immutable Git tags (`private: true`); a consumer cannot pin an unreleased commit; rollout order is land → tag → validate → reference consumer first. The next version number (current `0.9.8` in both `package.json` and `Cargo.toml`) is undecided.
- Longhorn coordination: `longhorn/package.json` devDeps pin `poodle-core`/`poodle-svelte` `0.3.0` in a separate repo. No 0.4.1 lane is planned there yet (prior `g16.109` wave was the 0.3.0 Tier-1 lane; Poodle `g18.037` notes Longhorn adoption waits for a later patch). Underlay↔Longhorn are parallel Tier-1 consumers of published Poodle, not serial. Desktop `g02.109` needs the accepted Market commit plus Underlay and Longhorn prerequisites (per Acowtancy `g05.070` Next Task) — that is the join point, not an Underlay→Longhorn dependency.

## Unknown / unresolved (each blocks `ready`)

1. Owning generation and task placement for the producer bump (`g11`/`g12` extension vs. new lane) — no file, no frontier entry.
2. Completed 0.4.0-breaking census over Underlay's Poodle callers with a pass/fail disposition per removed/changed API.
3. Measured lockfile scope: exact `bun.lock` importer rows, new transitive identities/integrities, and the frozen-rerun reproducibility oracle.
4. Declared release bump, upgrade-note shape, and reference-first rollout order per contract 023.
5. Confirmed Longhorn parallel-lane coordination (who plans it, exact pins, Desktop join timing) — no cross-repo authority assumed.

## Impact of promoting now

A ready handoff today would smuggle unconfirmed intent: unbounded compatibility fallout, unmeasured lock churn, an undeclared release version, and an assumed Longhorn ordering the contracts do not support.

## Next decision owner

Underlay operator plus repo maintainers in Chatterbox planning. Needed: confirm generation placement, order the Poodle-caller census, declare the release bump and 023 rollout shape, and confirm Longhorn parallel coordination. The Market worker and Longhorn lanes must not proceed on an assumed Underlay tag.

## Suggested next move (planning only, no worker)

Compile a bounded producer task file (pin + lock + census + release + validation oracles, explicit serial/parallel edges to Market `g05.070` and the Longhorn lane) and re-check the readiness rubric before any handoff or dispatch.
