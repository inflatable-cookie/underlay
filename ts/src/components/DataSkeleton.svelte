<script lang="ts" module>
  export type {
    DataSkeletonType,
    DataSkeletonListPattern,
    DataSkeletonGridPattern,
    DataSkeletonDetailSection,
    DataSkeletonPreset
  } from "./data-skeleton";
</script>

<script lang="ts">
  import Skeleton from "./Skeleton.svelte";
  import {
    getDataSkeletonPreset,
    normaliseDataSkeletonSections,
    type DataSkeletonDetailSection,
    type DataSkeletonGridPattern,
    type DataSkeletonListPattern,
    type DataSkeletonType
  } from "./data-skeleton";

  interface Props {
    type?: DataSkeletonType;
    pattern?: string | null;
    count?: number;
    columns?: number;
    rows?: number;
    header?: boolean;
    sections?: DataSkeletonDetailSection[] | null;
    animate?: boolean;
    class?: string;
  }

  let {
    type = "list",
    pattern = null,
    count = 3,
    columns = 3,
    rows = 5,
    header = true,
    sections = null,
    animate = true,
    class: className = ""
  }: Props = $props();

  const preset = $derived(pattern ? getDataSkeletonPreset(pattern) : null);
  const effectiveType = $derived(preset?.type ?? type);
  const effectivePattern = $derived(preset?.pattern ?? (preset ? null : pattern));
  const effectiveCount = $derived(preset?.count ?? count);
  const effectiveColumns = $derived(preset?.columns ?? columns);
  const effectiveRows = $derived(preset?.rows ?? rows);
  const effectiveHeader = $derived(preset?.header ?? header);
  const effectiveSections = $derived(normaliseDataSkeletonSections(preset?.sections ?? sections));

  const listPattern = $derived(
    (effectivePattern === "avatar-text" || effectivePattern === "card"
      ? effectivePattern
      : "default") as DataSkeletonListPattern
  );
  const gridPattern = $derived(
    (effectivePattern === "product-card" ? effectivePattern : "default") as DataSkeletonGridPattern
  );
  const listItems = $derived(Array.from({ length: Math.max(1, effectiveCount) }, (_, index) => index));
  const tableColumns = $derived(Array.from({ length: Math.max(1, effectiveColumns) }, (_, index) => index));
  const tableRows = $derived(Array.from({ length: Math.max(1, effectiveRows) }, (_, index) => index));
</script>

<div
  class={`underlay-data-skeleton underlay-data-skeleton--${effectiveType} ${className}`}
  data-skeleton-type={effectiveType}
  data-skeleton-pattern={effectivePattern ?? ""}
>
  {#if effectiveType === "list"}
    <div class="underlay-data-skeleton__stack">
      {#each listItems as item (item)}
        {#if listPattern === "avatar-text"}
          <div class="underlay-data-skeleton__list-item underlay-data-skeleton__list-item--avatar" data-testid="data-skeleton-item">
            <Skeleton variant="avatar" {animate} />
            <div class="underlay-data-skeleton__body">
              <Skeleton variant="title" width="40%" {animate} />
              <Skeleton variant="text" lines={2} {animate} />
            </div>
          </div>
        {:else if listPattern === "card"}
          <div data-testid="data-skeleton-item">
            <Skeleton variant="card" {animate} class="underlay-data-skeleton__card-item">
              <div class="underlay-data-skeleton__body">
                <Skeleton variant="title" width="45%" {animate} />
                <Skeleton variant="text" lines={2} {animate} />
                <Skeleton variant="button" width="7rem" {animate} />
              </div>
            </Skeleton>
          </div>
        {:else}
          <div class="underlay-data-skeleton__list-item" data-testid="data-skeleton-item">
            <div class="underlay-data-skeleton__body">
              <Skeleton variant="title" width="38%" {animate} />
              <Skeleton variant="text" lines={2} {animate} />
            </div>
          </div>
        {/if}
      {/each}
    </div>
  {:else if effectiveType === "grid"}
    <div
      class="underlay-data-skeleton__grid"
      style:grid-template-columns={`repeat(${Math.max(1, effectiveColumns)}, minmax(0, 1fr))`}
      data-grid-columns={Math.max(1, effectiveColumns)}
    >
      {#each listItems as item (item)}
        <div data-testid="data-skeleton-item">
          <Skeleton variant="card" {animate} class="underlay-data-skeleton__grid-card">
            {#if gridPattern === "product-card"}
              <Skeleton variant="custom" height="10rem" radius="0.75rem" {animate} />
              <Skeleton variant="title" width="58%" {animate} />
              <Skeleton variant="text" lines={2} {animate} />
              <Skeleton variant="button" width="5rem" {animate} />
            {:else}
              <Skeleton variant="title" width="52%" {animate} />
              <Skeleton variant="text" lines={3} {animate} />
              <Skeleton variant="button" width="6rem" {animate} />
            {/if}
          </Skeleton>
        </div>
      {/each}
    </div>
  {:else if effectiveType === "table"}
    <div class="underlay-data-skeleton__table" role="presentation" aria-hidden="true">
      {#if effectiveHeader}
        <div class="underlay-data-skeleton__table-row underlay-data-skeleton__table-row--header">
          {#each tableColumns as column (column)}
            <div class="underlay-data-skeleton__table-cell">
              <Skeleton variant="button" width="70%" height="0.9rem" radius="0.25rem" {animate} />
            </div>
          {/each}
        </div>
      {/if}
      {#each tableRows as row (row)}
        <div class="underlay-data-skeleton__table-row" data-testid="data-skeleton-item">
          {#each tableColumns as column (column)}
            <div class="underlay-data-skeleton__table-cell">
              <Skeleton variant="text" width={column === tableColumns.length - 1 ? "55%" : "85%"} {animate} />
            </div>
          {/each}
        </div>
      {/each}
    </div>
  {:else}
    <div class="underlay-data-skeleton__detail">
      {#each effectiveSections as section (section)}
        <section class={`underlay-data-skeleton__detail-section underlay-data-skeleton__detail-section--${section}`} data-testid={`data-skeleton-section-${section}`}>
          {#if section === "header"}
            <Skeleton variant="title" width="32%" {animate} />
            <Skeleton variant="text" lines={2} {animate} />
          {:else if section === "stats"}
            <div class="underlay-data-skeleton__stats">
              {#each Array.from({ length: 3 }, (_, index) => index) as stat (stat)}
                <Skeleton variant="card" {animate} class="underlay-data-skeleton__stat-card">
                  <Skeleton variant="text" width="45%" {animate} />
                  <Skeleton variant="title" width="65%" {animate} />
                </Skeleton>
              {/each}
            </div>
          {:else if section === "description"}
            <Skeleton variant="text" lines={4} {animate} />
          {:else if section === "actions"}
            <div class="underlay-data-skeleton__actions">
              <Skeleton variant="button" width="7rem" {animate} />
              <Skeleton variant="button" width="5rem" {animate} />
            </div>
          {/if}
        </section>
      {/each}
    </div>
  {/if}
</div>

<style>
  .underlay-data-skeleton {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-data-skeleton__stack,
  .underlay-data-skeleton__detail {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-data-skeleton__list-item {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
    padding: var(--underlay-space-3, 0.75rem) 0;
    border-bottom: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.16));
  }

  .underlay-data-skeleton__list-item--avatar {
    grid-template-columns: auto 1fr;
    align-items: start;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-data-skeleton__body {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-data-skeleton__card-item,
  .underlay-data-skeleton__grid-card,
  .underlay-data-skeleton__stat-card {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-data-skeleton__grid {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-data-skeleton__table {
    display: grid;
    gap: 0;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.18));
    border-radius: var(--underlay-radius-md, 0.5rem);
    overflow: hidden;
  }

  .underlay-data-skeleton__table-row {
    display: grid;
    grid-template-columns: repeat(var(--underlay-data-skeleton-columns, 1), minmax(0, 1fr));
  }

  .underlay-data-skeleton__table-row--header {
    background: rgba(148, 163, 184, 0.06);
  }

  .underlay-data-skeleton__table-cell {
    padding: var(--underlay-space-3, 0.75rem);
    border-bottom: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.14));
  }

  .underlay-data-skeleton__table-row:last-child .underlay-data-skeleton__table-cell {
    border-bottom: none;
  }

  .underlay-data-skeleton__detail-section {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-data-skeleton__stats {
    display: grid;
    gap: var(--underlay-space-3, 0.75rem);
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }

  .underlay-data-skeleton__actions {
    display: flex;
    gap: var(--underlay-space-2, 0.5rem);
    flex-wrap: wrap;
  }

  @media (max-width: 700px) {
    .underlay-data-skeleton__grid {
      grid-template-columns: 1fr !important;
    }

    .underlay-data-skeleton__stats {
      grid-template-columns: 1fr;
    }
  }
</style>
