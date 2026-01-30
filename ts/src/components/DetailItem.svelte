<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** The label/key for this item */
    label: string;
    /** Plain text value (use children snippet for complex content) */
    value?: string | number | boolean | null;
    /** Display value as monospace code */
    code?: boolean;
    /** Capitalize the value */
    capitalize?: boolean;
    /** Additional CSS classes */
    class?: string;
    /** Custom content instead of plain value */
    children?: Snippet;
  }

  let {
    label,
    value = null,
    code = false,
    capitalize = false,
    class: className,
    children
  }: Props = $props();

  const displayValue = $derived(() => {
    if (value === null || value === undefined) return null;
    if (typeof value === "boolean") return value ? "Yes" : "No";
    return String(value);
  });

  const valueClass = $derived(
    ["detail-item__value", code && "detail-item__value--code", capitalize && "detail-item__value--capitalize"]
      .filter(Boolean)
      .join(" ")
  );
</script>

<div class={`detail-item ${className ?? ""}`}>
  <dt class="detail-item__label">{label}</dt>
  <dd class={valueClass}>
    {#if children}
      {@render children()}
    {:else if displayValue()}
      {displayValue()}
    {:else}
      <span class="detail-item__empty">Not set</span>
    {/if}
  </dd>
</div>

<style>
  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 1rem;
  }

  .detail-item__label {
    font-size: 0.875rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    flex-shrink: 0;
  }

  .detail-item__value {
    margin: 0;
    font-size: 0.875rem;
    color: var(--underlay-color-text, #e5e7eb);
    text-align: right;
    word-break: break-word;
  }

  .detail-item__value--code {
    font-family: var(--underlay-font-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace);
    font-size: 0.8125rem;
  }

  .detail-item__value--capitalize {
    text-transform: capitalize;
  }

  .detail-item__empty {
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.6;
  }
</style>
