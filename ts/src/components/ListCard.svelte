<script lang="ts">
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

{#snippet dragHandle()}
  <div class="underlay-list-card__drag-handle" aria-label="Drag to reorder">
    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
      <circle cx="5" cy="3" r="1.5"/>
      <circle cx="11" cy="3" r="1.5"/>
      <circle cx="5" cy="8" r="1.5"/>
      <circle cx="11" cy="8" r="1.5"/>
      <circle cx="5" cy="13" r="1.5"/>
      <circle cx="11" cy="13" r="1.5"/>
    </svg>
  </div>
{/snippet}

{#snippet compactContent()}
  {#if showDragHandle}
    {@render dragHandle()}
  {/if}
  <div class="underlay-list-card__media underlay-list-card__media--compact">
    {#if media}
      {@render media()}
    {/if}
  </div>
  <span class="underlay-list-card__title underlay-list-card__title--compact">{title}</span>
{/snippet}

{#snippet fullContent()}
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
      {@render compactContent()}
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

  /* Selection mode - clickable media area with checkbox */
  .underlay-list-card__media--selectable {
    border: none;
    cursor: pointer;
    transition: background-color 0.12s ease-out, filter 0.12s ease-out;
    filter: grayscale(60%) opacity(0.7);
  }

  .underlay-list-card__media--selectable:hover {
    filter: grayscale(30%) opacity(0.85);
  }

  /* Full color when selected */
  .underlay-list-card--selected .underlay-list-card__media--selectable {
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

  /* Slot wrapper for the actions trigger - no styling, just positioning */
  .underlay-list-card__media-slot {
    width: var(--_underlay-list-card-media-size);
    height: var(--_underlay-list-card-media-size);
  }

  /* The dropdown trigger button becomes the media area */
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

  /* Constrain SVG icons to fit within the icon area */
  .underlay-list-card__icon :global(svg) {
    max-width: 100%;
    max-height: 100%;
    flex-shrink: 0;
  }

  /* Also constrain icons in the plain media area (non-actions mode) */
  .underlay-list-card__media :global(svg) {
    max-width: 60%;
    max-height: 60%;
  }

  /* Support MediaThumbnail filling the icon area */
  .underlay-list-card__icon :global(.media-thumbnail) {
    width: 100%;
    height: 100%;
  }

  .underlay-list-card__icon :global(.media-thumbnail__image) {
    object-fit: cover;
  }

  .underlay-list-card__dots {
    font-size: 1rem;
    font-weight: 700;
    line-height: 1;
    opacity: 0.8;
    padding-bottom: 0.3rem;
    color: var(--underlay-color-on-primary, #fff);
  }

  .underlay-list-card__body {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .underlay-list-card__title-row {
    display: flex;
    align-items: baseline;
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

  /* Compact media icon */
  .underlay-list-card__media--compact {
    width: 28px;
    height: 28px;
    min-width: 28px;
    border-radius: var(--underlay-radius-sm, 0.375rem);
    font-size: 0.875rem;
  }

  /* Compact title - inline, single line, truncated */
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

  /* Drag handle styling */
  .underlay-list-card__drag-handle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.6;
    cursor: grab;
    touch-action: none; /* Prevent scroll while dragging on touch devices */
    flex-shrink: 0;
    margin-left: -4px;
    margin-right: -2px;
  }

  .underlay-list-card__drag-handle:hover {
    opacity: 1;
  }

  .underlay-list-card--compact:active .underlay-list-card__drag-handle {
    cursor: grabbing;
  }
</style>
