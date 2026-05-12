<script lang="ts">
  import { Grid, NavCard, PageHeader } from "@poodle/svelte";
  import type { SystemIndexCardConfig, TemplateSurface } from "./template.types";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string | null;
    backLabel?: string;
    cards: SystemIndexCardConfig[];
    beforeCards?: TemplateSurface;
    columns?: string;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
  }

  let {
    title = "System",
    subtitle,
    backHref = null,
    backLabel,
    cards,
    beforeCards,
    columns = "repeat(auto-fit, minmax(18rem, 1fr))",
    headerLevel = 2
  }: Props = $props();
</script>

<div class="underlay-system-index-page">
  <PageHeader
    {title}
    {subtitle}
    backHref={backHref ?? null}
    {backLabel}
    level={headerLevel}
  />

  {#if beforeCards}
    {@render beforeCards()}
  {/if}

  <Grid {columns} gap="md" asRole="navigation" ariaLabel={`${title} sections`}>
    {#each cards as card}
      {#if card.icon}
        <NavCard
          href={card.href}
          title={card.title}
          description={card.description}
        >
          <svelte:fragment slot="icon">
            <span
              class="underlay-system-index-page__icon"
              style:background={card.accent ?? "var(--poodle-color-accent-base)"}
            >
              {@render card.icon()}
            </span>
          </svelte:fragment>
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
