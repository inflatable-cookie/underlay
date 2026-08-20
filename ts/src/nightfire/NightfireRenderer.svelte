<script lang="ts">
  import { getBlockRenderer } from "./render-registry";
  import "./render-registrations";
  import type { NightfireDraftValue } from "./types";

  type BlockLike = {
    type?: string;
    data?: Record<string, unknown>;
  };

  interface Props {
    value?: NightfireDraftValue | null;
  }

  let { value = null }: Props = $props();

  function normalizeBlocks(v: NightfireDraftValue | null): BlockLike[] {
    if (!v) return [];
    if (Array.isArray(v.blocks) && v.blocks.length > 0) {
      return v.blocks as BlockLike[];
    }
    return [];
  }

  const blocks = $derived(normalizeBlocks(value));
</script>

{#if blocks.length === 0}
  <!-- No Nightfire content -->
{:else}
  {#each blocks as block}
    {#if block?.type}
      {#key `${value?.schema ?? ""}:${block.type}`}
        {#if getBlockRenderer(value?.schema, block.type)}
          {@const BlockRenderer = getBlockRenderer(value?.schema, block.type)}
          <BlockRenderer
            {block}
            {value}
          />
        {/if}
      {/key}
    {/if}
  {/each}
{/if}
