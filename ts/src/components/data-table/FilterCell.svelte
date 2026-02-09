<script lang="ts">
  import type { DataTableColumn } from "../DataTable.svelte";
  import Select from "../Select.svelte";
  import DateInput from "../DateInput.svelte";
  import TextInput from "../TextInput.svelte";

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
    <DateInput
      {value}
      onchange={onChange}
    />
  {:else}
    <TextInput
      search
      placeholder={`Filter ${column.label.toLowerCase()}...`}
      {value}
      debounce={300}
      onchange={onChange}
    />
  {/if}
{/if}
