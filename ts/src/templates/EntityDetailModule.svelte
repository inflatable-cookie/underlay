<script lang="ts">
  import type { Snippet } from "svelte";
  import { Card } from "@inflatable-cookie/poodle-svelte";

  interface Props {
    span?: "half" | "full";
    measure?: boolean;
    children?: Snippet;
  }

  let {
    span = "half",
    measure = false,
    children
  }: Props = $props();
</script>

<div
  class="underlay-entity-detail-module"
  data-span={span}
  data-measure={measure}
>
  <Card class="underlay-entity-detail-module__card">
    {#if children}
      <div class="underlay-entity-detail-module__body">
        {@render children()}
      </div>
    {/if}
  </Card>
</div>

<style>
  .underlay-entity-detail-module {
    min-width: 0;
    height: 100%;
  }

  .underlay-entity-detail-module[data-span="full"] {
    grid-column: 1 / -1;
  }

  :global(.underlay-entity-detail-module__card) {
    height: 100%;
  }

  .underlay-entity-detail-module__body {
    height: 100%;
  }

  .underlay-entity-detail-module__body {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .underlay-entity-detail-module[data-measure="true"] {
    max-width: 46rem;
  }

  .underlay-entity-detail-module[data-measure="true"][data-span="full"] {
    width: min(100%, 46rem);
  }

  @media (max-width: 64rem) {
    .underlay-entity-detail-module[data-measure="true"] {
      max-width: none;
      width: auto;
    }
  }
</style>
