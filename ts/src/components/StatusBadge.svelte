<script lang="ts">
  import type { Snippet } from "svelte";

  /**
   * StatusBadge - A generic badge component for displaying boolean states
   *
   * Shows a colored badge with customizable labels and colors.
   * Green for positive/active states, orange for negative/inactive states.
   *
   * Common use cases:
   * - Live/Draft status
   * - Free/Restricted access
   * - Active/Inactive
   * - Published/Unpublished
   */
  interface Props {
    /** The boolean state to display */
    value: boolean;
    /** Label when value is true */
    trueLabel: string;
    /** Label when value is false */
    falseLabel: string;
    /**
     * Variant for styling
     * - success: green when true, orange when false (default)
     * - danger: green when true, red when false
     */
    variant?: "success" | "danger";
    /** Optional icon to show when value is true */
    trueIcon?: Snippet;
    /** Optional icon to show when value is false */
    falseIcon?: Snippet;
  }

  let {
    value,
    trueLabel,
    falseLabel,
    variant = "success",
    trueIcon,
    falseIcon
  }: Props = $props();

  const currentIcon = $derived(value ? trueIcon : falseIcon);
  const currentLabel = $derived(value ? trueLabel : falseLabel);
</script>

<span
  class="status-badge"
  class:status-badge--true={value}
  class:status-badge--false={!value}
  class:status-badge--danger={variant === "danger"}
  class:status-badge--with-icon={currentIcon !== undefined}
>
  {#if currentIcon}
    <span class="status-badge__icon">
      {@render currentIcon()}
    </span>
  {/if}
  <span class="status-badge__label">{currentLabel}</span>
</span>

<style>
  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.35em;
    padding: 0.2em 0.6em;
    font-size: 0.85rem;
    font-weight: 500;
    border-radius: 0.25rem;
    transition: background-color 0.2s ease, color 0.2s ease;
  }

  .status-badge__icon {
    display: inline-flex;
    align-items: center;
    line-height: 0;
  }

  .status-badge__icon :global(svg) {
    width: 1em;
    height: 1em;
  }

  .status-badge__label {
    line-height: 1;
  }

  /* Success variant (default): green/orange */
  .status-badge--true {
    background: rgba(34, 197, 94, 0.2);
    color: #22c55e;
  }

  .status-badge--false {
    background: rgba(251, 146, 60, 0.2);
    color: #fb923c;
  }

  /* Danger variant: green/red */
  .status-badge--danger.status-badge--false {
    background: rgba(239, 68, 68, 0.2);
    color: #ef4444;
  }
</style>
