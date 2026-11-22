<script lang="ts">
  import { PageHeader } from "@inflatable-cookie/poodle-svelte";
  import { useTemplateBackInfo } from "./template-back-info.svelte";
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

  const getResolvedBackInfo = useTemplateBackInfo(() => ({
    href: backHref,
    label: backLabel,
    contextual: backIsContextual,
    resolveContext: resolveBackContext
  }));

  let resolvedBackInfo = $derived(getResolvedBackInfo());

  // Render the caller's href during SSR and the first client paint, then let the
  // contextual resolution take over once navigation context is readable.
  let backInfoHref = $derived(resolvedBackInfo?.href ?? backHref);
  let backInfoLabel = $derived(resolvedBackInfo?.label ?? backLabel);
  let backInfoIsContextual = $derived(resolvedBackInfo?.contextual ?? backIsContextual);
</script>

<div class="underlay-admin-dashboard-page">
  <PageHeader
    {title}
    {subtitle}
    sizeRole="prominent"
    backHref={backInfoHref}
    backLabel={backInfoLabel}
    backIsContextual={backInfoIsContextual}
    level={headerLevel}
  />

  {#if beforeSections}
    {@render beforeSections()}
  {/if}

  {#if sections.length > 0}
    {#each sections as section (section.id)}
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
