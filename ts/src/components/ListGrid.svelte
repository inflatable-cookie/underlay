<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Minimum item width - number (in ems) or string with unit */
    minItemWidth?: number | string | null;
    /** Gap between items - number (in pixels) or string with unit */
    gap?: number | string | null;
    children?: Snippet;
  }

  let {
    minItemWidth = null,
    gap = null,
    children
  }: Props = $props();

  function formatValue(value: number | string | null, defaultUnit: string): string | null {
    if (value == null) return null;
    if (typeof value === "number") return `${value}${defaultUnit}`;
    return value;
  }

  let style = $derived(
    [
      minItemWidth != null ? `--underlay-list-grid-min: ${formatValue(minItemWidth, "em")};` : null,
      gap != null ? `--underlay-list-grid-gap: ${formatValue(gap, "px")};` : null
    ]
      .filter(Boolean)
      .join(" ")
  );
</script>

<div class="underlay-list-grid" style={style || undefined}>
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .underlay-list-grid {
    display: grid;
    grid-template-columns: repeat(
      auto-fill,
      minmax(
        min(var(--underlay-list-grid-min, 360px), 100%),
        1fr
      )
    );
    gap: var(--underlay-list-grid-gap, 14px);
    align-items: stretch;
  }
</style>
