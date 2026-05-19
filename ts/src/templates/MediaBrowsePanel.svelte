<script lang="ts">
  import { Button, Callout, MediaThumbnail } from "@poodle/svelte";
  import type { MediaPickerBrowseItem } from "./template.types";

  interface Props {
    loading?: boolean;
    error?: string | null;
    items?: MediaPickerBrowseItem[];
    hasMore?: boolean;
    emptyMessage?: string;
    loadMoreLabel?: string;
    onSelect?: (item: MediaPickerBrowseItem) => void;
    onLoadMore?: () => void;
  }

  let {
    loading = false,
    error = null,
    items = [],
    hasMore = false,
    emptyMessage = "No media found",
    loadMoreLabel = "Load more",
    onSelect,
    onLoadMore
  }: Props = $props();

  function toMediaKind(kind?: string | null): "image" | "audio" | "video" | "document" | "embed" {
    switch (kind) {
      case "image":
      case "audio":
      case "video":
      case "embed":
        return kind;
      default:
        return "document";
    }
  }
</script>

<div class="underlay-media-browse-panel">
  {#if loading && items.length === 0}
    <div class="underlay-media-browse-panel__state">
      <p>Loading media...</p>
    </div>
  {:else if error}
    <Callout tone="danger" message={error} announceMode="polite" />
  {:else if items.length === 0}
    <div class="underlay-media-browse-panel__state">
      <p>{emptyMessage}</p>
    </div>
  {:else}
    <div class="underlay-media-browse-panel__grid">
      {#each items as item (item.id)}
        <button
          type="button"
          class="underlay-media-browse-panel__item"
          onclick={() => onSelect?.(item)}
        >
          <MediaThumbnail
            kind={toMediaKind(item.kind)}
            presentation="compact"
            aspectRatio="square"
            ariaLabel={item.label}
          >
            {#if item.thumbnailUrl}
              <img
                src={item.thumbnailUrl}
                alt={item.label}
                class="underlay-media-browse-panel__image"
              />
            {/if}
          </MediaThumbnail>
          <span class="underlay-media-browse-panel__label">{item.label}</span>
          {#if item.meta}
            <span class="underlay-media-browse-panel__meta">{item.meta}</span>
          {/if}
        </button>
      {/each}
    </div>

    {#if hasMore}
      <div class="underlay-media-browse-panel__actions">
        <Button variant="secondary" onClick={() => onLoadMore?.()} disabled={loading}>
          {loading ? "Loading..." : loadMoreLabel}
        </Button>
      </div>
    {/if}
  {/if}
</div>

<style>
  .underlay-media-browse-panel {
    --underlay-media-browse-grid-gap: var(--poodle-space-stack-sm);
    --underlay-media-browse-item-gap: 0.375rem;
    --underlay-media-browse-item-pad: 0.75rem;
    --underlay-media-browse-grid-min: 11rem;
    width: 100%;
    box-sizing: border-box;
    min-height: 18rem;
  }

  .underlay-media-browse-panel__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--underlay-media-browse-grid-min), 1fr));
    gap: var(--underlay-media-browse-grid-gap);
    margin-top: var(--underlay-media-browse-grid-gap);
  }

  .underlay-media-browse-panel__item {
    display: grid;
    gap: var(--underlay-media-browse-item-gap);
    padding: var(--underlay-media-browse-item-pad);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
    color: inherit;
    text-align: left;
    cursor: pointer;
  }

  .underlay-media-browse-panel__item:hover,
  .underlay-media-browse-panel__item:focus-visible {
    border-color: var(--poodle-color-border-focus);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 90%, transparent);
    outline: none;
  }

  .underlay-media-browse-panel__image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .underlay-media-browse-panel__label {
    font-size: var(--poodle-typography-body-size);
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-media-browse-panel__meta,
  .underlay-media-browse-panel__state p {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.45;
  }

  .underlay-media-browse-panel__actions {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: center;
    margin-top: var(--underlay-media-browse-grid-gap);
  }

  .underlay-media-browse-panel__state {
    display: grid;
    place-items: center;
    min-height: 18rem;
    text-align: center;
  }
</style>
