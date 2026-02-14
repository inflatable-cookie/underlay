<script lang="ts">
  import type { Component, Snippet } from "svelte";
  import Button from "./Button.svelte";

  interface Props {
    /** Lucide icon or custom component displayed above the title */
    icon?: Component<{ size?: number }>;
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
          <a href={actionHref} class="underlay-button underlay-button--pill underlay-button--subtle underlay-button--sm">
            {actionLabel}
          </a>
        {:else if onaction}
          <Button variant="subtle" size="sm" onclick={onaction}>
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
</style>
