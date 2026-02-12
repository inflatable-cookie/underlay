<script lang="ts">
  import {
    getMediaDisplayName,
    getMediaKindLabel,
    type MediaSummary
  } from "../../patterns/index.js";
  import MediaThumbnail from "../MediaThumbnail.svelte";

  interface Props {
    item: MediaSummary;
    onSelect: (media: MediaSummary) => void;
  }

  let {
    item,
    onSelect
  }: Props = $props();
</script>

<button
  type="button"
  class="underlay-media-item"
  onclick={() => onSelect(item)}
>
  <MediaThumbnail
    thumbnailUrl={item.thumbnailUrl}
    kind={item.kind}
    alt={getMediaDisplayName(item)}
    size={48}
    showAccent
  />
  <div class="underlay-media-item__info">
    <span class="underlay-media-item__title"
      >{getMediaDisplayName(item)}</span
    >
    <span class="underlay-media-item__meta"
      >{getMediaKindLabel(item.kind)}</span
    >
  </div>
</button>

<style>
  .underlay-media-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem;
    border: 1px solid var(--underlay-color-border, #374151);
    border-radius: 0.5rem;
    background: var(--underlay-color-surface, #1f2937);
    cursor: pointer;
    transition:
      border-color 0.15s,
      background-color 0.15s;
    text-align: center;
  }

  .underlay-media-item:hover {
    border-color: var(--underlay-color-primary, #3b82f6);
    background: var(--underlay-color-surface-raised, #374151);
  }

  .underlay-media-item__info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .underlay-media-item__title {
    font-size: 0.875rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 150px;
  }

  .underlay-media-item__meta {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
