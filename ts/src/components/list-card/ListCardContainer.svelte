<script lang="ts">
  type RenderSnippet = (...args: any[]) => any;

  interface Props {
    href?: string | null;
    title: string;
    ariaLabel?: string | null;
    isLive?: boolean;
    isCompact?: boolean;
    isSelectionMode?: boolean;
    selected?: boolean;
    cardClass: string;
    style?: string;
    onclick?: ((event: MouseEvent) => void) | null;
    onCardClick?: ((event: MouseEvent) => void) | null;
    fullContent: RenderSnippet;
    compactContent: RenderSnippet;
  }

  let {
    href = null,
    title,
    ariaLabel = null,
    isLive = true,
    isCompact = false,
    isSelectionMode = false,
    selected = false,
    cardClass,
    style,
    onclick = null,
    onCardClick = null,
    fullContent,
    compactContent
  }: Props = $props();
</script>

{#if href && !isCompact && !isSelectionMode}
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
      onclick={onCardClick ?? undefined}
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

  .underlay-list-card-shell:hover {
    z-index: 2;
  }

  .underlay-list-card-shell:focus-within {
    z-index: 10;
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

  .underlay-list-card--selectable {
    cursor: pointer;
    font: inherit;
    text-align: left;
    width: 100%;
  }

  .underlay-list-card--selected {
    border-color: var(--underlay-list-card-accent);
    box-shadow: 0 0 0 1px var(--underlay-list-card-accent);
  }

  .underlay-list-card-shell--selected > .underlay-list-card {
    border-color: var(--underlay-list-card-accent);
    box-shadow: 0 0 0 1px var(--underlay-list-card-accent);
  }

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

  .underlay-list-card--draft:hover,
  .underlay-list-card-shell--draft:hover > .underlay-list-card {
    opacity: 1;
    filter: none;
  }

  @media (max-width: 480px) {
    .underlay-list-card {
      --_underlay-list-card-media-size: 56px;
    }
  }

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

  .underlay-list-card--compact:hover {
    transform: none;
    box-shadow: var(--underlay-shadow-card, none);
  }
</style>
