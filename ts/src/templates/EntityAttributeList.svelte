<script lang="ts">
  import type { Snippet } from "svelte";
  import { DetailItem, DetailSection } from "@inflatable-cookie/poodle-svelte";
  import { default as EntityDetailModule } from "./EntityDetailModule.svelte";
  import type { DetailItemConfig } from "./template.types";

  interface Props {
    title?: string | null;
    description?: string;
    columns?: 1 | 2 | 3;
    separated?: boolean;
    span?: "half" | "full";
    items: DetailItemConfig[];
  }

  let {
    title = null,
    description,
    columns = 2,
    separated = false,
    span = "half",
    items
  }: Props = $props();
</script>

<EntityDetailModule {span}>
  <DetailSection
    title={title}
    description={description ?? null}
    {columns}
    {separated}
  >
    {#each items as item}
      <DetailItem
        label={item.label}
        description={item.description ?? null}
        value={typeof item.value === "string" ? item.value : undefined}
        emptyText={item.emptyText}
        truncateValue={item.truncateValue ?? false}
        layout={item.layout ?? "stacked"}
        presentation={item.presentation ?? "surface"}
        span={item.span ?? null}
      >
        {#if typeof item.value !== "string"}
          {@render item.value()}
        {/if}
      </DetailItem>
    {/each}
  </DetailSection>
</EntityDetailModule>
