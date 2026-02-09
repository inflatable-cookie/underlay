/**
 * Type of media item
 */
export const MediaKind = {
  Image: "image",
  Video: "video",
  Audio: "audio",
  Document: "document",
  Pdf: "pdf",
  Other: "other"
} as const;

export type MediaKind = (typeof MediaKind)[keyof typeof MediaKind];

/**
 * Visibility/access level of media
 */
export const MediaVisibility = {
  Public: "public",
  Restricted: "restricted"
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
  Purging: "purging"
} as const;

export type MediaVersionState =
  (typeof MediaVersionState)[keyof typeof MediaVersionState];
