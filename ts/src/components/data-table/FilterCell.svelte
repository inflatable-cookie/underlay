<script lang="ts">
  import { SearchField, TextInput } from "@poodle/svelte-primitives";
  import type { DataTableColumn } from "../DataTable.svelte";
  import Select from "../Select.svelte";

  interface Props {
    column: DataTableColumn<any>;
    value: string;
    onChange: (value: string) => void;
  }

  let {
    column,
    value,
    onChange
  }: Props = $props();

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    return () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
    };
  });

  function handleSearchValueChange(nextValue: string): void {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(() => {
      debounceTimer = null;
      onChange(nextValue);
    }, 300);
  }
</script>

{#if column.filterable}
  {#if column.filterType === "select" && column.filterOptions}
    <Select
      {value}
      onchange={onChange}
      placeholder="All"
      items={[
        { value: "", label: "All" },
        ...column.filterOptions.map((opt) =>
          typeof opt === "string" ? { value: opt, label: opt } : opt
        )
      ]}
    />
  {:else if column.filterType === "date"}
    <TextInput
      id={`filter-${column.key ?? column.label.toLowerCase().replace(/\s+/g, "-")}`}
      type="date"
      {value}
      on:valueChange={(event) => onChange(event.detail.value)}
    />
  {:else}
    <SearchField
      id={`filter-${column.key ?? column.label.toLowerCase().replace(/\s+/g, "-")}`}
      placeholder={`Filter ${column.label.toLowerCase()}...`}
      {value}
      on:valueChange={(event) => handleSearchValueChange(event.detail.value)}
    />
  {/if}
{/if}
