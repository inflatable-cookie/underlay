<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Additional CSS classes */
    class?: string;
    children: Snippet;
  }

  let {
    class: className,
    children
  }: Props = $props();
</script>

<div class="details-card {className ?? ''}">
  {@render children()}
</div>

<style>
  .details-card {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.25rem 1.5rem;
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-radius: var(--underlay-radius-lg, 0.75rem);
  }
</style>
