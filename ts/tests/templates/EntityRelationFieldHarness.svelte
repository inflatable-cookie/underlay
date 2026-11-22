<script lang="ts">
  import EntityRelationField from "../../src/templates/EntityRelationField.svelte";
  import type { SelectableRelation } from "../../src/patterns/RelationSelector/types";

  interface Item {
    id: string;
    name: string;
  }

  interface Props {
    id?: string;
    label?: string;
    value?: string | null;
    required?: boolean;
    error?: string | null;
    disabled?: boolean;
  }

  let {
    id = "relation-field",
    label = "Related item",
    value = null,
    required = false,
    error = null,
    disabled = false
  }: Props = $props();

  const items = (): Item[] => [
    { id: "item-1", name: "First item" },
    { id: "item-2", name: "Second item" }
  ];

  const toSelectable = (item: Item): SelectableRelation => ({
    id: item.id,
    label: item.name
  });

  const getSearchText = (item: Item): string[] => [item.name];
</script>

<EntityRelationField
  {id}
  name={id}
  {label}
  {value}
  {items}
  {toSelectable}
  {getSearchText}
  {required}
  {error}
  {disabled}
  placeholder="Select related item"
/>
