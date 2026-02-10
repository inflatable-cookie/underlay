<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon?: any;
    class?: string;
    controls?: Snippet;
    children?: Snippet;
  }

  let {
    title,
    icon: Icon,
    class: className,
    controls,
    children,
  }: Props = $props();
</script>

<section class={`underlay-ops-section ${className ?? ""}`}>
  <div class="underlay-ops-section__header">
    <h2 class="underlay-ops-section__title">
      {#if Icon}
        {@const IconComponent = Icon}
        <IconComponent size={16} aria-hidden="true" />
      {/if}
      {title}
    </h2>
    {#if controls}
      <div class="underlay-ops-section__controls">{@render controls()}</div>
    {/if}
  </div>
  <div class="underlay-ops-section__content">{@render children?.()}</div>
</section>

<style>
  .underlay-ops-section {
    margin-top: 1rem;
  }

  .underlay-ops-section__header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: space-between;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .underlay-ops-section__title {
    margin: 0;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 1rem;
  }

  .underlay-ops-section__controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: min(100%, 16rem);
  }

  .underlay-ops-section__content {
    min-width: 0;
  }
</style>
