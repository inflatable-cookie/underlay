<script lang="ts">
  import RelationSelectorPopoverListSection from "../../src/patterns/RelationSelector/RelationSelectorPopoverListSection.svelte";
  import type { SelectableRelation } from "../../src/patterns/RelationSelector/types.js";

  interface Props {
    label: string;
    items: SelectableRelation[];
    focusedIndex: number;
    useRenderItem?: boolean;
    isSelected: (id: string) => boolean;
    onItemClick: (item: SelectableRelation) => void;
    onListKeyDown: (event: KeyboardEvent, items: SelectableRelation[]) => void;
    onListRef: (node: HTMLUListElement | null) => void;
  }

  let {
    label,
    items,
    focusedIndex,
    useRenderItem = false,
    isSelected,
    onItemClick,
    onListKeyDown,
    onListRef
  }: Props = $props();
</script>

{#snippet renderItemSnippet(item: SelectableRelation, selected: boolean)}
  <div data-testid={"custom-item-" + item.id}>
    {item.label}::{selected ? "selected" : "idle"}
  </div>
{/snippet}

<RelationSelectorPopoverListSection
  {label}
  {items}
  {focusedIndex}
  {isSelected}
  {onItemClick}
  {onListKeyDown}
  renderItem={useRenderItem ? renderItemSnippet : undefined}
  {onListRef}
/>
