<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    href?: string | null;
    title: string;
    subtitle?: string | null;
    ariaLabel?: string | null;
    accent?: string | null;
    media?: Snippet;
    trailing?: Snippet;
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    href = null,
    title,
    subtitle = null,
    ariaLabel = null,
    accent = null,
    media,
    trailing,
    actions,
    children
  }: Props = $props();

  let hasActions = $derived(Boolean(actions));
  let style = $derived(accent ? `--underlay-list-card-accent: ${accent};` : undefined);
</script>

{#if href}
  <div class="underlay-list-card-shell" {style}>
    <a
      class={`underlay-list-card underlay-list-card__link ${hasActions ? "underlay-list-card--has-actions" : ""}`}
      {href}
      aria-label={ariaLabel ?? title}
    >
      <div class="underlay-list-card__media">
        {#if media}
          {@render media()}
        {/if}
      </div>

      <div class="underlay-list-card__body">
        <div class="underlay-list-card__title-row">
          <h3 class="underlay-list-card__title">{title}</h3>
          {#if trailing}
            {@render trailing()}
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
    </a>

    {#if hasActions}
      <div class="underlay-list-card__actions">
        {#if actions}
          {@render actions()}
        {/if}
      </div>
    {/if}
  </div>
{:else}
  <div
    class={`underlay-list-card ${hasActions ? "underlay-list-card--has-actions" : ""}`}
    aria-label={ariaLabel ?? title}
    {style}
  >
    <div class="underlay-list-card__media">
      {#if media}
        {@render media()}
      {/if}
    </div>

    <div class="underlay-list-card__body">
      <div class="underlay-list-card__title-row">
        <h3 class="underlay-list-card__title">{title}</h3>
        {#if trailing}
          {@render trailing()}
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

    {#if hasActions}
      <div class="underlay-list-card__actions">
        {#if actions}
          {@render actions()}
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .underlay-list-card-shell {
    position: relative;
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
    text-decoration: none;
    border-radius: var(--underlay-radius-md, var(--underlay-radius-md, 0.75rem));
    padding: var(--underlay-space-4, var(--underlay-space-4, 1rem));
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

  /* Give the top-right overlay room. */
  .underlay-list-card--has-actions {
    padding-right: calc(var(--underlay-space-4, var(--underlay-space-4, 1rem)) + 2.25rem);
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

  .underlay-list-card__actions {
    position: absolute;
    top: calc(var(--underlay-space-4, var(--underlay-space-4, 1rem)) * 0.75);
    right: calc(var(--underlay-space-4, var(--underlay-space-4, 1rem)) * 0.75);
    z-index: 1;
  }

  .underlay-list-card__media {
    width: var(--_underlay-list-card-media-size);
    height: var(--_underlay-list-card-media-size);
    border-radius: var(--underlay-radius-lg, var(--underlay-radius-lg, 1rem));
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(
      in srgb,
      var(--underlay-list-card-accent) 18%,
      var(--underlay-color-accent-tint-bg, var(--underlay-color-accent-tint-bg, rgba(255, 255, 255, 0.03)))
    );
    border: 1px solid
      color-mix(
        in srgb,
        var(--underlay-list-card-accent) 30%,
        var(--underlay-color-accent-tint-border, var(--underlay-color-accent-tint-border, rgba(148, 163, 184, 0.25)))
      );
    color: var(--underlay-list-card-accent);
    overflow: hidden;
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

  .underlay-list-card__title {
    margin: 0;
    font-size: 1rem;
    line-height: 1.2;
    font-weight: 650;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__subtitle {
    margin: 0;
    font-size: 0.9rem;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__meta {
    display: flex;
    flex-direction: column;
    gap: calc(var(--underlay-space-1, var(--underlay-space-1, 0.25rem)) * 0.6);
    font-size: 0.82rem;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
  }

  @media (max-width: 520px) {
    .underlay-list-card {
      --_underlay-list-card-media-size: var(
        --underlay-list-card-media-size,
        var(
          --underlay-list-card-media-size-sm,
          var(--underlay-list-card-media-size-sm, 64px)
        )
      );
    }
  }
</style>
