<script lang="ts">
  import type { Snippet } from "svelte";
  import type { BannerVariant } from "./banner";

  interface Props {
    /** Visual style variant */
    variant?: BannerVariant;
    /** Main message to display */
    message: string;
    /** Optional additional content/actions */
    children?: Snippet;
  }

  let {
    variant = "warning",
    message,
    children
  }: Props = $props();
</script>

<div class="underlay-banner underlay-banner--{variant}" role="status">
  <svg
    class="underlay-banner__icon"
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    {#if variant === "error"}
      <circle cx="12" cy="12" r="10"/>
      <line x1="15" x2="9" y1="9" y2="15"/>
      <line x1="9" x2="15" y1="9" y2="15"/>
    {:else if variant === "info"}
      <circle cx="12" cy="12" r="10"/>
      <line x1="12" x2="12" y1="16" y2="12"/>
      <line x1="12" x2="12.01" y1="8" y2="8"/>
    {:else if variant === "success"}
      <circle cx="12" cy="12" r="10"/>
      <polyline points="9 12 11 14 15 10"/>
    {:else}
      <circle cx="12" cy="12" r="10"/>
      <line x1="12" x2="12" y1="8" y2="12"/>
      <line x1="12" x2="12.01" y1="16" y2="16"/>
    {/if}
  </svg>
  <span class="underlay-banner__message">{message}</span>
  {#if children}
    <div class="underlay-banner__actions">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .underlay-banner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.9rem;
  }

  .underlay-banner__icon {
    width: 1.25rem;
    height: 1.25rem;
    flex-shrink: 0;
  }

  .underlay-banner__message {
    flex: 1;
  }

  .underlay-banner__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  /* Warning variant (amber) */
  .underlay-banner--warning {
    background: rgba(251, 191, 36, 0.12);
    border: 1px solid rgba(251, 191, 36, 0.3);
    color: #fbbf24;
  }

  /* Error variant (red) */
  .underlay-banner--error {
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
  }

  /* Info variant (blue) */
  .underlay-banner--info {
    background: rgba(59, 130, 246, 0.12);
    border: 1px solid rgba(59, 130, 246, 0.3);
    color: #60a5fa;
  }

  /* Success variant (green) */
  .underlay-banner--success {
    background: rgba(34, 197, 94, 0.12);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #4ade80;
  }
</style>
