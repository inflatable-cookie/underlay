# Entity List Card

Status: active

`EntityListCard` is the shared card composition helper for card-mode admin
collections.

Use it when:

- an `EntityListPage` or `EntityList` surface renders cards
- the card needs the normal Underlay selection and reorder posture
- the card should expose a standard context-action menu

Apps still own entity-specific text, counters, badges, navigation, and
callbacks. `EntityListCard` owns the repeated shell.

Reference-grade admin apps should treat `EntityListCard` as the default card
surface for real browse/manage collections. Do not hand-roll repeated raw
`ListCard` compositions for those collections unless the card is a real
workflow exception or not an admin collection at all.

## Usage

```svelte
<script lang="ts">
  import { EntityListCard } from "@decodelabs/underlay/templates";

  let { project, selectionMode, reorderMode, selected = false } = $props();

  const menuItems = [
    { value: "edit", label: "Edit" },
    { value: "archive", label: "Archive" }
  ];

  function handleContextAction(value: string) {
    if (value === "edit") goto(`/projects/${project.id}/edit`);
    if (value === "archive") archiveProject(project.id);
  }
</script>

<EntityListCard
  title={project.name}
  subtitle={project.clientName}
  meta={project.code}
  href={`/projects/${project.id}`}
  leadingIcon="briefcase-business"
  badges={[
    { label: project.status, tone: project.live ? "success" : "neutral" }
  ]}
  selectionMode={selectionMode}
  reorderMode={reorderMode}
  selected={selected}
  contextMenuItems={selectionMode || reorderMode ? [] : menuItems}
  contextMenuTrigger="leading"
  contextMenuAriaLabel="Project actions"
  onContextAction={handleContextAction}
/>
```

## Defaults

`EntityListCard` defaults to the larger rounded-square leading shape. That is
the normal reference posture for both icon-led cards and media-thumb cards.

Override `leadingShape="circle"` only when the entity really wants circular
identity treatment.

## Context menu trigger

`contextMenuTrigger` controls how the card exposes context actions:

- `"context"`: right-click menu on the card
- `"leading"`: click the leading icon, image, or custom leading content

Reference posture:

- prefer `"leading"` for normal admin card actions
- use `"context"` only when the consumer explicitly wants right-click behavior

When `contextMenuTrigger="leading"`:

- the leading visual becomes the menu trigger
- the rest of the card still behaves like the normal card click target
- selection and reorder mode can still suppress the menu by passing no menu
  items

## App policy

`EntityListCard` is the normal shared card shell for reusable list wrappers.

- define the card once in `src/lib/cards/*`
- use that card from the app-local `src/lib/lists/*` wrapper
- do not build repeated raw `ListCard` compositions directly in routes
- do not keep raw `ListCard` in app-local collection cards when
  `EntityListCard` already fits
- keep raw `ListCard` only for explicit exceptions:
  - non-admin surfaces
  - one-off workflow cards
  - subordinate embeds where the `EntityListCard` posture would be artificial
