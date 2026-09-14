# g13.001 - Poodle 0.4.1 Producer Release

Status: ready
Owner: repo maintainers
Created: 2026-09-14
Depends on: published Poodle `0.4.1` (Core and Svelte in the npm
publication set); parallel with `g11.002` and `g12.001`/Card 278, no
shared mutable paths
Authority: operator directive to move every project pinning Poodle to
published `0.4.1`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/023-release-and-compatibility-rollout.md`, Poodle
`docs/release-notes/0.4.1.md` plus the `0.4.0` breaking section of
Poodle `CHANGELOG.md`
UI classification: none

## Outcome

Underlay declares `@inflatable-cookie/poodle-svelte` exact `0.4.1`
from the public registry, its root `bun.lock` resolves one Poodle
identity (`poodle-svelte` `0.4.1` over transitive `poodle-core`
`0.4.1`), forced compatibility fallout is absorbed in owned files
only, and the validated `v0.9.9` tag with upgrade notes exists so
Market `g05.070` can pin an accepted, truthful producer release. No
manifest or lock in this repo names Poodle `0.3.0` afterwards.

## Current evidence

Committed pin scope (only live manifests; fixtures excluded):

- root `package.json` `dependencies.@inflatable-cookie/poodle-svelte`
  exact `0.3.0`.
- root `bun.lock` records `@inflatable-cookie/poodle-svelte@0.3.0`
  with transitive `@inflatable-cookie/poodle-core@0.3.0`.
- `Cargo.toml` workspace carries no Poodle edge; no Rust work is in
  scope. `ts/tests/fixtures/**/package.json` matches are test
  fixtures, not live manifests.

Caller census against the `0.4.0` breaking minor (worker re-verifies at
its base; any hit beyond this disposition stops the lane):

- Slider removal surface — absent. Zero matches in `ts/src`,
  `ts/tests`, and `templates/` for `appearance` on Slider,
  `variant="standard"`, `sliderFallbackText`,
  `rangeSliderFallbackText`, `formatVisibleRange`,
  `defaultVisibleRangeText`, `resolveRangeVisibleRange`,
  `assertHorizontalBlockAppearance`, or `SliderAppearance`. The
  `appearance="badge"` / `"subtle"` hits are the unrelated `Pill`
  prop. No file imports `Slider` or `RangeSlider` from Poodle.
- Markdown trust semantics — compatible by construction, proof owed.
  Exactly three `renderHtml` sites, all pre-sanitized:
  `ts/src/nightfire/markup/MarkdownEditor.svelte` (one) and
  `MarkdownEditorSurface.svelte` (two), each passing
  `renderSafeMarkdownPreview` (`sanitizeHtml(marked.parse(...))`)
  into `@inflatable-cookie/poodle-svelte/markdown`. No `htmlPolicy`
  usage anywhere. Underlay's own
  `ts/src/nightfire/markup/MarkdownRenderer.svelte` does not use
  Poodle's renderer (local `marked` + `sanitizeHtml`). The `0.4.0`
  safe-by-default path therefore sanitizes already-sanitized HTML;
  the worker proves preview output unchanged and adds no
  `htmlPolicy="trusted"`.
- CSS recipe hooks — absent. Zero matches for `data-appearance`,
  `--poodle-recipe-slider-*`, or `--poodle-recipe-range-slider-*`.
- New additive surface — unexposed. No imports of `CodeEditor`,
  `RichTextEditor`/`RichTextRenderer`, Poodle `MarkdownRenderer`,
  `Tabs` pinning, or `[data-variant="block"]` selectors. The new
  CodeMirror/TipTap packages ride as transitive install graph only.

Published Poodle `0.4.1` contract: `poodle-svelte` `0.4.1` depends on
`poodle-core` exact `0.4.1`; peers `svelte >=5.56.8 <6`, optional
`marked ^18.0.9`; `0.4.1` itself carries no breaking API change; Cargo
packages do not move. `poodle-core` stays transitive — the worker adds
no new root declaration for it (precedent: `0.3.0` adoption).

## Ready-state rubric

- [x] Objective is bounded: one pin, one lock, owned fallout only,
  one patch release.
- [x] Governing refs point at current canonical surfaces.
- [x] Scope, acceptance criteria, validation, evidence, and stop
  conditions are explicit below.
- [x] No unresolved planning gaps; the census above is the worker's
  falsifiable baseline, not an open question.

## Dispatch manifest

- **State:** ready. One implementation worker PR lane; release tag
  cut after merge as task closeout.
- **Placement:** new parallel generation `g13`, independent of
  `g11.002` and `g12.001`/Card 278. No serial edge to either; neither
  lane's oracle touches this pin.
- **Downstream:** Market `g05.070` is serial behind the produced tag
  and unblocks its own handoff; this lane does not edit Market.
- **Longhorn:** parallel producer lane, no edge in either direction.
  Do not wait on it, do not pin it, do not edit it.
- **Completion:** `v0.9.9` tag validated from a clean tree, upgrade
  notes shipped, tag plus commit plus package bytes reported to
  Market `g05.070`.
- **Owned mutable paths:** root `package.json`, root `bun.lock`,
  forced fallout only in owned files (the two Nightfire markdown
  wrappers; component-test Vite config/setup only if fallout proves),
  version-synchronous files (`Cargo.toml`
  `workspace.package.version`, `package.json` version,
  `Cargo.lock`, `CHANGELOG.md` plus the upgrade-note block), this
  task, its handoff, and one closeout log.
- **Worker:** ordinary implementation worker; no merge, no tag
  creation, no release mutation beyond reviewable version/changelog
  file edits inside the PR.
- **Excluded:** product semantics, consumer repositories, Poodle or
  Longhorn edits, historical docs (`docs/guides/020-*` `0.2.2`
  examples, `contracts/ui/*poodle*` snapshots — read by no live
  check), sibling `file:`/`path` overrides, `bun`/`npm` overrides,
  hand-edited locks, `.github/workflows/`, merge.
- **Escalation:** repo maintainers for any census falsification or
  release-gate failure.

## Work

1. Re-verify the census at the worker base: run the removed-surface
   greps (Slider tokens, `htmlPolicy`, `data-appearance`,
   recipe-hook prefixes) and record the disposition per item. Any hit
   beyond the baseline stops the lane for planning.
2. Pin `@inflatable-cookie/poodle-svelte` exact `0.4.1` in root
   `package.json` from the public registry with this repo's Bun
   (`1.3.14`). No ranges, no sibling paths, no overrides.
3. Regenerate root `bun.lock` from the repo root (`bun install`).
   Record before/after importer rows, resolved Poodle identities and
   integrities, and the Underlay git record. Accept only lock changes
   reachable from the Poodle update. Run a second frozen install and
   require a clean diff.
4. Absorb only forced compatibility fallout in owned files; prove
   markdown preview output unchanged. No product-semantic changes.
5. Prepare the additive patch release in the PR: bump
   `workspace.package.version` and `package.json` version to `0.9.9`
   in sync (release sync-files keep `Cargo.lock` aligned), add the
   `CHANGELOG.md` entry with the contract 023 upgrade-note block
   (impact `additive`, exact consumer actions, validation commands,
   links). If step 1 or 4 exposes a breaking caller impact, stop and
   return to planning instead of silently taking a minor.
6. Validate: `bun install` clean, frozen rerun no-diff,
   `effigy --json deps status bun` (no links), lock grep proves one
   Poodle identity and zero `0.3.0` Poodle records in manifests and
   lock, `check:release-version-sync`, `effigy validate`
   (health, `check`, `check:types`, unit and component tests),
   `rust:check` and `rust:clippy` per release gates,
   `git diff --check`.
7. Push one branch, open one non-draft PR against current `main`,
   report the clean exact head. No merge.
8. After accepted review and merge (orchestrator-owned): cut tag
   `v0.9.9`, validate from the tag, record the tag, commit, and
   verified package bytes, fill Delivery below, and hand the triple
   to Market `g05.070`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Published graph is exact | a manifest uses a range, sibling path, or `0.3.0` | root manifest names exact `0.4.1`; lock names one Poodle identity over core `0.4.1`; zero `0.3.0` Poodle records in manifests and lock |
| Core rides transitive | a new root `poodle-core` declaration appears | `package.json` diff touches only the `poodle-svelte` row |
| Markdown stays safe | preview renders unsanitized HTML or needs a trusted policy | wrapper tests plus preview-output proof; no `htmlPolicy="trusted"` added |
| No removed surface re-exported | a deleted Slider/Markdown name is referenced | census re-verification log with per-item disposition |
| Lock regeneration is scoped | unrelated identities or integrities churn | before/after lock report lists only importer fields and Poodle-reachable records |
| Install is reproducible | a frozen rerun mutates the lock | clean root install, then frozen install with no diff |
| Release is synchronous | version files disagree | `check:release-version-sync` green; tag `v0.9.9` validates from a clean tree |
| Consumers know what to do | the tag ships with no upgrade path | `CHANGELOG.md` upgrade-note block per contract 023 |
| No hidden Longhorn edge | the lane waits on or edits Longhorn | diff touches this repo only; Longhorn named nowhere as a prerequisite |

## Stop conditions

- the census finds removed-surface use beyond the baseline;
- Poodle `0.4.1` forces product-contract or semantic changes;
- the install wants unrelated dependency churn or registry
  resolution fails;
- validation requires editing a sibling checkout;
- any release gate (`version-sync`, `validate`, `clippy`,
  `rust:check`) stays red;
- a breaking caller impact appears: stop for planning (minor-bump
  decision) instead of widening the patch.

## Evidence

Per closeout record: census re-verification log, before/after
manifest rows, lock identities/integrities, frozen-rerun proof,
validation runs, PR link with reviewed exact head and merge commit,
tag plus commit plus package bytes, and material limits.

## Delivery

Pending execution.

## Next task

After the `v0.9.9` tag validates, hand its tag, commit, and package
bytes to Market `g05.070` (which unblocks its own handoff) and close
`g13` — no further task is queued in this generation.
