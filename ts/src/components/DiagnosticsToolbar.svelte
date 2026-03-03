<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    title?: string;
    children?: Snippet;
    actions?: Snippet;
  }

  let { class: className = "", title, children, actions }: Props = $props();
</script>

<section class={`underlay-diagnostics-toolbar ${className}`}>
  <div class="underlay-diagnostics-toolbar__content">
    {#if title}
      <span class="underlay-diagnostics-toolbar__title">{title}</span>
    {/if}
    {@render children?.()}
  </div>
  {#if actions}
    <div class="underlay-diagnostics-toolbar__actions">
      {@render actions()}
    </div>
  {/if}
</section>

<style>
  .underlay-diagnostics-toolbar {
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: var(--underlay-radius-sm, 0.45rem);
    background: var(--underlay-color-surface-muted, rgba(15, 23, 42, 0.2));
    padding: 0.45rem 0.55rem;
    display: grid;
    gap: 0.4rem;
  }

  .underlay-diagnostics-toolbar__content {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.76rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-diagnostics-toolbar__title {
    font-weight: 600;
    color: var(--underlay-color-text, #e2e8f0);
    margin-right: 0.15rem;
  }

  .underlay-diagnostics-toolbar__actions {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
  }
</style>
