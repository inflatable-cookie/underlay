# 186 - Reorder Conflict Recovery Contract

Use this guide when implementing drag-and-drop reorder flows that must tolerate concurrent edits.

## Scope

Applies to any list with canonical persisted ordering (`sort_order`, `weight`, etc.) where reorder writes are batch-submitted.

Do **not** apply this pattern to lists that are derived from labels, timestamps, or other computed sorts.

## Backend Contract

On reorder submit conflict, return `409 Conflict` with context payload:

```json
{
  "error": {
    "code": "learning.reorder_conflict",
    "message": "Items have changed since you started reordering."
  },
  "context": {
    "added_ids": ["new-id-1"],
    "removed_ids": ["deleted-id-1"]
  }
}
```

Rules:

- `added_ids`: IDs currently on server but missing from submitted IDs.
- `removed_ids`: IDs submitted by client but no longer present on server.
- Keep payload machine-readable and stable across endpoints.

## Underlay Helpers

From `@decodelabs/underlay/runtime/data`:

- `extractReorderConflict(error)`
- `applyReorderConflict(controller, conflict, latestItems)`

`applyReorderConflict(...)` behavior:

- removes `removed_ids` from controller pending list,
- appends `added_ids` (resolved from `latestItems`) to pending list,
- returns a resolution summary (added/removed/unresolved counts).

## Reorder Submission Hook

Use Poodle `EditableList` with an app-owned submit wrapper:

```svelte
<script lang="ts">
  import { EditableList } from "@inflatable-cookie/poodle-svelte";

  let submitError = $state<string | null>(null);

  async function handleReorderSubmit() {
    submitError = null;

    try {
      await controller.submit();
      await handleSuccess();
    } catch (error) {
      const conflict = extractReorderConflict(error);
      if (!conflict) {
        submitError = error instanceof Error ? error.message : String(error);
        return;
      }

      const latestItems = await loadLatestItems();
      applyReorderConflict(controller, conflict, latestItems);
      submitError = "List changed while reordering. Review updates and save again.";
    }
  }
</script>

<EditableList
  items={controller.pending}
  dirty={controller.isDirty}
  submitting={controller.isPending}
  errorMessage={submitError}
  onSubmit={handleReorderSubmit}
  onCancel={() => {
    submitError = null;
    controller.reset();
    exitReorderMode();
  }}
  on:reorder={(event) => controller.updatePending(event.detail.items)}
>
  ...
</EditableList>
```

Optional UX improvements:

- keep `longListThreshold` enabled (default `50`) so operators see a warning before reordering very large lists.
- use `windowSize` (for example `50`) to enable page-window reorder UI for large lists.

## UX Expectations

- Keep user in reorder mode after conflict.
- Never auto-submit after merge.
- Show clear warning toast + inline error.
- Require explicit second save.
- Preserve existing unsaved ordering where possible.

## Rollout Checklist

1. Confirm entity is truly reorderable (not derived sort).
2. Ensure backend endpoint returns `409` with `added_ids`/`removed_ids`.
3. Use Underlay helpers in your submit wrapper.
4. Add local warning copy indicating changes were merged.
5. Add tests for conflict parsing and merge behavior.
