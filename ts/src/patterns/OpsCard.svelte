<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    class?: string;
    children?: Snippet;
  }

  let {
    title,
    class: className,
    children,
  }: Props = $props();
</script>

<article class={`underlay-ops-card ${className ?? ""}`}>
  <h2 class="underlay-ops-card__title">{title}</h2>
  <div class="underlay-ops-card__body">{@render children?.()}</div>
</article>

<style>
  .underlay-ops-card {
    padding: 0.9rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35));
    border-radius: 0.6rem;
    background: var(--underlay-color-surface-card, rgba(15, 23, 42, 0.45));
  }

  .underlay-ops-card__title {
    margin: 0 0 0.4rem;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .underlay-ops-card__body {
    font-size: 0.86rem;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }
</style>
