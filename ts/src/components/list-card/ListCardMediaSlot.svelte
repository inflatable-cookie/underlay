<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    media?: Snippet;
    actions?: Snippet<[{ trigger: Snippet; align: "start" | "center" | "end" }]>;
    actionsPlacement?: "media" | "media-overlay" | "trailing";
    isSelectionMode?: boolean;
    selected?: boolean;
    onSelectionToggle?: (event: Event) => void;
  }

  let {
    media,
    actions,
    actionsPlacement = "media",
    isSelectionMode = false,
    selected = false,
    onSelectionToggle
  }: Props = $props();

  let hasActions = $derived(Boolean(actions));

  function handleSelectionToggle(event: Event) {
    onSelectionToggle?.(event);
  }
</script>

{#snippet mediaTrigger()}
  <span class="underlay-list-card__media-content">
    <span class="underlay-list-card__icon">
      {#if media}
        {@render media()}
      {/if}
    </span>
    <span class="underlay-list-card__dots" aria-hidden="true">⋯</span>
  </span>
{/snippet}

{#snippet actionsTrigger()}
  <span class="underlay-list-card__dots-only" aria-hidden="true">⋯</span>
{/snippet}

{#if isSelectionMode}
  <button
    type="button"
    class="underlay-list-card__media underlay-list-card__media--selectable"
    onclick={handleSelectionToggle}
    aria-label={selected ? "Deselect" : "Select"}
  >
    <input
      type="checkbox"
      class="underlay-list-card__checkbox"
      checked={selected}
      onchange={handleSelectionToggle}
      onclick={(e) => e.stopPropagation()}
    />
  </button>
{:else if hasActions && actionsPlacement === "media"}
  <div class="underlay-list-card__media-slot">
    {@render actions?.({ trigger: mediaTrigger, align: "start" })}
  </div>
{:else}
  <div class="underlay-list-card__media-wrap">
    <div class="underlay-list-card__media">
      {#if media}
        {@render media()}
      {/if}
    </div>
    {#if hasActions && actionsPlacement === "media-overlay"}
      <div class="underlay-list-card__media-actions">
        {@render actions?.({ trigger: actionsTrigger, align: "end" })}
      </div>
    {/if}
  </div>
{/if}

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

  .underlay-list-card__media-wrap {
    position: relative;
    width: var(--_underlay-list-card-media-size);
    height: var(--_underlay-list-card-media-size);
  }

  .underlay-list-card__media-actions {
    position: absolute;
    bottom: 0.2rem;
    left: 50%;
    transform: translateX(-50%);
  }

  .underlay-list-card__media-actions :global(.underlay-dropdown-menu-trigger) {
    width: auto;
    height: 1.25rem;
    padding: 0 0.5rem;
    border-radius: 999px;
    border: 1px solid rgba(148, 163, 184, 0.3);
    background: rgba(15, 23, 42, 0.5);
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    backdrop-filter: blur(6px);
    font-size: 0.75rem;
    line-height: 1;
  }

  .underlay-list-card__media-actions :global(.underlay-dropdown-menu-trigger:hover) {
    background: rgba(15, 23, 42, 0.75);
  }

  .underlay-list-card__media--selectable {
    border: none;
    cursor: pointer;
    transition: background-color 0.12s ease-out, filter 0.12s ease-out;
    filter: grayscale(60%) opacity(0.7);
  }

  .underlay-list-card__media--selectable:hover {
    filter: grayscale(30%) opacity(0.85);
  }

  :global(.underlay-list-card--selected) .underlay-list-card__media--selectable {
    filter: none;
  }

  .underlay-list-card__media--selectable:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-list-card-accent);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-list-card__checkbox {
    width: 28px;
    height: 28px;
    border: none;
    outline: none;
    appearance: none;
    -webkit-appearance: none;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    position: relative;
  }

  .underlay-list-card__checkbox:checked {
    background: rgba(255, 255, 255, 0.95);
  }

  .underlay-list-card__checkbox:checked::after {
    content: "";
    position: absolute;
    left: 9px;
    top: 4px;
    width: 8px;
    height: 14px;
    border: solid var(--underlay-list-card-accent);
    border-width: 0 3px 3px 0;
    transform: rotate(45deg);
  }

  .underlay-list-card__media-slot {
    width: var(--_underlay-list-card-media-size);
    height: var(--_underlay-list-card-media-size);
  }

  .underlay-list-card__media-slot :global(.underlay-dropdown-menu-trigger) {
    width: 100%;
    height: 100%;
    border-radius: var(--underlay-radius-md, var(--underlay-radius-md, 0.75rem));
    padding: 0;
    background: var(--underlay-list-card-accent);
    border: none;
    color: var(--underlay-color-on-primary, #fff);
    cursor: pointer;
  }

  .underlay-list-card__media-slot :global(.underlay-dropdown-menu-trigger:hover) {
    background: color-mix(in srgb, var(--underlay-list-card-accent) 85%, black);
  }

  .underlay-list-card__media-slot :global(.underlay-dropdown-menu-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-list-card-accent);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-list-card__media-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    gap: 0.15rem;
  }

  .underlay-list-card__dots-only {
    font-size: 1rem;
    font-weight: 700;
    line-height: 1;
  }

  .underlay-list-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    width: 100%;
    padding-top: 0.25rem;
    color: var(--underlay-color-on-primary, #fff);
    overflow: hidden;
  }

  .underlay-list-card__icon :global(svg) {
    max-width: 100%;
    max-height: 100%;
    flex-shrink: 0;
  }

  .underlay-list-card__dots {
    font-size: 1rem;
    font-weight: 700;
    line-height: 1;
    opacity: 0.8;
    padding-bottom: 0.3rem;
    color: var(--underlay-color-on-primary, #fff);
  }
</style>
