<script lang="ts">
  import RelationPickerList from "../../src/patterns/relation-picker/RelationPickerList.svelte";
  import type { PickableItem, PickerSection } from "../../src/patterns/relation-picker-types.js";

  interface Props {
    displaySections: PickerSection[] | null;
    displayItems: PickableItem[];
    sectionLabel?: string;
    selectedIds: string[];
    focusedIndex: number;
    useRenderItem?: boolean;
    onItemClick: (item: PickableItem) => void;
    onListKeyDown: (event: KeyboardEvent) => void;
    getGlobalIndex: (sectionIndex: number, itemIndex: number) => number;
    onListRef: (node: HTMLUListElement | null) => void;
  }

  let {
    displaySections,
    displayItems,
    sectionLabel,
    selectedIds,
    focusedIndex,
    useRenderItem = false,
    onItemClick,
    onListKeyDown,
    getGlobalIndex,
    onListRef
  }: Props = $props();
</script>

{#snippet customItem(item, selected)}
  <div data-testid={`custom-item-${item.id}`}>{item.label}::{selected ? "selected" : "plain"}</div>
{/snippet}

<RelationPickerList
  {displaySections}
  {displayItems}
  {sectionLabel}
  {selectedIds}
  {focusedIndex}
  {onItemClick}
  {onListKeyDown}
  {getGlobalIndex}
  {onListRef}
  renderItem={useRenderItem ? customItem : undefined}
/>
