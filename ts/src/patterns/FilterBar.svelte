<script lang="ts">
  import type { Snippet } from "svelte";
  import ChevronUp from "lucide-svelte/icons/chevron-up";

  interface Props {
    title?: string;
    startCollapsed?: boolean;
    children?: Snippet;
  }

  let { title = "Filters", startCollapsed = false, children }: Props = $props();

  let collapsed = $state(true);
  let hasInteracted = $state(false);

  $effect(() => {
    if (!hasInteracted) collapsed = startCollapsed;
  });

  function toggle() {
    hasInteracted = true;
    collapsed = !collapsed;
  }
</script>

<section class="underlay-filter-bar" class:underlay-filter-bar--expanded={!collapsed}>
  {#if collapsed}
    <header class="underlay-filter-bar__header">
      <h2 class="underlay-filter-bar__title">{title}</h2>
      <button type="button" class="underlay-filter-bar__toggle" onclick={toggle}>
        Show filters
      </button>
    </header>
  {:else}
    <button
      type="button"
      class="underlay-filter-bar__close"
      onclick={toggle}
      aria-label="Collapse filters"
    >
      <ChevronUp size="1em" strokeWidth={2.5} />
    </button>
    <div class="underlay-filter-bar__body">
      {@render children?.()}
    </div>
  {/if}
</section>

<style>
  .underlay-filter-bar {
    position: relative;
    border-radius: var(--underlay-radius-lg, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background: var(--underlay-color-surface-muted, rgba(15, 23, 42, 0.65));
    padding: var(--underlay-space-3, 0.75rem);
    margin-bottom: var(--underlay-space-4, 1rem);
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-filter-bar--expanded {
    padding: var(--underlay-space-3, 0.75rem);
  }

  .underlay-filter-bar__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-filter-bar__title {
    margin: 0;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .underlay-filter-bar__toggle {
    border-radius: var(--underlay-radius-pill, 999px);
    border: 1px solid var(--underlay-color-border-strong, rgba(148, 163, 184, 0.5));
    background: var(--underlay-color-button-neutral-bg, rgba(15, 23, 42, 0.3));
    color: var(--underlay-color-text, #e5e7eb);
    padding: 0.2rem var(--underlay-space-3, 0.75rem);
    font-size: var(--underlay-font-size-xs, 0.75rem);
    cursor: pointer;
  }

  .underlay-filter-bar__toggle:hover {
    background: var(
      --underlay-color-button-neutral-hover,
      var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.12))
    );
  }

  .underlay-filter-bar__close {
    position: absolute;
    top: var(--underlay-space-2, 0.5rem);
    right: var(--underlay-space-2, 0.5rem);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    background: transparent;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    cursor: pointer;
  }

  .underlay-filter-bar__close:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.12));
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-filter-bar__body {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-filter-bar__body :global(.underlay-field) {
    min-width: 12rem;
  }
</style>
