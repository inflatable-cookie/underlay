<script lang="ts">
  import type { Snippet } from "svelte";
  import { MetaBar, MetaItem, PageHeader } from "@poodle/svelte";

  interface MetaItemConfig {
    label: string;
    value: string | Snippet;
    separator?: boolean;
  }

  interface Props {
    title?: string | null;
    subtitle?: string | null;
    eyebrow?: string | null;
    meta?: MetaItemConfig[];
    header?: Snippet;
    children?: Snippet;
  }

  let {
    title = null,
    subtitle = null,
    eyebrow = null,
    meta: detailMeta = [],
    header,
    children
  }: Props = $props();

  const hasSubHeader = $derived(Boolean(title || subtitle || eyebrow || detailMeta.length > 0 || header));
</script>

<div class="underlay-entity-detail">
  {#if hasSubHeader}
    <PageHeader
      {title}
      {subtitle}
      {eyebrow}
      level={3}
      align="start"
      ariaLabel="Detail section header"
    >
      {#snippet meta()}
        {#if detailMeta.length > 0}
          <MetaBar ariaLabel="Detail metadata">
            {#each detailMeta as metaItem}
              <MetaItem label={metaItem.label} separator={metaItem.separator ?? true}>
                {#if typeof metaItem.value === "string"}
                  {metaItem.value}
                {:else}
                  {@render metaItem.value()}
                {/if}
              </MetaItem>
            {/each}
          </MetaBar>
        {/if}
      {/snippet}

      {#if header}
        {@render header()}
      {/if}
    </PageHeader>
  {/if}

  {#if children}
    <div class="underlay-entity-detail__modules">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .underlay-entity-detail {
    display: grid;
    gap: var(--poodle-space-stack-lg);
  }

  .underlay-entity-detail__modules {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--poodle-space-stack-lg);
    align-items: stretch;
  }

  @media (max-width: 64rem) {
    .underlay-entity-detail__modules {
      grid-template-columns: 1fr;
    }
  }
</style>
