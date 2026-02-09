import type { MediaKind, MediaVisibility } from "./enums";
import type { MediaDetail, MediaSummary, MediaVersion } from "./dto";

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
