<script lang="ts">
  import type { Component, SvelteComponent, Snippet } from "svelte";
  import { Button } from "@poodle/svelte-primitives";

  type IconComponent =
    | Component<{ size?: number }>
    | (new (...args: any[]) => SvelteComponent);

  interface Props {
    /** Lucide icon or custom component displayed above the title */
    icon?: IconComponent;
    /** Primary message */
    title: string;
    /** Secondary explanatory text */
    description?: string;
    /** CTA button label */
    actionLabel?: string;
    /** CTA link (renders an anchor-styled button) */
    actionHref?: string;
    /** CTA callback (alternative to href) */
    onaction?: () => void;
    /** Full-size (centered block) or compact (inline, smaller) */
    variant?: "default" | "compact";
    /** Override entire content */
    children?: Snippet;
    /** Additional CSS class */
    class?: string;
  }

  let {
    icon: Icon = undefined,
    title,
    description = undefined,
    actionLabel = undefined,
    actionHref = undefined,
    onaction = undefined,
    variant = "default",
    children,
    class: className = ""
  }: Props = $props();

  const hasAction = $derived(actionLabel && (actionHref || onaction));
  const iconSize = $derived(variant === "compact" ? 24 : 40);
</script>

{#if children}
  <div class="underlay-empty-state underlay-empty-state--{variant} {className}">
    {@render children()}
  </div>
{:else}
  <div class="underlay-empty-state underlay-empty-state--{variant} {className}">
    {#if Icon}
      <div class="underlay-empty-state__icon">
        <Icon size={iconSize} />
      </div>
    {/if}

    <p class="underlay-empty-state__title">{title}</p>

    {#if description}
      <p class="underlay-empty-state__description">{description}</p>
    {/if}

    {#if hasAction}
      <div class="underlay-empty-state__action">
        {#if actionHref}
          <a href={actionHref} class="underlay-empty-state__action-link">
            {actionLabel}
          </a>
        {:else if onaction}
          <Button variant="ghost" size="sm" on:click={onaction}>
            {actionLabel}
          </Button>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  :global(.underlay-empty-state) {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    gap: 0.5rem;
  }

  :global(.underlay-empty-state--default) {
    padding: 3rem 1.5rem;
  }

  :global(.underlay-empty-state--compact) {
    padding: 1.5rem 1rem;
  }

  :global(.underlay-empty-state__icon) {
    color: var(--underlay-color-text-muted, #64748b);
    opacity: 0.5;
    margin-bottom: 0.25rem;
  }

  :global(.underlay-empty-state--compact .underlay-empty-state__icon) {
    margin-bottom: 0;
  }

  :global(.underlay-empty-state__title) {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    font-weight: 500;
    color: var(--underlay-color-text-muted, #64748b);
  }

  :global(.underlay-empty-state--compact .underlay-empty-state__title) {
    font-size: var(--underlay-font-size-xs, 0.8125rem);
  }

  :global(.underlay-empty-state__description) {
    margin: 0;
    font-size: var(--underlay-font-size-xs, 0.8125rem);
    color: var(--underlay-color-text-muted, #64748b);
    opacity: 0.8;
    max-width: 28rem;
  }

  :global(.underlay-empty-state--compact .underlay-empty-state__description) {
    font-size: 0.75rem;
  }

  :global(.underlay-empty-state__action) {
    margin-top: 0.5rem;
  }

  :global(.underlay-empty-state__action a) {
    text-decoration: none;
  }

  :global(.underlay-empty-state__action-link) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--poodle-typography-label-weight);
    line-height: 1;
    text-decoration: none;
  }

  :global(.underlay-empty-state__action-link:hover) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 72%, transparent);
  }

  :global(.underlay-empty-state__action-link:focus-visible) {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
