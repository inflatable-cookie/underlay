<script lang="ts">
  import Image from "lucide-svelte/icons/image";
  import FileText from "lucide-svelte/icons/file-text";
  import {
    MediaKind,
    getMediaDisplayName,
    getMediaKindAccent,
    getMediaKindLabel,
    type MediaSummary
  } from "../../patterns/index.js";

  interface Props {
    item: MediaSummary;
    onSelect: (media: MediaSummary) => void;
  }

  let {
    item,
    onSelect
  }: Props = $props();

  const Icon = $derived(item.kind === MediaKind.Image ? Image : FileText);
</script>

<button
  type="button"
  class="media-item"
  onclick={() => onSelect(item)}
>
  <div
    class="media-item__icon"
    style="color: {getMediaKindAccent(item.kind)}"
  >
    <Icon size={24} />
  </div>
  <div class="media-item__info">
    <span class="media-item__title"
      >{getMediaDisplayName(item)}</span
    >
    <span class="media-item__meta"
      >{getMediaKindLabel(item.kind)}</span
    >
  </div>
</button>

<style>
  .media-item {
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

  .media-item:hover {
    border-color: var(--underlay-color-primary, #3b82f6);
    background: var(--underlay-color-surface-raised, #374151);
  }

  .media-item__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: 0.375rem;
    background: var(--underlay-color-surface-raised, #374151);
  }

  .media-item__info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .media-item__title {
    font-size: 0.875rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 150px;
  }

  .media-item__meta {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
