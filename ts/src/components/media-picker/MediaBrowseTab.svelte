<script lang="ts">
  import type { MediaSummary } from "../../patterns/index.js";
  import Button from "../Button.svelte";
  import FormError from "../FormError.svelte";
  import PageLoading from "../PageLoading.svelte";
  import Image from "lucide-svelte/icons/image";
  import MediaBrowseItem from "./MediaBrowseItem.svelte";

  interface Props {
    loading: boolean;
    error: string | null;
    items: MediaSummary[];
    hasMore: boolean;
    onLoadMore: () => void;
    onSelectMedia: (media: MediaSummary) => void;
  }

  let {
    loading,
    error,
    items,
    hasMore,
    onLoadMore,
    onSelectMedia
  }: Props = $props();
</script>

<div class="underlay-browse-content">
  {#if loading && items.length === 0}
    <PageLoading message="Loading media..." />
  {:else if error}
    <FormError message={error} />
  {:else if items.length === 0}
    <div class="underlay-empty-state">
      <Image size={32} />
      <p>No media found</p>
    </div>
  {:else}
    <div class="underlay-media-grid">
      {#each items as item}
        <MediaBrowseItem {item} onSelect={onSelectMedia} />
      {/each}
    </div>

    {#if hasMore}
      <div class="underlay-load-more">
        <Button
          variant="secondary"
          onclick={onLoadMore}
          disabled={loading}
        >
          {loading ? "Loading..." : "Load more"}
        </Button>
      </div>
    {/if}
  {/if}
</div>

<style>
  .underlay-browse-content {
    min-height: 300px;
  }

  .underlay-media-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .underlay-empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 2rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-load-more {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
  }
</style>
