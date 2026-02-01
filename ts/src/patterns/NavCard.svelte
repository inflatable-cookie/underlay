<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    href: string;
    title: string;
    description?: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon?: any;
    /** Custom background for the icon (CSS gradient or color) */
    iconBackground?: string;
    variant?: "default" | "danger";
    children?: Snippet;
  }

  let {
    href,
    title,
    description,
    icon: Icon,
    iconBackground,
    variant = "default",
    children
  }: Props = $props();

  let iconStyle = $derived(iconBackground ? `background: ${iconBackground};` : undefined);
</script>

<a class="underlay-nav-card underlay-nav-card--{variant}" {href}>
  {#if Icon}
    {@const IconComponent = Icon}
    <span class="underlay-nav-card__icon" style={iconStyle} aria-hidden="true">
      <IconComponent class="underlay-nav-card__icon-svg" />
    </span>
  {/if}

  <div class="underlay-nav-card__content">
    <span class="underlay-nav-card__title">{title}</span>
    {#if description}
      <span class="underlay-nav-card__description">{description}</span>
    {/if}
    {#if children}
      <div class="underlay-nav-card__extra">{@render children()}</div>
    {/if}
  </div>
</a>

<style>
  .underlay-nav-card {
    display: flex;
    align-items: flex-start;
    gap: var(--underlay-space-4, 1rem);
    padding: var(--underlay-space-5, 1.25rem);
    border-radius: var(--underlay-radius-lg, 0.75rem);
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.03));
    border: 1px solid var(--underlay-color-border-subtle, rgba(255, 255, 255, 0.1));
    text-decoration: none;
    color: inherit;
    transition: background-color 0.15s ease, border-color 0.15s ease;
  }

  .underlay-nav-card:hover {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.06));
    border-color: var(--underlay-color-border-strong, rgba(148, 163, 184, 0.3));
  }

  .underlay-nav-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border-radius: var(--underlay-radius-md, 0.5rem);
    background: var(--underlay-color-primary, #3b82f6);
    flex-shrink: 0;
  }

  :global(.underlay-nav-card__icon-svg) {
    width: 1.5rem;
    height: 1.5rem;
    color: var(--underlay-color-on-primary, #fff);
  }

  .underlay-nav-card__content {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    min-width: 0;
  }

  .underlay-nav-card__title {
    font-weight: 600;
    font-size: 1.05em;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-nav-card__description {
    font-size: 0.875em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    line-height: 1.5;
  }

  .underlay-nav-card__extra {
    margin-top: var(--underlay-space-2, 0.5rem);
  }

  /* Danger variant */
  .underlay-nav-card--danger {
    border-color: rgba(239, 68, 68, 0.3);
  }

  .underlay-nav-card--danger:hover {
    border-color: rgba(239, 68, 68, 0.5);
    background: rgba(239, 68, 68, 0.08);
  }

  .underlay-nav-card--danger .underlay-nav-card__icon {
    background: linear-gradient(135deg, #dc2626, #ef4444);
  }
</style>
