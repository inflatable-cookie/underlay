<script lang="ts">
  import type { Snippet } from "svelte";

  type ListGridVariant = "default" | "compact";

  interface Props {
    /** Visual variant - 'compact' uses single column layout with tighter spacing */
    variant?: ListGridVariant;
    /** Minimum item width - number (in ems) or string with unit (ignored in compact mode) */
    minItemWidth?: number | string | null;
    /** Gap between items - number (in pixels) or string with unit */
    gap?: number | string | null;
    /** Actions slot for toolbar buttons (e.g., Save/Cancel in reorder mode) */
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    variant = "default",
    minItemWidth = null,
    gap = null,
    actions,
    children
  }: Props = $props();

  let isCompact = $derived(variant === "compact");
  let hasActions = $derived(typeof actions === "function");
  let hasChildren = $derived(typeof children === "function");

  function formatValue(value: number | string | null, defaultUnit: string): string | null {
    if (value == null) return null;
    if (typeof value === "number") return `${value}${defaultUnit}`;
    return value;
  }

  let style = $derived(
    [
      !isCompact && minItemWidth != null ? `--underlay-list-grid-min: ${formatValue(minItemWidth, "em")};` : null,
      gap != null ? `--underlay-list-grid-gap: ${formatValue(gap, "px")};` : null
    ]
      .filter(Boolean)
      .join(" ")
  );

  let gridClass = $derived([
    "underlay-list-grid",
    isCompact && "underlay-list-grid--compact"
  ].filter(Boolean).join(" "));
</script>

{#if hasActions}
  <div class="underlay-list-grid-header">
    {@render actions?.()}
  </div>
{/if}

<div class={gridClass} style={style || undefined}>
  {#if hasChildren}
    {@render children?.()}
  {/if}
</div>

<style>
  .underlay-list-grid-header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-list-grid {
    display: grid;
    grid-template-columns: repeat(
      auto-fill,
      minmax(
        min(var(--underlay-list-grid-min, 360px), 100%),
        1fr
      )
    );
    gap: var(--underlay-list-grid-gap, 1.25rem);
    align-items: stretch;
  }

  /* Compact variant: single column, tighter spacing */
  .underlay-list-grid--compact {
    grid-template-columns: 1fr;
    gap: var(--underlay-list-grid-gap, 0.5rem);
  }
</style>
