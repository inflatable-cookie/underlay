<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Title text - used when titleSnippet is not provided */
    title?: string;
    titleSuffix?: Snippet;
    /** Snippet for custom title content (takes precedence over title prop) */
    titleSnippet?: Snippet;
    subtitle?: string | null;
    trailing?: Snippet;
    actions?: Snippet<[{ trigger: Snippet; align: "start" | "center" | "end" }]>;
    actionsPlacement?: "media" | "media-overlay" | "trailing";
    children?: Snippet;
  }

  let {
    title = "",
    titleSuffix,
    titleSnippet,
    subtitle = null,
    trailing,
    actions,
    actionsPlacement = "media",
    children
  }: Props = $props();

  let hasActions = $derived(Boolean(actions));
</script>

{#snippet actionsTrigger()}
  <span class="underlay-list-card__dots-only" aria-hidden="true">⋯</span>
{/snippet}

<div class="underlay-list-card__body">
  <div class="underlay-list-card__title-row">
    <h3 class="underlay-list-card__title">
      {#if titleSnippet}
        {@render titleSnippet()}
      {:else}
        {title}
      {/if}
      {#if titleSuffix}{@render titleSuffix()}{/if}
    </h3>
    {#if trailing || (hasActions && actionsPlacement === "trailing")}
      <div class="underlay-list-card__title-actions">
        {#if trailing}
          {@render trailing()}
        {/if}
        {#if hasActions && actionsPlacement === "trailing"}
          <div class="underlay-list-card__actions">
            {@render actions?.({ trigger: actionsTrigger, align: "end" })}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  {#if subtitle}
    <p class="underlay-list-card__subtitle">{subtitle}</p>
  {/if}

  <div class="underlay-list-card__meta">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .underlay-list-card__dots-only {
    font-size: 1rem;
    font-weight: 700;
    line-height: 1;
  }

  .underlay-list-card__body {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .underlay-list-card__title-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--underlay-space-3, var(--underlay-space-3, 0.75rem));
    min-width: 0;
  }

  .underlay-list-card__title-actions {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-2, var(--underlay-space-2, 0.5rem));
    flex-shrink: 0;
  }

  .underlay-list-card__title {
    margin: 0;
    font-size: 1em;
    line-height: 1.2;
    font-weight: 650;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__subtitle {
    margin: 0;
    font-size: 0.9em;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__meta {
    display: flex;
    flex-direction: column;
    gap: calc(var(--underlay-space-1, var(--underlay-space-1, 0.25rem)) * 0.6);
    font-size: 0.82em;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    min-width: 0;
  }

  @media (max-width: 480px) {
    .underlay-list-card__title-actions :global(.underlay-pill) {
      font-size: 0.5em;
      padding: 0.2em 0.5em;
    }
  }
</style>
