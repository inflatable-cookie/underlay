import type { MediaKind, MediaVersionState, MediaVisibility } from "./enums";

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
  /** Number of places this media is used */
  usageCount: number;
  /** URL to thumbnail image (if available) */
  thumbnailUrl: string | null;
  /** URL to original file when it can be previewed directly */
  originalUrl?: string | null;
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
  updatedBy: string | null;
  deletedBy: string | null;
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
  ownerField: string | null;
  contentKind: string;
  locatorKind: string;
  locatorKey: string;
  usageRole: string;
  provenanceKind: string;
  createdAt: string;
}
