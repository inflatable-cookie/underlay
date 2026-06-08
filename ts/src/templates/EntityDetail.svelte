<script lang="ts">
  import type { Snippet } from "svelte";
  import { MetaBar, PageHeader } from "@poodle/svelte";
  import { default as EntityMetaItem } from "./EntityMetaItem.svelte";
  import type { DetailMetaItemConfig } from "./template.types";

  interface Props {
    title?: string | null;
    subtitle?: string | null;
    eyebrow?: string | null;
    meta?: DetailMetaItemConfig[];
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
              <EntityMetaItem item={metaItem} />
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
    container-type: inline-size;
  }

  .underlay-entity-detail__modules {
    --underlay-entity-detail-modules-gap: var(--poodle-space-stack-lg);
    --underlay-entity-detail-modules-min-width: 22rem;

    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        min(
          100%,
          max(
            var(--underlay-entity-detail-modules-min-width),
            calc((100% - var(--underlay-entity-detail-modules-gap)) / 2)
          )
        ),
        1fr
      )
    );
    gap: var(--underlay-entity-detail-modules-gap);
    align-items: stretch;
  }

  @container (max-width: 46rem) {
    .underlay-entity-detail__modules {
      grid-template-columns: 1fr;
    }
  }
</style>
