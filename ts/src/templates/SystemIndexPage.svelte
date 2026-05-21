<script lang="ts">
  import { Grid, NavCard, PageHeader } from "@poodle/svelte";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";
  import Calendar from "lucide-svelte/icons/calendar";
  import ClipboardList from "lucide-svelte/icons/clipboard-list";
  import Layers from "lucide-svelte/icons/layers";
  import { getBackButtonInfo } from "../patterns/navigation";
  import type { SystemIndexCardConfig, TemplateSurface } from "./template.types";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    backIsContextual?: boolean;
    resolveBackContext?: boolean;
    cards?: SystemIndexCardConfig[];
    extraCards?: SystemIndexCardConfig[];
    includeCoreCards?: boolean;
    beforeCards?: TemplateSurface;
    columns?: string;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
  }

  let {
    title = "System",
    subtitle,
    backHref = null,
    backLabel,
    backIsContextual = false,
    resolveBackContext = true,
    cards = undefined,
    extraCards = [],
    includeCoreCards = true,
    beforeCards,
    columns = "repeat(auto-fit, minmax(18rem, 1fr))",
    headerLevel = 2
  }: Props = $props();

  const coreCards: SystemIndexCardConfig[] = [
    {
      href: "/system/errors",
      title: "Error log",
      description: "View and investigate application errors and exceptions.",
      accent: "var(--admin-color-danger, #dc2626)",
      icon: errorsIconSnippet as never
    },
    {
      href: "/system/jobs",
      title: "Job queue",
      description: "Monitor background jobs, view status, and retry failed jobs.",
      accent: "var(--admin-color-primary, #8b5cf6)",
      icon: jobsIconSnippet as never
    },
    {
      href: "/system/scheduled-tasks",
      title: "Scheduled tasks",
      description: "Manage cron-scheduled maintenance tasks.",
      accent: "var(--admin-color-success, #10b981)",
      icon: tasksIconSnippet as never
    },
    {
      href: "/system/audit",
      title: "Audit log",
      description: "Track changes made to content and configuration.",
      accent: "var(--admin-color-info, #6366f1)",
      icon: auditIconSnippet as never
    }
  ];

  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);

  const visibleCards = $derived(cards ?? [...(includeCoreCards ? coreCards : []), ...extraCards]);

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

<div class="underlay-system-index-page">
  <PageHeader
    {title}
    {subtitle}
    sizeRole="prominent"
    backHref={resolvedBackInfo?.href ?? null}
    backLabel={resolvedBackInfo?.label ?? backLabel}
    backIsContextual={resolvedBackInfo?.contextual ?? false}
    level={headerLevel}
  />

  {#if beforeCards}
    {@render beforeCards()}
  {/if}

  <Grid {columns} gap="md" asRole="navigation" ariaLabel={`${title} sections`}>
    {#each visibleCards as card}
      {#if card.icon}
        {@const renderIcon = card.icon!}
        <NavCard
          href={card.href}
          title={card.title}
          description={card.description}
        >
          {#snippet icon()}
            <span
              class="underlay-system-index-page__icon"
              style:background={card.accent ?? "var(--poodle-color-accent-base)"}
            >
              {@render renderIcon()}
            </span>
          {/snippet}
        </NavCard>
      {:else}
        <NavCard
          href={card.href}
          title={card.title}
          description={card.description}
        />
      {/if}
    {/each}
  </Grid>
</div>

{#snippet errorsIconSnippet()}
  <AlertTriangle />
{/snippet}

{#snippet jobsIconSnippet()}
  <Layers />
{/snippet}

{#snippet tasksIconSnippet()}
  <Calendar />
{/snippet}

{#snippet auditIconSnippet()}
  <ClipboardList />
{/snippet}

<style>
  .underlay-system-index-page {
    display: grid;
    gap: 1rem;
  }

  .underlay-system-index-page__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: var(--poodle-radius-control);
    color: white;
  }

  .underlay-system-index-page__icon :global(svg) {
    width: 1rem;
    height: 1rem;
  }
</style>
