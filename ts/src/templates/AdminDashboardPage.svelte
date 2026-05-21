<script lang="ts">
  import { PageHeader } from "@poodle/svelte";
  import { getBackButtonInfo } from "../patterns/navigation";
  import type { AdminDashboardSectionConfig, TemplateSurface } from "./template.types";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    resolveBackContext?: boolean;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    beforeSections?: TemplateSurface;
    sections?: AdminDashboardSectionConfig[];
    content?: TemplateSurface;
  }

  let {
    title = "Dashboard",
    subtitle,
    backHref = null,
    backLabel,
    backIsContextual = false,
    resolveBackContext = true,
    headerLevel = 2,
    beforeSections,
    sections = [],
    content
  }: Props = $props();

  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);

  $effect(() => {
    if (!backHref) {
      resolvedBackInfo = null;
      return;
    }

    const fallbackLabel = backLabel ?? "Back";
    if (!resolveBackContext) {
      resolvedBackInfo = {
        href: backHref,
        label: fallbackLabel,
        contextual: backIsContextual
      };
      return;
    }

    const contextualBackInfo = getBackButtonInfo(fallbackLabel, backHref);
    resolvedBackInfo = {
      href: contextualBackInfo.href,
      label: contextualBackInfo.label,
      contextual: Boolean(contextualBackInfo.isContextual || backIsContextual)
    };
  });
</script>

<div class="underlay-admin-dashboard-page">
  <PageHeader
    {title}
    {subtitle}
    sizeRole="prominent"
    backHref={resolvedBackInfo?.href ?? null}
    backLabel={resolvedBackInfo?.label ?? backLabel}
    backIsContextual={resolvedBackInfo?.contextual ?? false}
    level={headerLevel}
  />

  {#if beforeSections}
    {@render beforeSections()}
  {/if}

  {#if sections.length > 0}
    {#each sections as section}
      <section class="underlay-admin-dashboard-page__section" data-section={section.id}>
        {#if section.title}
          <h2 class="underlay-admin-dashboard-page__section-title">{section.title}</h2>
        {/if}
        {@render section.content()}
      </section>
    {/each}
  {:else if content}
    {@render content()}
  {/if}
</div>

<style>
  .underlay-admin-dashboard-page {
    display: grid;
    gap: 1rem;
  }

  .underlay-admin-dashboard-page__section {
    display: grid;
    gap: 1rem;
  }

  .underlay-admin-dashboard-page__section-title {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
  }
</style>
