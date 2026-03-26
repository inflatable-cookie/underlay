<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    label?: string;
    children: Snippet;
  }

  let { label, children }: Props = $props();
</script>

<div class="underlay-detail-meta__item">
  {#if label}
    <span class="underlay-detail-meta__label">{label}</span>
  {/if}
  <span class="underlay-detail-meta__value">{@render children()}</span>
</div>

<style>
  .underlay-detail-meta__item {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    width: max-content;
  }

  .underlay-detail-meta__label {
    font-size: 0.8em;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.75));
  }

  .underlay-detail-meta__label::after {
    content: ":";
    margin-left: 0.12rem;
    opacity: 0.5;
  }

  .underlay-detail-meta__value {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    color: var(--underlay-color-text, #e5e7eb);
  }
</style>
