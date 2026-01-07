<script lang="ts">
  export let href: string | null = null;
  export let title: string;
  export let subtitle: string | null = null;
  export let ariaLabel: string | null = null;
  export let accent: string | null = null;
</script>

{#if href}
  <a
    class="underlay-list-card"
    href={href}
    aria-label={ariaLabel ?? title}
    style={accent ? `--underlay-list-card-accent: ${accent};` : undefined}
  >
    <div class="underlay-list-card__media">
      <slot name="media" />
    </div>

    <div class="underlay-list-card__body">
      <div class="underlay-list-card__title-row">
        <h3 class="underlay-list-card__title">{title}</h3>
        <slot name="trailing" />
      </div>

      {#if subtitle}
        <p class="underlay-list-card__subtitle">{subtitle}</p>
      {/if}

      <div class="underlay-list-card__meta">
        <slot />
      </div>
    </div>
  </a>
{:else}
  <div
    class="underlay-list-card"
    aria-label={ariaLabel ?? title}
    style={accent ? `--underlay-list-card-accent: ${accent};` : undefined}
  >
    <div class="underlay-list-card__media">
      <slot name="media" />
    </div>

    <div class="underlay-list-card__body">
      <div class="underlay-list-card__title-row">
        <h3 class="underlay-list-card__title">{title}</h3>
        <slot name="trailing" />
      </div>

      {#if subtitle}
        <p class="underlay-list-card__subtitle">{subtitle}</p>
      {/if}

      <div class="underlay-list-card__meta">
        <slot />
      </div>
    </div>
  </div>
{/if}

<style>
  .underlay-list-card {
    --underlay-list-card-accent: var(
      --underlay-color-primary,
      var(--froyo-color-primary, #2563eb)
    );
    --underlay-list-card-media-size: var(
      --underlay-list-card-media-size,
      var(--froyo-list-card-media-size, 76px)
    );

    display: grid;
    grid-template-columns: var(--underlay-list-card-media-size) 1fr;
    gap: var(--underlay-density-gap, var(--froyo-density-gap, 0.75rem));
    align-items: center;
    text-decoration: none;
    border-radius: var(--underlay-radius-md, var(--froyo-radius-md, 0.75rem));
    padding: var(--underlay-space-4, var(--froyo-space-4, 1rem));
    background: var(
      --underlay-color-surface-muted,
      var(--froyo-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--froyo-color-border-subtle, rgba(148, 163, 184, 0.25))
      );
    color: var(--underlay-color-text, var(--froyo-color-text, inherit));
    box-shadow: var(--underlay-shadow-card, var(--froyo-shadow-card, none));
    transition:
      transform 0.12s ease-out,
      border-color 0.12s ease-out,
      box-shadow 0.12s ease-out;
  }

  .underlay-list-card:hover {
    transform: translateY(var(--underlay-lift-hover, var(--froyo-lift-hover, -1px)));
    border-color: color-mix(
      in srgb,
      var(--underlay-list-card-accent) 65%,
      transparent
    );
    box-shadow: var(--underlay-shadow-card-hover, var(--froyo-shadow-card-hover, none));
  }

  .underlay-list-card:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--froyo-focus-ring-width, 2px)) solid
      var(--underlay-list-card-accent);
    outline-offset: var(--underlay-focus-ring-offset, var(--froyo-focus-ring-offset, 2px));
  }

  .underlay-list-card__media {
    width: var(--underlay-list-card-media-size);
    height: var(--underlay-list-card-media-size);
    border-radius: var(--underlay-radius-lg, var(--froyo-radius-lg, 1rem));
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(
      in srgb,
      var(--underlay-list-card-accent) 18%,
      var(--underlay-color-accent-tint-bg, var(--froyo-color-accent-tint-bg, rgba(255, 255, 255, 0.03)))
    );
    border: 1px solid
      color-mix(
        in srgb,
        var(--underlay-list-card-accent) 30%,
        var(--underlay-color-accent-tint-border, var(--froyo-color-accent-tint-border, rgba(148, 163, 184, 0.25)))
      );
    color: var(--underlay-list-card-accent);
    overflow: hidden;
  }

  .underlay-list-card__body {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, var(--froyo-space-1, 0.25rem));
  }

  .underlay-list-card__title-row {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--underlay-space-3, var(--froyo-space-3, 0.75rem));
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
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, #9ca3af));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-list-card__meta {
    display: flex;
    flex-direction: column;
    gap: calc(var(--underlay-space-1, var(--froyo-space-1, 0.25rem)) * 0.6);
    font-size: 0.82rem;
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, #9ca3af));
  }

  @media (max-width: 520px) {
    .underlay-list-card {
      --underlay-list-card-media-size: var(
        --underlay-list-card-media-size-sm,
        var(--froyo-list-card-media-size-sm, 64px)
      );
    }
  }
</style>
