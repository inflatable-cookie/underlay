<script lang="ts">
  import { AlertDialog, Icon, MediaThumbnail, TimeAgo, formatFileSize } from "@poodle/svelte";
  import { gotoWithContext } from "../client/navigation";
  import {
    getMediaKindAccent,
    getMediaKindLabel,
    getMediaVisibilityLabel,
    MediaKind,
    MediaVisibility,
    type MediaKind as MediaKindType,
    type MediaVisibility as MediaVisibilityType
  } from "../runtime/media";
  import { default as EntityListCard } from "./EntityListCard.svelte";
  import type { EntityListCardBadge } from "./entity-list-card.types";

  interface MediaListCardItem {
    id: string;
    kind: string;
    visibility?: string | null;
    title?: string | null;
    originalFilename?: string | null;
    mimeType?: string | null;
    thumbnailUrl?: string | null;
    originalUrl?: string | null;
    byteSize?: number | null;
    updatedAt?: string | null;
  }

  interface Props {
    media: MediaListCardItem;
    selectionMode?: boolean;
    reorderMode?: boolean;
    selected?: boolean;
    listHref?: string;
    detailHref?: (media: MediaListCardItem) => string;
    contextLabel?: string;
    onOpen?: (media: MediaListCardItem) => void;
    onSelectionChange?: (mediaId: string, selected: boolean) => void;
    onDelete?: (mediaId: string) => void;
    onCopyId?: (mediaId: string) => void | Promise<void>;
  }

  let {
    media,
    selectionMode = false,
    reorderMode = false,
    selected = false,
    listHref = "/media",
    detailHref = (entry) => `/media/${entry.id}`,
    contextLabel = "Media",
    onOpen,
    onSelectionChange,
    onDelete,
    onCopyId
  }: Props = $props();

  let confirmDeleteOpen = $state(false);
  let failedThumbnailUrl = $state<string | null>(null);

  const normalizedKind = $derived(normalizeMediaKind(media.kind));
  const normalizedVisibility = $derived(normalizeMediaVisibility(media.visibility ?? null));
  const title = $derived(media.title ?? media.originalFilename ?? "Untitled");
  const isSvg = $derived(isSvgMedia(media));
  const rawThumbnailUrl = $derived(
    media.thumbnailUrl?.trim() || (isSvg ? media.originalUrl?.trim() : null) || null
  );
  const previewImageUrl = $derived(
    rawThumbnailUrl && rawThumbnailUrl !== failedThumbnailUrl ? rawThumbnailUrl : null
  );
  const subtitle = $derived(
    media.title && media.originalFilename && media.originalFilename !== media.title
      ? media.originalFilename
      : null
  );
  const badges = $derived<EntityListCardBadge[]>([
    { label: getMediaKindLabel(normalizedKind), accent: getMediaKindAccent(normalizedKind) }
  ]);
  const fileSizeText = $derived(media.byteSize ? formatFileSize(media.byteSize) : null);
  const menuItems = $derived([
    { value: "copy-id", label: "Copy media ID" },
    ...(onDelete
      ? [
          { value: "separator", label: "", kind: "separator" as const },
          { value: "delete", label: "Move to trash", tone: "danger" as const }
        ]
      : [])
  ]);
  const thumbnailKind = $derived(toThumbnailKind(normalizedKind));
  const leadingIcon = $derived(getLeadingIcon(normalizedKind));

  function normalizeMediaKind(kind: string): MediaKindType {
    if (kind === MediaKind.Image) return MediaKind.Image;
    if (kind === MediaKind.Audio) return MediaKind.Audio;
    if (kind === MediaKind.Video) return MediaKind.Video;
    if (kind === MediaKind.Pdf) return MediaKind.Pdf;
    if (kind === MediaKind.Document) return MediaKind.Document;
    return MediaKind.Other;
  }

  function normalizeMediaVisibility(visibility: string | null): MediaVisibilityType {
    if (visibility === MediaVisibility.Restricted) return MediaVisibility.Restricted;
    return MediaVisibility.Public;
  }

  function toThumbnailKind(kind: MediaKindType): "image" | "audio" | "video" | "document" | "embed" {
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

  function isSvgMedia(item: MediaListCardItem): boolean {
    return (
      item.mimeType === "image/svg+xml" ||
      item.originalFilename?.toLowerCase().endsWith(".svg") === true
    );
  }

  function handleOpen(): void {
    if (onOpen) {
      onOpen(media);
      return;
    }

    void gotoWithContext(detailHref(media), {
      label: contextLabel,
      href: listHref,
      type: "list"
    });
  }

  function handleDelete(): void {
    onDelete?.(media.id);
    confirmDeleteOpen = false;
  }

  function handleContextAction(value: string): void {
    if (value === "copy-id") {
      void onCopyId?.(media.id);
      return;
    }

    if (value === "delete") {
      confirmDeleteOpen = true;
    }
  }
</script>

{#snippet mediaLeading()}
  <MediaThumbnail
    kind={thumbnailKind}
    presentation="default"
    aspectRatio="square"
    ariaLabel={title}
  >
    <img
      src={previewImageUrl}
      alt={media.title ?? ""}
      class="underlay-media-list-card__thumbnail"
      onerror={() => {
        failedThumbnailUrl = rawThumbnailUrl;
      }}
    />
  </MediaThumbnail>
{/snippet}

{#snippet mediaFooter()}
  {#if fileSizeText}
    <span>{fileSizeText}</span>
  {/if}

  {#if fileSizeText && media.updatedAt}
    <span aria-hidden="true">·</span>
  {/if}

  {#if media.updatedAt}
    <TimeAgo datetime={media.updatedAt} typography="inherit" />
  {/if}
{/snippet}

{#snippet visibilityCorner()}
  <span title={getMediaVisibilityLabel(normalizedVisibility)}>
    <Icon icon="lock" ariaLabel={getMediaVisibilityLabel(normalizedVisibility)} size="sm" />
  </span>
{/snippet}

<EntityListCard
  title={title}
  {subtitle}
  leadingShape="rounded-square"
  {reorderMode}
  selectionMode={selectionMode}
  {selected}
  badges={badges}
  corner={normalizedVisibility !== MediaVisibility.Public ? visibilityCorner : undefined}
  footer={fileSizeText || media.updatedAt ? mediaFooter : undefined}
  leading={previewImageUrl ? mediaLeading : undefined}
  {leadingIcon}
  contextMenuItems={selectionMode || reorderMode ? [] : menuItems}
  contextMenuAriaLabel="Media actions"
  contextMenuTrigger="leading"
  onSelectionChange={(nextSelected) => onSelectionChange?.(media.id, nextSelected)}
  onContextAction={handleContextAction}
  onClick={selectionMode || reorderMode ? undefined : handleOpen}
/>

{#if confirmDeleteOpen}
  <AlertDialog
    open={confirmDeleteOpen}
    title="Move media to trash"
    description={`Are you sure you want to move "${title}" to trash? You can restore it later.`}
    confirmLabel="Move to trash"
    onConfirm={handleDelete}
    onCancel={() => {
      confirmDeleteOpen = false;
    }}
    tone="danger"
  />
{/if}

<style>
  .underlay-media-list-card__thumbnail {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
</style>
