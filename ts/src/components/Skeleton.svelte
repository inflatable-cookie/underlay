<script lang="ts" module>
  // Skeleton variant types
  export type SkeletonVariant = "text" | "title" | "button" | "avatar" | "card" | "custom";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Skeleton shape/type */
    variant?: SkeletonVariant;
    /** Width (CSS value) */
    width?: string;
    /** Height (CSS value) */
    height?: string;
    /** Border radius (CSS value) */
    radius?: string;
    /** Number of text lines (for variant="text") */
    lines?: number;
    /** Whether to animate with shimmer effect */
    animate?: boolean;
    /** Additional CSS class */
    class?: string;
    /** Children for card variant */
    children?: Snippet;
  }

  let {
    variant = "text",
    width,
    height,
    radius,
    lines = 1,
    animate = true,
    class: className = "",
    children
  }: Props = $props();

  // Compute default dimensions based on variant
  const computedWidth = $derived(width ?? getDefaultWidth(variant));
  const computedHeight = $derived(height ?? getDefaultHeight(variant));
  const computedRadius = $derived(radius ?? getDefaultRadius(variant));

  function getDefaultWidth(v: SkeletonVariant): string {
    switch (v) {
      case "title":
        return "60%";
      case "button":
        return "6rem";
      case "avatar":
        return "2.5rem";
      case "card":
        return "100%";
      case "text":
      default:
        return "100%";
    }
  }

  function getDefaultHeight(v: SkeletonVariant): string {
    switch (v) {
      case "title":
        return "1.5rem";
      case "button":
        return "2.5rem";
      case "avatar":
        return "2.5rem";
      case "card":
        return "auto";
      case "text":
      default:
        return "1rem";
    }
  }

  function getDefaultRadius(v: SkeletonVariant): string {
    switch (v) {
      case "avatar":
        return "50%";
      case "button":
        return "var(--underlay-radius-pill, 999px)";
      case "card":
        return "var(--underlay-radius-md, 0.5rem)";
      default:
        return "var(--underlay-radius-sm, 0.25rem)";
    }
  }
</script>

{#if variant === "text" && lines > 1}
  <div class="underlay-skeleton-lines {className}">
    {#each Array(lines) as _, i}
      <div
        class="underlay-skeleton underlay-skeleton--text"
        class:underlay-skeleton--animate={animate}
        style:width={i === lines - 1 ? "75%" : computedWidth}
        style:height={computedHeight}
        style:border-radius={computedRadius}
        role="presentation"
        aria-hidden="true"
      ></div>
    {/each}
  </div>
{:else if variant === "card"}
  <div
    class="underlay-skeleton underlay-skeleton--card {className}"
    class:underlay-skeleton--animate={animate}
    style:width={computedWidth}
    style:border-radius={computedRadius}
    role="presentation"
    aria-hidden="true"
  >
    {#if children}
      <div class="underlay-skeleton-card__content">
        {@render children()}
      </div>
    {/if}
  </div>
{:else}
  <div
    class="underlay-skeleton underlay-skeleton--{variant} {className}"
    class:underlay-skeleton--animate={animate}
    style:width={computedWidth}
    style:height={computedHeight}
    style:border-radius={computedRadius}
    role="presentation"
    aria-hidden="true"
  ></div>
{/if}

<style>
  .underlay-skeleton {
    background-color: var(--underlay-skeleton-bg, rgba(148, 163, 184, 0.15));
    position: relative;
    overflow: hidden;
  }

  /* Shimmer animation */
  .underlay-skeleton--animate::after {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      var(--underlay-skeleton-shimmer, rgba(255, 255, 255, 0.08)) 50%,
      transparent 100%
    );
    animation: underlay-skeleton-shimmer 1.5s infinite;
  }

  @keyframes underlay-skeleton-shimmer {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(100%);
    }
  }

  /* Text lines container */
  .underlay-skeleton-lines {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  /* Card variant */
  .underlay-skeleton--card {
    padding: var(--underlay-space-4, 1rem);
    min-height: 4rem;
  }

  .underlay-skeleton-card__content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  /* Avatar - ensure circle */
  .underlay-skeleton--avatar {
    flex-shrink: 0;
  }

  /* Dark mode support */
  :global([data-theme="dark"]) .underlay-skeleton {
    background-color: var(--underlay-skeleton-bg-dark, rgba(148, 163, 184, 0.1));
  }

  :global([data-theme="dark"]) .underlay-skeleton--animate::after {
    background: linear-gradient(
      90deg,
      transparent 0%,
      var(--underlay-skeleton-shimmer-dark, rgba(255, 255, 255, 0.05)) 50%,
      transparent 100%
    );
  }

  /* Reduce motion for accessibility */
  @media (prefers-reduced-motion: reduce) {
    .underlay-skeleton--animate::after {
      animation: none;
    }
  }
</style>
