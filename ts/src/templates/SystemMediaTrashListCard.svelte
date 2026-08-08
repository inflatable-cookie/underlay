<script lang="ts">
  import {
    AlertDialog,
    Button,
    MediaThumbnail,
    formatFileSize
  } from "@inflatable-cookie/poodle-svelte";
  import RotateCcw from "lucide-svelte/icons/rotate-ccw";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import {
    getMediaKindAccent,
    getMediaKindLabel,
    MediaKind,
    type MediaKind as MediaKindType
  } from "../patterns/media-types";
  import { default as EntityListCard } from "./EntityListCard.svelte";
  import type { EntityListCardBadge } from "./entity-list-card.types";
  import type { SystemMediaTrashItem } from "./template.types";

  interface Props<TMedia extends SystemMediaTrashItem = SystemMediaTrashItem> {
    media: TMedia;
    href?: string | null;
    onRestore?: (media: TMedia) => void;
    onPurge?: (media: TMedia) => void;
    onClick?: () => void;
  }

  let {
    media,
    href = null,
    onRestore,
    onPurge,
    onClick
  }: Props = $props();

  let confirmPurgeOpen = $state(false);
  let failedThumbnailUrl = $state<string | null>(null);

  const title = $derived(media.title ?? media.originalFilename ?? "Untitled");
  const isSvg = $derived(isSvgMedia(media));
  const rawThumbnailUrl = $derived(
    media.thumbnailUrl?.trim() || (isSvg ? media.originalUrl?.trim() : null) || null
  );
  const previewImageUrl = $derived(
    rawThumbnailUrl && rawThumbnailUrl !== failedThumbnailUrl ? rawThumbnailUrl : null
  );
  const normalizedKind = $derived(normalizeMediaKind(media.kind));
  const badges = $derived<EntityListCardBadge[]>([
    {
      label: getMediaKindLabel(normalizedKind),
      accent: getMediaKindAccent(normalizedKind)
    },
    {
      label: "deleted",
      accent: "#ef4444",
      appearance: "badge",
      size: "sm"
    }
  ]);
  const footerText = $derived(
    [
      media.byteSize ? formatFileSize(media.byteSize) : null,
      media.deletedAt ? `Deleted ${new Date(media.deletedAt).toLocaleDateString()}` : null
    ]
      .filter(Boolean)
      .join(" · ")
  );
  const thumbnailKind = $derived(toPoodleMediaKind(normalizedKind));
  const leadingIcon = $derived(getLeadingIcon(normalizedKind));

  function normalizeMediaKind(kind: string): MediaKindType {
    if (kind === MediaKind.Image) return MediaKind.Image;
    if (kind === MediaKind.Audio) return MediaKind.Audio;
    if (kind === MediaKind.Video) return MediaKind.Video;
    if (kind === MediaKind.Pdf) return MediaKind.Pdf;
    if (kind === MediaKind.Document) return MediaKind.Document;
    return MediaKind.Other;
  }

  function toPoodleMediaKind(kind: MediaKindType): "image" | "audio" | "video" | "document" | "embed" {
    if (kind === MediaKind.Image) return "image";
    if (kind === MediaKind.Audio) return "audio";
    if (kind === MediaKind.Video) return "video";
    return "document";
  }

  function getLeadingIcon(kind: MediaKindType): string {
    if (kind === MediaKind.Image) return "image";
    if (kind === MediaKind.Audio) return "music";
    if (kind === MediaKind.Video) return "video";
    return "file-text";
  }

  function isSvgMedia(item: SystemMediaTrashItem): boolean {
    return (
      item.mimeType === "image/svg+xml" ||
      item.originalFilename?.toLowerCase().endsWith(".svg") === true
    );
  }
</script>

{#snippet mediaLeading()}
  <MediaThumbnail
    kind={thumbnailKind}
    presentation="compact"
    aspectRatio="square"
    ariaLabel={title}
  >
    <img
      src={previewImageUrl}
      alt={media.title ?? ""}
      class="underlay-system-media-trash-card__thumbnail"
      onerror={() => {
        failedThumbnailUrl = rawThumbnailUrl;
      }}
    />
  </MediaThumbnail>
{/snippet}

{#snippet mediaFooter()}
  <div class="underlay-system-media-trash-card__actions">
    <Button type="button" variant="ghost" size="sm" onClick={() => onRestore?.(media)}>
      <RotateCcw size={14} />
      Restore
    </Button>
    <Button
      type="button"
      variant="ghost"
      tone="danger"
      size="sm"
      onClick={() => {
        confirmPurgeOpen = true;
      }}
    >
      <Trash2 size={14} />
      Delete
    </Button>
  </div>
{/snippet}

<EntityListCard
  title={title}
  {href}
  accentColor="#64748b"
  {badges}
  {footerText}
  leading={previewImageUrl ? mediaLeading : undefined}
  {leadingIcon}
  footer={mediaFooter}
  {onClick}
/>

{#if confirmPurgeOpen}
  <AlertDialog
    open={confirmPurgeOpen}
    title="Permanently delete media"
    description={`Are you sure you want to permanently delete "${title}"? This action cannot be undone.`}
    confirmLabel="Delete forever"
    onConfirm={() => {
      onPurge?.(media);
      confirmPurgeOpen = false;
    }}
    onCancel={() => {
      confirmPurgeOpen = false;
    }}
    tone="danger"
  />
{/if}

<style>
  .underlay-system-media-trash-card__thumbnail {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .underlay-system-media-trash-card__actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
