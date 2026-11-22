<script lang="ts" generics="TItem, TSelectable extends SelectableRelation">
  import { Field as PoodleField } from "@inflatable-cookie/poodle-svelte";
  import { RelationSelector } from "../patterns";
  import { createLocalSearchFns } from "../patterns/local-search";
  import type {
    RelationSearchFn,
    RelationSuggestionsFn,
    SelectableRelation,
  } from "../patterns/RelationSelector/types";
  import type { SelectionHistory } from "../patterns/selection-history";

  interface Props {
    /** Field id (label + error wiring) and hidden input name. */
    id: string;
    name: string;
    label: string;
    value?: string | null;
    onChange?: (value: string) => void;
    /** Remote data source: search fn + optional suggestions fn. */
    search?: RelationSearchFn<TSelectable>;
    suggestions?: RelationSuggestionsFn<TSelectable>;
    /** Local data source: items searched client-side via createLocalSearchFns. */
    items?: () => TItem[];
    toSelectable?: (item: TItem) => TSelectable;
    getSearchText?: (item: TItem) => string[];
    initialSelection?: TSelectable | null;
    selectionHistory?: SelectionHistory;
    placeholder?: string;
    disabled?: boolean;
    required?: boolean;
    error?: string | null;
  }

  let {
    id,
    name,
    label,
    value = null,
    onChange,
    search,
    suggestions,
    items,
    toSelectable,
    getSearchText,
    initialSelection = null,
    selectionHistory,
    placeholder = "Select…",
    disabled = false,
    required = false,
    error = null,
  }: Props = $props();

  // Local search: built from items when no remote search is provided.
  const localFns = $derived.by(() => {
    if (search || !items || !toSelectable) return null;
    return createLocalSearchFns(items, {
      toSelectable,
      getSearchText: getSearchText ?? ((item) => [String(item)]),
    });
  });

  const resolvedSearch = $derived(search ?? localFns?.search);
  const resolvedSuggestions = $derived(suggestions ?? localFns?.suggest);

  // Field only publishes an error id through `describedBy` when it is told the
  // control is invalid, so mirror the error into the validation state.
  const validationState = $derived(error ? "invalid" : "none");
</script>

<PoodleField {id} {label} {error} {required} {validationState}>
  {#snippet control({ describedBy, validationState })}
    <input type="hidden" {name} value={value ?? ""} />
    {#if resolvedSearch}
      <!--
        RelationSelector renders its own composite trigger, so the field id and
        the error/hint wiring from Field are attached to the control group
        instead of the hidden input, which HTML constraint validation and
        assistive technology ignore.
      -->
      <div
        {id}
        class="underlay-entity-relation-field__control"
        role="group"
        aria-label={label}
        aria-describedby={describedBy ?? undefined}
        data-validation-state={validationState}
      >
        <RelationSelector
          label={placeholder}
          {value}
          {initialSelection}
          onChange={(val) => onChange?.(val ?? "")}
          search={resolvedSearch}
          suggestions={resolvedSuggestions}
          {selectionHistory}
          {placeholder}
          {disabled}
          {required}
        />
      </div>
    {/if}
  {/snippet}
</PoodleField>

<style>
  .underlay-entity-relation-field__control {
    display: block;
    width: 100%;
    min-width: 0;
  }
</style>
