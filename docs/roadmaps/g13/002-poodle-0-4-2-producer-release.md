# g13.002 - Poodle 0.4.2 Producer Release

Status: ready
Owner: repo maintainers
Created: 2026-09-14
Depends on: g13.001 complete; Underlay `v0.9.9` released at
`ac91af5a58deae56a9317877a3a6c8ceeb9d4586`; Poodle `0.4.2` published
Authority: operator request in the Queue Chatterbox thread on 2026-09-14
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/023-release-and-compatibility-rollout.md`, Poodle
`docs/release-notes/0.4.2.md`
UI classification: none

## Outcome

Underlay declares `@inflatable-cookie/poodle-svelte` exact `0.4.2`, the root
lock resolves `poodle-svelte` and transitive `poodle-core` at one public
`0.4.2` identity, and synchronous release metadata prepares truthful Underlay
`v0.9.10`. Existing Underlay behavior remains unchanged. No consumer or sibling
repository is edited.

## Current evidence

- Underlay `v0.9.9` is clean at
  `ac91af5a58deae56a9317877a3a6c8ceeb9d4586`. Root `package.json` and
  `bun.lock` declare Poodle `0.4.1`.
- Poodle `v0.4.2` resolves to
  `d2438aef7d34df31b958d172c9c162e83063d83c`. Its release note says the patch
  repairs the published Svelte Tabs cross-window bridge regression and has no
  breaking API change.
- Public `poodle-svelte@0.4.2` integrity is
  `sha512-TAWcw2JVmIXKwNaa3Q+UB+/fL0gmNeYsyCV6zNSgqH1+qu15W09MoqC08fzoB7+JiflUxC56LzpfdU9H9dt0bA==`.
  It depends on exact `poodle-core@0.4.2`, whose integrity is
  `sha512-4VJ2EejtpDEeVyxy4gI0QFqJd2BTrwf+ML3mB6RgEHz7seJ+7zWuy20Trh+o51bBd8xnVmu/UZ0Ovr3jbbtOTQ==`.
- The current lock resolves Svelte `5.56.8`, satisfying Poodle's
  `>=5.56.8 <6` peer. `marked` remains an optional Poodle peer; Underlay's
  existing direct `17.x` contract does not change in this lane.
- Caller census: 86 source/test files import Poodle. Superseded by the
  worker re-census at `d6435435`: 83 real source/test importers (88 raw
  `rg` matches minus five reference-only files — two JSDoc `@example`
  import lines, two workspace-shape package-name string lists, and one
  vendor fixture `package.json`; enumerated in the g13.002 execution log,
  confirmed by statement-level and comment-aware scans). Four use `Tabs` or
  `TabItem`: `LoginMethodTabs.svelte`, `TwoFactorStep.svelte`,
  `EntityDetailPage.svelte`, and `MediaPickerWorkflow.svelte`. None passes
  `crossWindowSourceBridge`. The upstream fix therefore repairs a latent
  producer defect without requiring an Underlay API change.
- No local or remote `v0.9.10` tag exists.

## Dispatch manifest

- One ordinary implementation worker, one PR, independent exact-head review.
- Economical general-capability routing. No frontier profile request.
- Mutable paths: root `package.json`, `bun.lock`, synchronous release files
  (`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`), this task, its handoff, and one
  execution log. Source/test changes are allowed only for proven compatibility
  fallout.
- Excluded: consumer repositories, Poodle, Market, Desktop, Longhorn,
  `.github/workflows/`, product behavior, new shims, dependency overrides,
  local package links, merge, and tag creation by the worker.
- After accepted review and Queue-owned merge, maintainers cut and validate
  `v0.9.10` from the merged release commit under contract 023.

## Work

1. Re-run the Poodle caller census and confirm the public `0.4.2` metadata and
   integrities before editing.
2. Change only root `dependencies.@inflatable-cookie/poodle-svelte` from exact
   `0.4.1` to exact `0.4.2`. Keep `poodle-core` transitive.
3. Regenerate `bun.lock` with the repository Bun. Accept only importer and
   Poodle-reachable lock changes. Run a frozen install and require no further
   diff.
4. Run the focused Svelte component tests covering the four Tabs callers and
   the normal TypeScript/component validation. Change source only if a real
   compatibility failure is reproduced.
5. Prepare patch release `0.9.10`: synchronize Cargo/package versions and lock,
   add the contract 023 changelog upgrade block, and state that consumers move
   their Underlay tag only. Do not edit consumers.
6. Run `effigy --json deps status bun`, `effigy validate`, `effigy rust:check`,
   `effigy rust:clippy`, `effigy qa`, and `git diff --check`.
7. Push one Queue branch and open one non-draft PR. Report exact head, lock
   identities/integrities, census, validation, and handoff path. Do not merge or
   tag from the worker.

## Acceptance and review oracle

- Root manifest names exact `poodle-svelte` `0.4.2`; no live manifest or lock
  record retains Poodle `0.4.1`.
- `bun.lock` resolves one Svelte/Core pair at exact `0.4.2` with the certified
  public integrities above. No path, link, override, or direct Core dependency
  appears.
- The four Tabs caller surfaces and the full existing Svelte component suite
  pass without a compatibility shim.
- Version files agree on `0.9.10`; changelog carries impact, consumer action,
  validation, and rollback evidence required by contract 023.
- Full QA passes. The PR changes Underlay only.

## Stop conditions

- Public metadata or integrity differs from the pinned evidence.
- Lock regeneration changes unrelated dependency identities.
- A caller needs a product-semantic change, shim, or breaking Underlay API.
- Any required release gate remains red.
- `v0.9.10` already exists or main moves to another release identity.

## Consumer Upgrade Impact

Impact: additive patch correction. Consumers do not change their own Poodle
pins in this producer task. A consumer that needs the corrected graph moves its
Underlay dependency to the validated `v0.9.10` tag through its own roadmap.

## Evidence

Record the caller census, manifest/lock before and after identities, public
integrities, frozen-install result, validation commands, reviewed PR head,
merge commit, and final `v0.9.10` tag/commit.

## Next task

After the validated tag exists, return its exact identity to Desktop g02.109.
No consumer edit or automatic follow-on dispatch belongs to this task.
