<script lang="ts">
  import type { GroupedOptions, NightfireBlockOptionInput } from "./grouped-options";

  type TypeOption = Pick<NightfireBlockOptionInput, "type" | "label">;

  interface Props {
    value?: string;
    groupedOptions?: GroupedOptions[] | null;
    typeOptions?: TypeOption[];
    onChange: (event: Event) => void;
  }

  let {
    value,
    groupedOptions = null,
    typeOptions = [],
    onChange
  }: Props = $props();
</script>

<select
  {value}
  onchange={onChange}
  aria-label="Block type"
>
  {#if groupedOptions}
    {#each groupedOptions as group}
      {#if group.category}
        <optgroup label={group.label}>
          {#each group.options as opt}
            <option value={opt.type}>
              {opt.label}
            </option>
          {/each}
        </optgroup>
      {:else}
        {#each group.options as opt}
          <option value={opt.type}>
            {opt.label}
          </option>
        {/each}
      {/if}
    {/each}
  {:else}
    {#each typeOptions as opt}
      <option value={opt.type}>
        {opt.label}
      </option>
    {/each}
  {/if}
</select>

<style>
  select {
    padding: var(--underlay-field-padding-block) var(--underlay-field-padding-inline);
    border-radius: var(--underlay-radius-sm);
    border: none;
    background: var(--underlay-color-field-bg);
    color: var(--underlay-color-text);
    font-size: calc(1em * var(--underlay-font-scale-xs));
  }

  select:focus,
  select:focus-visible {
    outline: var(--underlay-focus-ring-width) solid var(--underlay-color-primary);
    outline-offset: var(--underlay-focus-ring-offset);
  }
</style>
