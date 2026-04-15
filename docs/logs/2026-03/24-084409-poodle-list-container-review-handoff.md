---
title: Poodle ListContainer review handoff
status: active
owner: Platform
updated: 2026-03-24
tags: [coordination, handoff]
---

## Objective

Produce a narrow Poodle-side decision and implementation brief for a new Poodle-native `ListContainer` composite so Underlay can retire its current list-page shell without transplanting Underlay-specific dependencies.

## Scope

- Review Underlay `ListContainer` as a reference behavior surface only and decide the smallest useful Poodle-native contract for a list-page shell.
- Define how loading, error, empty state, header actions, filters, batch actions, content, and pagination placement should work when rebuilt on top of Poodle-owned primitives and composites.
- Decide explicitly how the existing Underlay `PaginatedList` should be folded into `ListContainer` behavior or retired in favor of `ListContainer`.
- Do not widen this batch into `DiagnosticsToolbar`, `OpsSection`, `SubmitButton`, `CopyActionsMenu`, `EntityActionsMenu`, auth flows, or generic admin/page-shell work outside the list-container contract.

## Inputs

- [~/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md](~/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md)
- [~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json)
- [~/Dev/projects/underlay/ts/src/components/ListContainer.svelte](~/Dev/projects/underlay/ts/src/components/ListContainer.svelte)
- [~/Dev/projects/underlay/ts/src/components/PaginatedList.svelte](~/Dev/projects/underlay/ts/src/components/PaginatedList.svelte)
- [~/Dev/projects/underlay-reference/acme-admin/src/routes/(app)/system/poodle-gap-review/+page.svelte](~/Dev/projects/underlay-reference/acme-admin/src/routes/(app)/system/poodle-gap-review/+page.svelte)
- [~/Dev/projects/poodle/packages/svelte/primitives/src/index.ts](~/Dev/projects/poodle/packages/svelte/primitives/src/index.ts)
- [~/Dev/projects/poodle/packages/svelte/composites/src/index.ts](~/Dev/projects/poodle/packages/svelte/composites/src/index.ts)
- [~/Dev/projects/poodle/docs/guides/svelte-developer-guide.md](~/Dev/projects/poodle/docs/guides/svelte-developer-guide.md)

## Constraints

- Follow the repo instructions in `AGENTS.md`.
- Keep edits aligned with the active Northstar vision, roadmap, and log flow.
- Do not widen scope beyond the listed tasks.
- Treat the current Underlay `ListContainer` as reference behavior only, not as an implementation to port directly.
- Build the target contract from Poodle-owned primitives and composites rather than depending on Underlay `PageHeader`, `PageLoading`, `FormError`, or other Underlay-owned internals.
- Treat `PaginatedList` as redundant with `ListContainer`; do not create a sibling Poodle surface that preserves both names.
- If part of the old behavior is better expressed as composition than as API, prefer the smaller Poodle contract.

## Deliverables

- [~/Dev/projects/poodle/docs/logs/2026-03/24-084410-poodle-list-container-review.md](~/Dev/projects/poodle/docs/logs/2026-03/24-084410-poodle-list-container-review.md)
- [~/Dev/projects/poodle/packages/svelte/composites/src/ListContainer.svelte](~/Dev/projects/poodle/packages/svelte/composites/src/ListContainer.svelte)
- [~/Dev/projects/poodle/packages/svelte/composites/src/index.ts](~/Dev/projects/poodle/packages/svelte/composites/src/index.ts)
- [~/Dev/projects/poodle/docs/guides/svelte-developer-guide.md](~/Dev/projects/poodle/docs/guides/svelte-developer-guide.md)
- [~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json)

## Acceptance Criteria

- The Poodle-side output makes an explicit decision that `ListContainer` belongs in Poodle as a new Poodle-native composite contract.
- The brief states explicitly that `PaginatedList` should be folded into `ListContainer` behavior or retired rather than migrated as a sibling surface.
- The proposed or implemented `ListContainer` composes only Poodle-owned primitives and composites for header, actions, filters, empty/loading/error states, content, batch actions, and pagination placement.
- The output distinguishes clearly between API-level responsibilities of `ListContainer` and concerns that should stay as caller-owned composition.
- The Underlay inventory is updated only to reflect the Poodle-side decision and not to reopen already-closed wrapper or composition questions.

## Notes

- Current context: this handoff advances Underlay roadmap `g01.042` in [~/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md](~/Dev/projects/underlay/docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md) after a live demo review in `acme-admin` cut the ambiguous workflow surfaces down to a single real gap: `ListContainer`.
- Decisions: the user explicitly wants `ListContainer` in Poodle, but not as a mechanical carry-over from Underlay. `DiagnosticsToolbar` and `OpsSection` were judged redundant and should collapse into composition; `SubmitButton`, `CopyActionsMenu`, and `EntityActionsMenu` should improve primitive ergonomics or stay app-level rather than becoming named Poodle composites.
- Watch-outs: Underlay `ListContainer` currently relies on Underlay `PageHeader`, `PageLoading`, and `FormError`; that dependency graph must not come across unchanged. The contract should be rebuilt around Poodle `PageHeader`, loading/empty/error surfaces, and pagination composition. Keep the API small enough that apps are not forced into a rigid list-page shape when they only need partial scaffolding.
- Next move: in the Poodle thread, compare the current Underlay `ListContainer` behavior against the `acme-admin` review route and draft the minimal Poodle-native contract before implementing or documenting it.

## Completion Protocol

1. Update the Poodle log and any touched guides or exports with the result of this batch.
2. If the Poodle-side decision changes the Underlay migration posture, update [~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json](~/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json) and keep the next task aligned with roadmap `g01.042`.
3. Record unresolved risks explicitly, especially any uncertainty about how much pagination and empty/loading/error state should live inside `ListContainer` versus outside it.
4. Leave one clear next task for the following thread.
