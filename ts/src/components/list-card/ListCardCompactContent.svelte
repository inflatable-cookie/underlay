<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Title text - used when titleSnippet is not provided */
    title?: string;
    showDragHandle?: boolean;
    media?: Snippet;
    /** Snippet for custom title content (takes precedence over title prop) */
    titleSnippet?: Snippet;
  }

  let { title = "", showDragHandle = false, media, titleSnippet }: Props = $props();
</script>

{#if showDragHandle}
  <div class="underlay-list-card__drag-handle" aria-label="Drag to reorder">
    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
      <circle cx="5" cy="3" r="1.5" />
      <circle cx="11" cy="3" r="1.5" />
      <circle cx="5" cy="8" r="1.5" />
      <circle cx="11" cy="8" r="1.5" />
      <circle cx="5" cy="13" r="1.5" />
      <circle cx="11" cy="13" r="1.5" />
    </svg>
  </div>
{/if}
<div class="underlay-list-card__media underlay-list-card__media--compact">
  {#if media}
    {@render media()}
  {/if}
</div>
<span class="underlay-list-card__title underlay-list-card__title--compact">
  {#if titleSnippet}
    {@render titleSnippet()}
  {:else}
    {title}
  {/if}
</span>

<style>
  .underlay-list-card__media {
    width: var(--_underlay-list-card-media-size);
    height: var(--_underlay-list-card-media-size);
    border-radius: var(--underlay-radius-md, var(--underlay-radius-md, 0.75rem));
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--underlay-list-card-accent);
    color: var(--underlay-color-on-primary, #fff);
    overflow: hidden;
  }

  .underlay-list-card__media :global(svg) {
    max-width: 60%;
    max-height: 60%;
  }

  .underlay-list-card__media :global(.media-thumbnail) {
    width: 100%;
    height: 100%;
  }

  .underlay-list-card__media :global(.media-thumbnail__image) {
    object-fit: cover;
  }

  .underlay-list-card__media--compact {
    width: 28px;
    height: 28px;
    min-width: 28px;
    border-radius: var(--underlay-radius-sm, 0.375rem);
    font-size: 0.875rem;
  }

  .underlay-list-card__title--compact {
    flex: 1;
    min-width: 0;
    margin: 0;
    font-size: 0.9375rem;
    font-weight: 500;
    line-height: 1.3;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__drag-handle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.6;
    cursor: grab;
    touch-action: none;
    flex-shrink: 0;
    margin-left: -4px;
    margin-right: -2px;
  }

  .underlay-list-card__drag-handle:hover {
    opacity: 1;
  }

  :global(.underlay-list-card--compact):active .underlay-list-card__drag-handle {
    cursor: grabbing;
  }
</style>
