<script lang="ts">
  import { EmptyState } from "@inflatable-cookie/poodle-svelte";

  interface MediaPreviewModel {
    kind: string;
    title?: string | null;
    originalFilename?: string | null;
  }

  interface Props {
    media: MediaPreviewModel | null;
    mediaUrl: string | null;
    isImage: (kind: string) => boolean;
    isPdf: (kind: string) => boolean;
    emptyTitle?: string;
    emptyMessage?: string | null;
  }

  let {
    media,
    mediaUrl,
    isImage,
    isPdf,
    emptyTitle = "Preview not available",
    emptyMessage = "Preview is not available for this version."
  }: Props = $props();
</script>

<div class="underlay-media-preview-tab">
  {#if mediaUrl && media}
    {#if isImage(media.kind)}
      <img
        src={mediaUrl}
        alt={media.title || media.originalFilename || "Media preview"}
        class="underlay-media-preview-tab__image"
      />
    {:else if isPdf(media.kind)}
      <iframe
        src={mediaUrl}
        title={media.title || media.originalFilename || "PDF preview"}
        class="underlay-media-preview-tab__pdf"
        sandbox=""
      ></iframe>
    {:else}
      <EmptyState title={emptyTitle} message={emptyMessage} size="compact" />
    {/if}
  {:else}
    <EmptyState title={emptyTitle} message={emptyMessage} size="compact" />
  {/if}
</div>

<style>
  .underlay-media-preview-tab {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding: 1.5rem;
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border-radius: 0.5rem;
    min-height: 400px;
  }

  .underlay-media-preview-tab__image {
    max-width: 100%;
    max-height: 80vh;
    object-fit: contain;
    border-radius: 0.25rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .underlay-media-preview-tab__pdf {
    width: 100%;
    height: 80vh;
    border: 0;
    border-radius: 0.25rem;
    background: white;
  }
</style>
