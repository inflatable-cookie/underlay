<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Optional title for the list */
    title?: string;
    /** Additional CSS classes */
    class?: string;
    /** List items (use DetailItem components) */
    children?: Snippet;
  }

  let { title, class: className, children }: Props = $props();
</script>

<div class={`detail-list ${className ?? ""}`}>
  {#if title}
    <h3 class="detail-list__title">{title}</h3>
  {/if}
  <dl class="detail-list__items">
    {@render children?.()}
  </dl>
</div>

<style>
  .detail-list {
    display: flex;
    flex-direction: column;
  }

  .detail-list__title {
    margin: 0 0 0.75rem;
    font-size: 0.65rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.7;
  }

  .detail-list__items {
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
