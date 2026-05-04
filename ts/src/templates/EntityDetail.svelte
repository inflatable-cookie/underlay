<script lang="ts">
  import type { Snippet } from "svelte";
  import { useAuthenticatedData } from "../runtime/auth";
  import {
    MetaBar,
    MetaItem,
    DetailSection,
    DetailItem,
    PageLoading,
    Callout,
    Card
  } from "@poodle/svelte";

  // --- Types ---

  interface MetaItemConfig {
    label: string;
    value: string | Snippet;
  }

  interface DetailSectionConfig {
    title: string;
    columns?: number;
    separated?: boolean;
    items: DetailItemConfig[];
  }

  interface DetailItemConfig {
    label: string;
    value: string | Snippet;
    emptyText?: string;
  }

  interface CustomSectionConfig {
    title: string;
    content: Snippet;
  }

  interface Props {
    /** Data loading function */
    dataLoader: (fetch: typeof window.fetch, token: string | null) => Promise<T | null>;
    
    /** Metadata items for MetaBar */
    meta?: MetaItemConfig[];
    
    /** Detail sections */
    sections?: DetailSectionConfig[];
    
    /** Custom sections (non-standard content) */
    customSections?: CustomSectionConfig[];
    
    /** Optional callback when data changes */
    onDataChange?: () => void;
  }

  type T = $$Generic;

  // --- Props ---

  let {
    dataLoader,
    meta = [],
    sections = [],
    customSections = [],
    onDataChange
  }: Props = $props();

  // --- Data loading ---

  const pageData = useAuthenticatedData<T | null>(
    async (fetch, token) => {
      return await dataLoader(fetch, token);
    },
    { defaultValue: null }
  );

  const item = $derived(pageData.data);
  const hasContent = $derived(sections.length > 0 || customSections.length > 0);
</script>

{#if pageData.loading}
  <PageLoading presentation="inline" message="Loading..." />
{:else if pageData.error}
  <Callout tone="danger" message={pageData.error} announceMode="polite" />
{:else if item}
  <div class="entity-detail">
    {#if meta.length > 0}
      <MetaBar ariaLabel="Metadata">
        {#each meta as metaItem}
          <MetaItem label={metaItem.label}>
            {#if typeof metaItem.value === "string"}
              {metaItem.value}
            {:else}
              {@render metaItem.value()}
            {/if}
          </MetaItem>
        {/each}
      </MetaBar>
    {/if}

    {#if hasContent}
      <Card>
        <div class="detail-content">
          {#each sections as section}
            <DetailSection
              title={section.title}
              columns={section.columns ?? 2}
              separated={section.separated ?? true}
            >
              {#each section.items as detailItem}
                <DetailItem
                  label={detailItem.label}
                  value={typeof detailItem.value === "string" ? detailItem.value : undefined}
                  emptyText={detailItem.emptyText}
                >
                  {#if typeof detailItem.value !== "string"}
                    {@render detailItem.value()}
                  {/if}
                </DetailItem>
              {/each}
            </DetailSection>
          {/each}

          {#each customSections as customSection}
            <DetailSection title={customSection.title}>
              {@render customSection.content()}
            </DetailSection>
          {/each}
        </div>
      </Card>
    {/if}
  </div>
{/if}

<style>
  .entity-detail {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .detail-content {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
