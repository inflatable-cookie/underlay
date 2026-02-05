/**
 * Media Library types for Underlay.
 *
 * Generic types for media items, versions, usage tracking, and upload operations.
 * These types are designed to be shared across consuming applications.
 */

// ============================================================================
// Enums
// ============================================================================

/**
 * Type of media item
 */
export const MediaKind = {
  Image: "image",
  Video: "video",
  Audio: "audio",
  Document: "document",
  Pdf: "pdf",
  Other: "other",
} as const;

export type MediaKind = (typeof MediaKind)[keyof typeof MediaKind];

/**
 * Visibility/access level of media
 */
export const MediaVisibility = {
  Public: "public",
  Restricted: "restricted",
} as const;

export type MediaVisibility =
  (typeof MediaVisibility)[keyof typeof MediaVisibility];

/**
 * State of a media version
 */
export const MediaVersionState = {
  Uploading: "uploading",
  Ready: "ready",
  Failed: "failed",
  Purging: "purging",
} as const;

export type MediaVersionState =
  (typeof MediaVersionState)[keyof typeof MediaVersionState];

// ============================================================================
// Media DTOs
// ============================================================================

/**
 * Media item summary for list views
 */
export interface MediaSummary {
  id: string;
  kind: MediaKind;
  visibility: MediaVisibility;
  title: string | null;
  originalFilename: string | null;
  currentVersionId: string | null;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
  /** Byte size of current version (if available) */
  byteSize: number | null;
  /** MIME type of current version (if available) */
  mimeType: string | null;
  /** URL to thumbnail image (if available) */
  thumbnailUrl: string | null;
}

/**
 * Full media item detail
 */
export interface MediaDetail {
  id: string;
  kind: MediaKind;
  visibility: MediaVisibility;
  title: string | null;
  originalFilename: string | null;
  currentVersionId: string | null;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
  createdBy: string | null;
  /** Current version details (if available) */
  currentVersion: MediaVersion | null;
  /** Number of places this media is used */
  usageCount: number;
}

/**
 * Media version (immutable blob record)
 */
export interface MediaVersion {
  id: string;
  mediaId: string;
  state: MediaVersionState;
  storageProvider: string | null;
  bucket: string | null;
  objectKey: string | null;
  byteSize: number | null;
  mimeType: string | null;
  sha256: string | null;
  createdAt: string;
  createdBy: string | null;
  /** URL to the original file (if available) */
  url: string | null;
  /** Renditions (thumbnails, etc.) for this version */
  renditions: MediaRendition[];
}

/**
 * Media rendition (derived blob, e.g., thumbnail)
 */
export interface MediaRendition {
  id: string;
  versionId: string;
  kind: string;
  storageProvider: string | null;
  bucket: string | null;
  objectKey: string | null;
  byteSize: number | null;
  mimeType: string | null;
  width: number | null;
  height: number | null;
  createdAt: string;
  /** URL to the rendition (if available) */
  url: string | null;
}

/**
 * Media usage record (where media is referenced)
 */
export interface MediaUsage {
  id: string;
  mediaId: string;
  usedByType: string;
  usedById: string;
  field: string | null;
  createdAt: string;
}

// ============================================================================
// Request/Response Types
// ============================================================================

/**
 * Request to create a new media item
 */
export interface CreateMediaRequest {
  kind: MediaKind;
  visibility: MediaVisibility;
  title?: string | null;
  originalFilename?: string | null;
}

/**
 * Request to update a media item
 */
export interface UpdateMediaRequest {
  title?: string | null;
  /** Original filename (used for Content-Disposition on downloads) */
  originalFilename?: string | null;
  visibility?: MediaVisibility;
}

/**
 * Request to check for duplicate files
 */
export interface CheckDuplicateRequest {
  sha256: string;
}

/**
 * Response from duplicate check
 */
export interface CheckDuplicateResponse {
  exists: boolean;
  media: MediaSummary | null;
}

/**
 * Request to initiate an upload
 */
export interface InitiateUploadRequest {
  contentType: string;
  contentLength: number;
  sha256?: string | null;
}

/**
 * Response from initiating an upload
 */
export interface InitiateUploadResponse {
  versionId: string;
  uploadPlan: MediaUploadPlan;
}

/**
 * Upload plan returned by media API.
 *
 * Note: To use with `uploadToBlob()`, map `headers` to `requiredHeaders`.
 */
export interface MediaUploadPlan {
  uploadUrl: string;
  method: string;
  headers: Record<string, string>;
  expiresAt: string;
  maxBytes: number | null;
  allowedContentTypes: string[] | null;
}

/**
 * Request to finalise an upload
 */
export interface FinaliseUploadRequest {
  sha256: string;
  contentType: string;
}

/**
 * Response from finalising an upload
 */
export interface FinaliseUploadResponse {
  version: MediaVersion;
  media: MediaDetail;
}

// ============================================================================
// Query Parameters
// ============================================================================

/**
 * Query parameters for listing media
 */
export interface MediaListQuery {
  /** Search by title */
  q?: string;
  /** Filter by kind */
  kind?: MediaKind;
  /** Filter by visibility */
  visibility?: MediaVisibility;
  /** Include soft-deleted items */
  includeDeleted?: boolean;
  /** Only show unused media */
  unusedOnly?: boolean;
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Get human-readable label for a media kind
 */
export function getMediaKindLabel(kind: MediaKind): string {
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
export function getMediaKindAccent(kind: MediaKind): string {
  switch (kind) {
    case MediaKind.Image:
      return "#22c55e"; // green
    case MediaKind.Video:
      return "#f59e0b"; // amber
    case MediaKind.Audio:
      return "#8b5cf6"; // violet
    case MediaKind.Document:
      return "#3b82f6"; // blue
    case MediaKind.Pdf:
      return "#ef4444"; // red
    case MediaKind.Other:
      return "#94a3b8"; // gray
    default:
      return "#94a3b8"; // gray
  }
}

/**
 * Get human-readable label for media visibility
 */
export function getMediaVisibilityLabel(visibility: MediaVisibility): string {
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
export function getMediaVisibilityAccent(visibility: MediaVisibility): string {
  switch (visibility) {
    case MediaVisibility.Public:
      return "#3b82f6"; // blue
    case MediaVisibility.Restricted:
      return "#f59e0b"; // amber
    default:
      return "#94a3b8"; // gray
  }
}

/**
 * Get human-readable label for a version state
 */
export function getMediaVersionStateLabel(state: MediaVersionState): string {
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
export function getMediaVersionStateAccent(state: MediaVersionState): string {
  switch (state) {
    case MediaVersionState.Ready:
      return "#22c55e"; // green
    case MediaVersionState.Uploading:
      return "#3b82f6"; // blue
    case MediaVersionState.Failed:
      return "#ef4444"; // red
    case MediaVersionState.Purging:
      return "#f59e0b"; // amber
    default:
      return "#94a3b8"; // gray
  }
}

/**
 * Detect media kind from MIME type
 */
export function detectMediaKindFromMimeType(mimeType: string): MediaKind {
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
  // Common document types
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

// ============================================================================
// Icon Utilities
// ============================================================================

// Import lucide icons for media kinds
import ImageIcon from "lucide-svelte/icons/image";
import FileTextIcon from "lucide-svelte/icons/file-text";
import VideoIcon from "lucide-svelte/icons/video";
import MusicIcon from "lucide-svelte/icons/music";
import FileIcon from "lucide-svelte/icons/file";
import type { Component } from "svelte";

/**
 * Icon component type (lucide-svelte icons)
 */
export type IconComponent = Component<{ size?: number | string; class?: string }>;

/**
 * Get the appropriate icon component for a media kind.
 *
 * @param kind - The media kind
 * @returns A Svelte component for the icon
 *
 * @example
 * ```svelte
 * <script>
 *   import { getMediaKindIcon, MediaKind } from "@decodelabs/underlay/patterns";
 *   const Icon = getMediaKindIcon(MediaKind.Image);
 * </script>
 *
 * <Icon size={24} />
 * ```
 */
export function getMediaKindIcon(kind: MediaKind): IconComponent {
  switch (kind) {
    case MediaKind.Image:
      return ImageIcon as unknown as IconComponent;
    case MediaKind.Video:
      return VideoIcon as unknown as IconComponent;
    case MediaKind.Audio:
      return MusicIcon as unknown as IconComponent;
    case MediaKind.Pdf:
      return FileTextIcon as unknown as IconComponent;
    case MediaKind.Document:
      return FileTextIcon as unknown as IconComponent;
    case MediaKind.Other:
    default:
      return FileIcon as unknown as IconComponent;
  }
}
