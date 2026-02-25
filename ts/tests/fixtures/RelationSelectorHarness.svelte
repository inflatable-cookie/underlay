<script lang="ts">
  import RelationSelector from "../../src/patterns/RelationSelector/RelationSelector.svelte";
  import type { SearchResult, SelectableRelation } from "../../src/patterns/RelationSelector/types.js";

  interface Props {
    mode?: "single" | "multi";
    label?: string;
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    error?: string;
    value?: string | null;
    values?: string[];
    initialSelection?: SelectableRelation | null;
    initialSelections?: SelectableRelation[];
    suggestionsItems?: SelectableRelation[];
    searchItems?: SelectableRelation[];
    allowCreate?: boolean;
    createLabel?: string;
    useCreateForm?: boolean;
    onchange?: (value: string | null) => void;
    onchangeMulti?: (values: string[]) => void;
    onCreate?: (item: SelectableRelation) => void;
  }

  let {
    mode = "single",
    label = "Select relation",
    placeholder = "Pick one",
    disabled = false,
    required = false,
    error = undefined,
    value = null,
    values = [],
    initialSelection = null,
    initialSelections = [],
    suggestionsItems = [],
    searchItems = [],
    allowCreate = false,
    createLabel = "Add new",
    useCreateForm = false,
    onchange,
    onchangeMulti,
    onCreate
  }: Props = $props();

  let localValue = $state<string | null>(null);
  let localValues = $state<string[]>([]);

  $effect(() => {
    localValue = value;
  });

  $effect(() => {
    localValues = values;
  });

  async function search(query: string): Promise<SearchResult<SelectableRelation>> {
    const lower = query.toLowerCase();
    const items = searchItems.filter((item) => item.label.toLowerCase().includes(lower));
    return { items, total: items.length };
  }

  async function suggestions(): Promise<SelectableRelation[]> {
    return suggestionsItems;
  }

  function handleChange(next: string | null) {
    localValue = next;
    onchange?.(next);
  }

  function handleChangeMulti(next: string[]) {
    localValues = next;
    onchangeMulti?.(next);
  }
</script>

{#snippet createFormSnippet(onSuccess: (item: SelectableRelation) => void, onCancel: () => void)}
  <div data-testid="selector-create-form">
    <button
      type="button"
      data-testid="selector-create-success"
      onclick={() => onSuccess({ id: "created", label: "Created relation" })}
    >
      Create
    </button>
    <button type="button" data-testid="selector-create-cancel" onclick={() => onCancel()}>
      Cancel
    </button>
  </div>
{/snippet}

<RelationSelector
  {mode}
  label={label}
  {placeholder}
  {disabled}
  {required}
  {error}
  bind:value={localValue}
  bind:values={localValues}
  {initialSelection}
  {initialSelections}
  {search}
  {suggestions}
  {allowCreate}
  {createLabel}
  onchange={handleChange}
  onchangeMulti={handleChangeMulti}
  {onCreate}
  createForm={useCreateForm ? createFormSnippet : undefined}
/>

<div data-testid="single-value">{localValue ?? "null"}</div>
<div data-testid="multi-values">{localValues.join(",")}</div>
