<script lang="ts">
  import ListCardCompactContent from "./list-card/ListCardCompactContent.svelte";
  import ListCardMediaSlot from "./list-card/ListCardMediaSlot.svelte";
  import type { Snippet } from "svelte";

  type ListCardVariant = "default" | "compact";

  interface Props {
    href?: string | null;
    title: string;
    /** Optional content to render after the title (e.g., badges, tags) */
    titleSuffix?: Snippet;
    subtitle?: string | null;
    ariaLabel?: string | null;
    accent?: string | null;
    /** Visual variant - 'compact' shows small icon + title only for reorder mode */
    variant?: ListCardVariant;
    /** When false, card displays with reduced opacity and dashed border to indicate draft/hidden status */
    isLive?: boolean;
    /** Show drag handle for reorder mode (only visible in compact variant) */
    showDragHandle?: boolean;
    /** Whether this card is selected (enables selection mode when provided) */
    selected?: boolean;
    /** Callback when selection changes - providing this enables selection mode */
    onSelectionChange?: (selected: boolean) => void;
    media?: Snippet;
    trailing?: Snippet;
    /** Renders the actions menu. When provided, the media area becomes a custom trigger containing the icon + dots.
     * The snippet receives `trigger` (the media content to render) and `align` (recommended dropdown alignment). */
    actions?: Snippet<[{ trigger: Snippet; align: "start" | "center" | "end" }]>;
    /** Where to place the actions trigger. Defaults to "media". */
    actionsPlacement?: "media" | "media-overlay" | "trailing";
    children?: Snippet;
    onclick?: ((event: MouseEvent) => void) | null;
  }

  let {
    href = null,
    title,
    titleSuffix,
    subtitle = null,
    ariaLabel = null,
    accent = null,
    variant = "default",
    isLive = true,
    showDragHandle = false,
    selected = false,
    onSelectionChange,
    media,
    trailing,
    actions,
    actionsPlacement = "media",
    children,
    onclick = null
  }: Props = $props();

  let hasActions = $derived(Boolean(actions));
  let isSelectionMode = $derived(Boolean(onSelectionChange));
  let style = $derived(accent ? `--underlay-list-card-accent: ${accent};` : undefined);
  let isCompact = $derived(variant === "compact");
  let cardClass = $derived([
    "underlay-list-card",
    !isLive && "underlay-list-card--draft",
    isCompact && "underlay-list-card--compact",
    isSelectionMode && "underlay-list-card--selectable"
  ].filter(Boolean).join(" "));

  function handleSelectionToggle(e: Event) {
    e.stopPropagation();
    onSelectionChange?.(!selected);
  }

  function handleCardClick(e: MouseEvent) {
    // In selection mode, clicking anywhere on the card toggles selection
    if (isSelectionMode) {
      e.preventDefault();
      onSelectionChange?.(!selected);
    }
  }
</script>

{#snippet actionsTrigger()}
  <span class="underlay-list-card__dots-only" aria-hidden="true">⋯</span>
{/snippet}

{#snippet fullContent()}
  <ListCardMediaSlot
    {media}
    {actions}
    {actionsPlacement}
    {isSelectionMode}
    {selected}
    onSelectionToggle={handleSelectionToggle}
  />

  <div class="underlay-list-card__body">
    <div class="underlay-list-card__title-row">
      <h3 class="underlay-list-card__title">
        {title}{#if titleSuffix}{@render titleSuffix()}{/if}
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
{/snippet}

{#if href && !isCompact && !isSelectionMode}
  <!-- Normal link mode -->
  <div class="underlay-list-card-shell" class:underlay-list-card-shell--draft={!isLive} {style}>
    <a
      class={cardClass}
      class:underlay-list-card__link={true}
      {href}
      aria-label={ariaLabel ?? title}
      onclick={onclick ?? undefined}
      {style}
    >
      {@render fullContent()}
    </a>
  </div>
{:else if isSelectionMode && !isCompact}
  <!-- Selection mode - entire card is clickable for toggle -->
  <div
    class="underlay-list-card-shell"
    class:underlay-list-card-shell--draft={!isLive}
    class:underlay-list-card-shell--selected={selected}
    {style}
  >
    <button
      type="button"
      class={cardClass}
      class:underlay-list-card--selected={selected}
      aria-label={selected ? `Deselect ${title}` : `Select ${title}`}
      aria-pressed={selected}
      onclick={handleCardClick}
      {style}
    >
      {@render fullContent()}
    </button>
  </div>
{:else}
  <div
    class={cardClass}
    class:underlay-list-card-shell--draft={!isLive && !isCompact}
    aria-label={ariaLabel ?? title}
    {style}
  >
    {#if isCompact}
      <ListCardCompactContent {title} {showDragHandle} {media} />
    {:else}
      {@render fullContent()}
    {/if}
  </div>
{/if}

<style>
  .underlay-list-card-shell {
    position: relative;
    min-width: 0;
  }

  .underlay-list-card {
    --underlay-list-card-accent: var(
      --underlay-color-primary,
      var(--underlay-color-primary, #2563eb)
    );
    --_underlay-list-card-media-size: var(
      --underlay-list-card-media-size,
      var(--underlay-list-card-media-size, 76px)
    );

    --_underlay-list-card-gap: var(
      --underlay-list-card-gap,
      calc(var(--underlay-density-gap, var(--underlay-density-gap, 0.75rem)) * 1.35)
    );

    display: grid;
    grid-template-columns: var(--_underlay-list-card-media-size) 1fr;
    gap: var(--_underlay-list-card-gap);
    align-items: center;
    min-width: 0;
    text-decoration: none;
    border-radius: var(--underlay-radius-lg, var(--underlay-radius-lg, 1rem));
    padding: var(--underlay-space-3, var(--underlay-space-3, 0.75rem));
    background: var(
      --underlay-color-surface-muted,
      var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25))
      );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    box-shadow: var(--underlay-shadow-card, var(--underlay-shadow-card, none));
    transition:
      transform 0.12s ease-out,
      border-color 0.12s ease-out,
      box-shadow 0.12s ease-out;
  }

  .underlay-list-card-shell:hover > .underlay-list-card {
    transform: translateY(var(--underlay-lift-hover, var(--underlay-lift-hover, -1px)));
    border-color: color-mix(
      in srgb,
      var(--underlay-list-card-accent) 65%,
      transparent
    );
    box-shadow: var(--underlay-shadow-card-hover, var(--underlay-shadow-card-hover, none));
  }

  .underlay-list-card:hover {
    transform: translateY(var(--underlay-lift-hover, var(--underlay-lift-hover, -1px)));
    border-color: color-mix(
      in srgb,
      var(--underlay-list-card-accent) 65%,
      transparent
    );
    box-shadow: var(--underlay-shadow-card-hover, var(--underlay-shadow-card-hover, none));
  }

  .underlay-list-card:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-list-card-accent);
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  /* Make entire card clickable in selection mode */
  .underlay-list-card--selectable {
    cursor: pointer;
    /* Reset button styles when card is a button */
    font: inherit;
    text-align: left;
    width: 100%;
  }

  /* Selected state */
  .underlay-list-card--selected {
    border-color: var(--underlay-list-card-accent);
    box-shadow: 0 0 0 1px var(--underlay-list-card-accent);
  }

  .underlay-list-card-shell--selected > .underlay-list-card {
    border-color: var(--underlay-list-card-accent);
    box-shadow: 0 0 0 1px var(--underlay-list-card-accent);
  }

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

  /* Draft/non-live state styling - monochrome with visible dashed border */
  .underlay-list-card--draft {
    opacity: 0.7;
    filter: grayscale(100%);
    border-style: dashed;
    border-color: var(--underlay-color-border-muted, rgba(148, 163, 184, 0.5));
  }

  .underlay-list-card-shell--draft > .underlay-list-card {
    opacity: 0.7;
    filter: grayscale(100%);
    border-style: dashed;
    border-color: var(--underlay-color-border-muted, rgba(148, 163, 184, 0.5));
  }

  /* Restore colour on hover */
  .underlay-list-card--draft:hover,
  .underlay-list-card-shell--draft:hover > .underlay-list-card {
    opacity: 1;
    filter: none;
  }

  /* =========================================================================
     Mobile responsive - reduce media size on smaller screens
     ========================================================================= */

  @media (max-width: 480px) {
    .underlay-list-card {
      --_underlay-list-card-media-size: 56px;
    }

    /* Smaller pills on mobile */
    .underlay-list-card__title-actions :global(.underlay-pill) {
      font-size: 0.5em;
      padding: 0.2em 0.5em;
    }
  }

  /* =========================================================================
     Compact variant - for reorder mode
     Smaller icon, title only, ~48px height, optional drag handle
     ========================================================================= */

  .underlay-list-card--compact {
    --_underlay-list-card-media-size: 28px;

    display: flex;
    align-items: center;
    gap: var(--underlay-space-3, 0.75rem);
    padding: var(--underlay-space-2, 0.5rem) var(--underlay-space-3, 0.75rem);
    min-height: 48px;
    border-radius: var(--underlay-radius-md, 0.5rem);
    cursor: grab;
  }

  .underlay-list-card--compact:active {
    cursor: grabbing;
  }

  /* Disable hover effects in compact mode - it's for reordering, not navigation */
  .underlay-list-card--compact:hover {
    transform: none;
    box-shadow: var(--underlay-shadow-card, none);
  }

</style>
