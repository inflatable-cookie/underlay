/**
 * Blob upload utilities for direct-to-S3 uploads from the browser.
 *
 * This module provides functions to:
 * - Upload files directly to blob storage using pre-signed URLs
 * - Compute SHA-256 hashes for deduplication
 * - Validate file types and sizes before upload
 */

import {
  type UploadPlan,
  type UploadProgress,
  type UploadResult,
  type UploadOptions,
  BlobUploadError,
  ALLOWED_MEDIA_TYPES,
  REJECTED_VIDEO_TYPES,
} from "./blob-types";

/**
 * Upload a file to blob storage using a pre-signed URL.
 *
 * @param plan - The upload plan from the server containing the pre-signed URL
 * @param file - The file to upload
 * @param options - Optional configuration (progress callback, abort signal)
 * @returns Promise resolving to the upload result
 *
 * @example
 * ```typescript
 * const plan = await api.initiateUpload(mediaId, file.name, file.type, file.size);
 * const result = await uploadToBlob(plan, file, {
 *   onProgress: (p) => console.log(`${p.percent}% uploaded`)
 * });
 * ```
 */
export async function uploadToBlob(
  plan: UploadPlan,
  file: File,
  options: UploadOptions = {}
): Promise<UploadResult> {
  const { onProgress, signal } = options;

  // Validate file size
  if (file.size > plan.maxBytes) {
    throw new BlobUploadError(
      `File size ${file.size} exceeds maximum ${plan.maxBytes} bytes`,
      "FILE_TOO_LARGE"
    );
  }

  // Validate content type
  if (plan.allowedContentTypes.length > 0 && !matchesAllowedMimeType(file.type, plan.allowedContentTypes)) {
    throw new BlobUploadError(
      `Content type ${file.type} not in allowed types: ${plan.allowedContentTypes.join(", ")}`,
      "INVALID_CONTENT_TYPE"
    );
  }

  // Check if upload URL has expired
  const expiresAt = new Date(plan.expiresAt);
  if (expiresAt < new Date()) {
    throw new BlobUploadError("Upload URL has expired", "UPLOAD_EXPIRED");
  }

  // Build headers
  const headers: Record<string, string> = {
    "Content-Type": file.type,
    ...plan.requiredHeaders,
  };

  // Use XMLHttpRequest for progress tracking
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();

    // Handle abort
    if (signal) {
      signal.addEventListener("abort", () => {
        xhr.abort();
      });
    }

    // Progress tracking
    if (onProgress) {
      xhr.upload.addEventListener("progress", (event) => {
        if (event.lengthComputable) {
          onProgress({
            loaded: event.loaded,
            total: event.total,
            percent: Math.round((event.loaded / event.total) * 100),
          });
        }
      });
    }

    // Completion handling
    xhr.addEventListener("load", () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve({
          objectKey: plan.objectKey,
          size: file.size,
          contentType: file.type,
        });
      } else {
        reject(
          new BlobUploadError(
            `Upload failed with status ${xhr.status}: ${xhr.statusText}`,
            "UPLOAD_FAILED",
            xhr.status
          )
        );
      }
    });

    xhr.addEventListener("error", () => {
      reject(new BlobUploadError("Network error during upload", "NETWORK_ERROR"));
    });

    xhr.addEventListener("abort", () => {
      reject(new BlobUploadError("Upload was aborted", "UPLOAD_ABORTED"));
    });

    // Open and send
    xhr.open(plan.method, plan.uploadUrl);

    for (const [key, value] of Object.entries(headers)) {
      xhr.setRequestHeader(key, value);
    }

    xhr.send(file);
  });
}

/**
 * Compute the SHA-256 hash of a file.
 *
 * Uses the Web Crypto API to compute the hash efficiently.
 * For large files, this reads the file in chunks to avoid memory issues.
 *
 * @param file - The file to hash
 * @returns Promise resolving to the hex-encoded SHA-256 hash
 *
 * @example
 * ```typescript
 * const hash = await computeFileHash(file);
 * const existing = await api.checkDuplicate(hash);
 * if (existing.exists) {
 *   // File already exists in library
 * }
 * ```
 */
export async function computeFileHash(file: File): Promise<string> {
  // For files smaller than 64MB, read all at once
  const CHUNK_THRESHOLD = 64 * 1024 * 1024;

  if (file.size <= CHUNK_THRESHOLD) {
    const buffer = await file.arrayBuffer();
    const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
    return bufferToHex(hashBuffer);
  }

  // Web Crypto does not support incremental SHA-256 hashing, so large files
  // still need a full read. Very large files are better hashed server-side.
  const buffer = await file.arrayBuffer();
  const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
  return bufferToHex(hashBuffer);
}

/**
 * Convert an ArrayBuffer to a hex string.
 */
function bufferToHex(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

/**
 * Validate a file's type against an allow list.
 *
 * @param file - The file to validate
 * @param allowedTypes - Array of allowed MIME types
 * @returns True if the file type is allowed
 *
 * @example
 * ```typescript
 * if (!validateFileType(file, ALLOWED_MEDIA_TYPES)) {
 *   showError("This file type is not supported");
 * }
 * ```
 */
export function validateFileType(
  file: File,
  allowedTypes: readonly string[] = ALLOWED_MEDIA_TYPES
): boolean {
  return matchesAllowedMimeType(file.type, allowedTypes);
}

function matchesAllowedMimeType(mimeType: string, allowedTypes: readonly string[]): boolean {
  const baseType = mimeType.split(";", 1)[0]?.trim().toLowerCase();
  return allowedTypes.some((allowedType) => allowedType.toLowerCase() === baseType);
}

/**
 * Check if a file is a video (which should be rejected).
 *
 * @param file - The file to check
 * @returns True if the file appears to be a video
 */
export function isVideoFile(file: File): boolean {
  return (
    REJECTED_VIDEO_TYPES.includes(file.type as (typeof REJECTED_VIDEO_TYPES)[number]) ||
    file.type.startsWith("video/")
  );
}

/**
 * Validate a file's size against a maximum.
 *
 * @param file - The file to validate
 * @param maxBytes - Maximum allowed size in bytes
 * @returns True if the file size is within the limit
 *
 * @example
 * ```typescript
 * const MAX_SIZE = 25 * 1024 * 1024; // 25MB
 * if (!validateFileSize(file, MAX_SIZE)) {
 *   showError(`File must be smaller than ${MAX_SIZE / 1024 / 1024}MB`);
 * }
 * ```
 */
export function validateFileSize(file: File, maxBytes: number): boolean {
  return file.size <= maxBytes;
}

/**
 * Format a file size in bytes to a human-readable string.
 *
 * @param bytes - Size in bytes (null/undefined returns "—")
 * @returns Human-readable size string (e.g., "1.5 MB")
 */
export function formatFileSize(bytes: number | null | undefined): string {
  if (bytes == null) return "—";
  if (bytes === 0) return "0 B";

  const units = ["B", "KB", "MB", "GB"];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = bytes / Math.pow(k, i);

  return `${size.toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
}

/**
 * Get a user-friendly file type description.
 *
 * @param mimeType - The MIME type
 * @returns Human-readable description
 */
export function getFileTypeDescription(mimeType: string): string {
  const descriptions: Record<string, string> = {
    "image/jpeg": "JPEG image",
    "image/png": "PNG image",
    "image/gif": "GIF image",
    "image/webp": "WebP image",
    "image/svg+xml": "SVG image",
    "application/pdf": "PDF document",
  };

  return descriptions[mimeType] || mimeType;
}

/**
 * Validate a file before upload, checking type, size, and rejecting videos.
 *
 * @param file - The file to validate
 * @param maxBytes - Maximum allowed size in bytes
 * @param allowedTypes - Array of allowed MIME types
 * @returns Validation result with any error message
 */
export function validateFile(
  file: File,
  maxBytes: number,
  allowedTypes: readonly string[] = ALLOWED_MEDIA_TYPES
): { valid: boolean; error?: string } {
  if (isVideoFile(file)) {
    return {
      valid: false,
      error: "Video files are not supported. Please use Vimeo for video content.",
    };
  }

  if (!validateFileType(file, allowedTypes)) {
    return {
      valid: false,
      error: `File type "${file.type || "unknown"}" is not supported. Allowed types: ${allowedTypes.join(", ")}`,
    };
  }

  if (!validateFileSize(file, maxBytes)) {
    return {
      valid: false,
      error: `File is too large (${formatFileSize(file.size)}). Maximum size is ${formatFileSize(maxBytes)}.`,
    };
  }

  return { valid: true };
}
