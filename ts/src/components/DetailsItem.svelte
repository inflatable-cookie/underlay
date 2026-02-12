<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** The label/key for this detail item */
    label: string;
    /** Plain text value (use children snippet for complex content) */
    value?: string | number | null;
    /** Display value as monospace code */
    code?: boolean;
    /** Span multiple columns (use "full" for entire row) */
    span?: number | "full";
    /** Show as muted/secondary (for less important info) */
    muted?: boolean;
    /** Additional CSS classes */
    class?: string;
    /** Custom content instead of plain value */
    children?: Snippet;
  }

  let {
    label,
    value = null,
    code = false,
    span,
    muted = false,
    class: className,
    children
  }: Props = $props();

  const style = $derived(
    span
      ? `grid-column: span ${span === "full" ? "var(--details-grid-columns, 4)" : span}`
      : undefined
  );

  const itemClass = $derived(
    [
      "details-item",
      muted && "details-item--muted",
      className
    ]
      .filter(Boolean)
      .join(" ")
  );
</script>

<div class={itemClass} {style}>
  <dt class="underlay-details-item__label">{label}</dt>
  <dd class="underlay-details-item__value">
    {#if children}
      {@render children()}
    {:else if value !== null && value !== undefined}
      {#if code}
        <code class="underlay-details-item__code">{value}</code>
      {:else}
        {value}
      {/if}
    {:else}
      <span class="underlay-details-item__empty">—</span>
    {/if}
  </dd>
</div>

<style>
  .underlay-details-item {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    min-width: 0;
  }

  .underlay-details-item__label {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--underlay-color-text-muted, #64748b);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    line-height: 1.2;
  }

  .underlay-details-item__value {
    margin: 0;
    font-size: 1rem;
    color: var(--underlay-color-text, #e5e7eb);
    line-height: 1.4;
    word-break: break-word;
  }

  .underlay-details-item__code {
    display: inline-block;
    font-family: var(--underlay-font-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace);
    font-size: 0.875em;
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    background: rgba(148, 163, 184, 0.1);
    padding: 0.15em 0.4em;
    border-radius: 0.25rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-details-item__empty {
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-details-item--muted .underlay-details-item__value {
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
