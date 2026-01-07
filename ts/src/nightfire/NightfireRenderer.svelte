<script lang="ts">
  import { getBlockRenderer } from "./render-registry";
  import "./render-registrations";
  import type { NightfireValue } from "./index";

  type BlockLike = {
    type?: string;
    data?: Record<string, unknown>;
  };

  export let value: NightfireValue | null = null;

  function normalizeBlocks(v: NightfireValue | null): BlockLike[] {
    if (!v) return [];
    if (Array.isArray(v.blocks) && v.blocks.length > 0) {
      return v.blocks as BlockLike[];
    }
    if (v.block) {
      return [v.block as BlockLike];
    }
    return [];
  }

  $: blocks = normalizeBlocks(value);
</script>

{#if blocks.length === 0}
  <!-- No Nightfire content -->
{:else}
  {#each blocks as block}
    {#if block?.type}
      {#key `${value?.schema ?? ""}:${block.type}`}
        {#if getBlockRenderer(value?.schema, block.type)}
          <svelte:component
            this={getBlockRenderer(value?.schema, block.type)}
            {block}
            {value}
          />
        {/if}
      {/key}
    {/if}
  {/each}
{/if}
