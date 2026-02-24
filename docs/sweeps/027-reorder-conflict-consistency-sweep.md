# 027 - Reorder Conflict Consistency Sweep

Use this sweep to verify concurrency-safe reorder behavior is implemented consistently.

## Scope

All reorderable collections across Underlay-consuming apps.

## Step 1 - Reorder Eligibility

For each candidate list:

- Confirm ordering is canonical and persisted.
- Confirm it is not label/date/computed sort.
- Remove reorder UI if list is not inherently reorderable.

## Step 2 - API Conflict Contract

For each reorder endpoint:

- Conflict returns HTTP `409`.
- Error code is stable (`*.reorder_conflict`).
- Response includes context keys:
  - `added_ids`
  - `removed_ids`

## Step 3 - Underlay Pattern Usage

In each UI reorder surface:

- `ReorderableList` is used.
- `onsubmiterror` is wired.
- `extractReorderConflict(...)` is used to detect conflict payloads.
- `applyReorderConflict(...)` is used to merge conflict deltas.

## Step 4 - UX Behavior

On conflict:

- User remains in reorder mode.
- Pending order is updated (remove deleted + append added).
- Warning toast and inline error are shown.
- User must explicitly save again.

## Step 5 - Tests

Minimum coverage:

- Conflict parser handles `409` payload from transport errors.
- Merge logic removes/ appends correctly.
- Non-conflict errors remain unchanged.

## Step 6 - Docs and Roadmaps

- Link implementation to `docs/guides/186-reorder-conflict-recovery.md`.
- Update app roadmap entries to only include truly reorderable entities.
