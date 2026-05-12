<script lang="ts">
  import { PageHeader } from "@poodle/svelte";
  import type { AdminDashboardSectionConfig, TemplateSurface } from "./template.types";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
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
    headerLevel = 2,
    beforeSections,
    sections = [],
    content
  }: Props = $props();
</script>

<div class="underlay-admin-dashboard-page">
  <PageHeader
    {title}
    {subtitle}
    backHref={backHref ?? null}
    {backLabel}
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
