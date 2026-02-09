import { MediaKind, MediaVersionState, MediaVisibility, type MediaKind as MediaKindType, type MediaVersionState as MediaVersionStateType, type MediaVisibility as MediaVisibilityType } from "./enums";
import type { MediaDetail, MediaSummary } from "./dto";

/**
 * Get human-readable label for a media kind
 */
export function getMediaKindLabel(kind: MediaKindType): string {
  switch (kind) {
    case MediaKind.Image:
      return "Image";
    case MediaKind.Video:
      return "Video";
    case MediaKind.Audio:
      return "Audio";
    case MediaKind.Document:
      return "Document";
    case MediaKind.Pdf:
      return "PDF";
    case MediaKind.Other:
      return "Other";
    default:
      return kind;
  }
}

/**
 * Get accent color for a media kind (for pills/badges)
 */
export function getMediaKindAccent(kind: MediaKindType): string {
  switch (kind) {
    case MediaKind.Image:
      return "#22c55e";
    case MediaKind.Video:
      return "#f59e0b";
    case MediaKind.Audio:
      return "#8b5cf6";
    case MediaKind.Document:
      return "#3b82f6";
    case MediaKind.Pdf:
      return "#ef4444";
    case MediaKind.Other:
      return "#94a3b8";
    default:
      return "#94a3b8";
  }
}

/**
 * Get human-readable label for media visibility
 */
export function getMediaVisibilityLabel(visibility: MediaVisibilityType): string {
  switch (visibility) {
    case MediaVisibility.Public:
      return "Public";
    case MediaVisibility.Restricted:
      return "Restricted";
    default:
      return visibility;
  }
}

/**
 * Get accent color for media visibility (for pills/badges)
 */
export function getMediaVisibilityAccent(visibility: MediaVisibilityType): string {
  switch (visibility) {
    case MediaVisibility.Public:
      return "#3b82f6";
    case MediaVisibility.Restricted:
      return "#f59e0b";
    default:
      return "#94a3b8";
  }
}

/**
 * Get human-readable label for a version state
 */
export function getMediaVersionStateLabel(state: MediaVersionStateType): string {
  switch (state) {
    case MediaVersionState.Uploading:
      return "Uploading";
    case MediaVersionState.Ready:
      return "Ready";
    case MediaVersionState.Failed:
      return "Failed";
    case MediaVersionState.Purging:
      return "Purging";
    default:
      return state;
  }
}

/**
 * Get accent color for a version state (for pills/badges)
 */
export function getMediaVersionStateAccent(state: MediaVersionStateType): string {
  switch (state) {
    case MediaVersionState.Ready:
      return "#22c55e";
    case MediaVersionState.Uploading:
      return "#3b82f6";
    case MediaVersionState.Failed:
      return "#ef4444";
    case MediaVersionState.Purging:
      return "#f59e0b";
    default:
      return "#94a3b8";
  }
}

/**
 * Detect media kind from MIME type
 */
export function detectMediaKindFromMimeType(mimeType: string): MediaKindType {
  if (mimeType.startsWith("image/")) {
    return MediaKind.Image;
  }
  if (mimeType.startsWith("video/")) {
    return MediaKind.Video;
  }
  if (mimeType.startsWith("audio/")) {
    return MediaKind.Audio;
  }
  if (mimeType === "application/pdf") {
    return MediaKind.Pdf;
  }
  if (
    mimeType.includes("word") ||
    mimeType.includes("document") ||
    mimeType.includes("excel") ||
    mimeType.includes("spreadsheet") ||
    mimeType.includes("powerpoint") ||
    mimeType.includes("presentation") ||
    mimeType === "text/plain" ||
    mimeType === "text/csv" ||
    mimeType === "text/html" ||
    mimeType === "text/markdown"
  ) {
    return MediaKind.Document;
  }

  return MediaKind.Other;
}

/**
 * Check if a media item is soft-deleted
 */
export function isMediaDeleted(media: MediaSummary | MediaDetail): boolean {
  return media.deletedAt !== null;
}

/**
 * Get display name for a media item (title or filename fallback)
 */
export function getMediaDisplayName(media: MediaSummary | MediaDetail): string {
  return media.title || media.originalFilename || "Untitled";
}
