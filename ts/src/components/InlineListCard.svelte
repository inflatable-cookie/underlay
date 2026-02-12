<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Card title displayed in uppercase */
    title: string;
    /** Message shown when there are no items */
    emptyMessage?: string;
    /** Whether the list has items (controls empty state display) */
    hasItems?: boolean;
    /** Optional action button (typically "Add" button) */
    action?: Snippet;
    /** List items to render */
    children?: Snippet;
  }

  let {
    title,
    emptyMessage = "No items.",
    hasItems = true,
    action,
    children
  }: Props = $props();
</script>

<div class="underlay-inline-list-card">
  <div class="underlay-inline-list-card__header">
    <h4 class="underlay-inline-list-card__title">{title}</h4>
    {#if action}
      <div class="underlay-inline-list-card__action">
        {@render action()}
      </div>
    {/if}
  </div>

  <div class="underlay-inline-list-card__body">
    {#if hasItems && children}
      <ul class="underlay-inline-list-card__list">
        {@render children()}
      </ul>
    {:else}
      <p class="underlay-inline-list-card__empty">{emptyMessage}</p>
    {/if}
  </div>
</div>

<style>
  .underlay-inline-list-card {
    max-width: 65ch;
    min-width: 0;
    overflow: hidden;
    padding: 1rem 1.25rem;
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-radius: var(--underlay-radius-lg, 0.75rem);
  }

  .underlay-inline-list-card__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 0.75rem;
  }

  .underlay-inline-list-card__title {
    margin: 0;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .underlay-inline-list-card__action {
    flex-shrink: 0;
  }

  .underlay-inline-list-card__body {
    margin: 0;
  }

  .underlay-inline-list-card__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .underlay-inline-list-card__empty {
    margin: 0;
    font-size: 0.9em;
    font-style: italic;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
  }
</style>
