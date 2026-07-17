/**
 * Blob storage types for direct-to-S3 uploads.
 *
 * These types mirror the Rust types in underlay-blob and provide
 * client-side utilities for uploading files directly to blob storage.
 */

/**
 * Upload plan returned by the server after initiating an upload.
 * Contains the pre-signed URL and constraints for the upload.
 */
export interface UploadPlan {
  /** The URL to upload to (pre-signed PUT URL for S3, or direct endpoint for local) */
  uploadUrl: string;
  /** HTTP method to use (typically "PUT") */
  method: string;
  /** Required headers that must be included in the upload request */
  requiredHeaders: Record<string, string>;
  /** Maximum allowed file size in bytes */
  maxBytes: number;
  /** Allowed content types */
  allowedContentTypes: string[];
  /** When the upload URL expires (ISO 8601) */
  expiresAt: string;
  /** The final object key (for reference) */
  objectKey: string;
}

/**
 * Progress information during upload.
 */
export interface UploadProgress {
  /** Bytes uploaded so far */
  loaded: number;
  /** Total bytes to upload */
  total: number;
  /** Progress as a percentage (0-100) */
  percent: number;
}

/**
 * Result of a successful upload.
 */
export interface UploadResult {
  /** The object key that was uploaded */
  objectKey: string;
  /** Size of the uploaded file in bytes */
  size: number;
  /** Content type of the uploaded file */
  contentType: string;
}

/**
 * Options for the upload function.
 */
export interface UploadOptions {
  /** Callback for progress updates */
  onProgress?: (progress: UploadProgress) => void;
  /** AbortSignal to cancel the upload */
  signal?: AbortSignal;
}

/**
 * Reference to a stored object with its location.
 */
export interface StoredObject {
  /** The storage provider name (e.g., "s3", "local") */
  provider: string;
  /** The bucket or container name */
  bucket: string;
  /** The object key (path) */
  key: string;
  /** Size in bytes */
  size: number;
  /** Content type (MIME type) */
  contentType: string;
  /** ETag if available */
  etag?: string;
}

/**
 * Error thrown during upload operations.
 */
export class BlobUploadError extends Error {
  constructor(
    message: string,
    public readonly code: BlobErrorCode,
    public readonly statusCode?: number
  ) {
    super(message);
    this.name = "BlobUploadError";
  }
}

/**
 * Error codes for upload operations.
 */
export type BlobErrorCode =
  | "FILE_TOO_LARGE"
  | "INVALID_CONTENT_TYPE"
  | "UPLOAD_EXPIRED"
  | "UPLOAD_FAILED"
  | "UPLOAD_ABORTED"
  | "NETWORK_ERROR"
  | "VALIDATION_ERROR";

/**
 * Allowed file types for media uploads.
 *
 * Client-side checks are a UX hint only; the server enforces its own
 * allowlist and magic-byte verification. SVG is deliberately absent: it is
 * scriptable content and the server default rejects it. Consumers that
 * genuinely need SVG must opt in server-side and serve it as an attachment
 * or from a sandboxed origin.
 */
export const ALLOWED_IMAGE_TYPES = [
  "image/jpeg",
  "image/png",
  "image/gif",
  "image/webp",
  "image/avif",
] as const;

export const ALLOWED_PDF_TYPES = ["application/pdf"] as const;

export const ALLOWED_MEDIA_TYPES = [
  ...ALLOWED_IMAGE_TYPES,
  ...ALLOWED_PDF_TYPES,
] as const;

/**
 * Video types that should be rejected.
 */
export const REJECTED_VIDEO_TYPES = [
  "video/mp4",
  "video/webm",
  "video/ogg",
  "video/quicktime",
  "video/x-msvideo",
  "video/x-matroska",
] as const;

export type AllowedImageType = (typeof ALLOWED_IMAGE_TYPES)[number];
export type AllowedPdfType = (typeof ALLOWED_PDF_TYPES)[number];
export type AllowedMediaType = (typeof ALLOWED_MEDIA_TYPES)[number];
