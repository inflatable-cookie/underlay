/**
 * Media Upload Flow - Reusable state machine for media upload operations.
 *
 * This pattern encapsulates the common upload flow:
 * 1. select - File selection
 * 2. checking - Computing hash, checking for duplicates
 * 3. duplicate - Duplicate found, user chooses
 * 4. uploading - Uploading to blob storage with progress
 * 5. finalising - Finalising upload
 * 6. complete - Success
 * 7. error - Error state
 *
 * Usage:
 * ```typescript
 * const uploadFlow = createMediaUploadFlow({
 *   checkDuplicate: async (sha256) => { ... },
 *   createMedia: async (request) => { ... },
 *   initiateUpload: async (mediaId, request) => { ... },
 *   finaliseUpload: async (mediaId, versionId, request) => { ... },
 *   onComplete: (media) => { ... },
 * });
 *
 * // In template:
 * // {#if uploadFlow.step === 'select'}...{/if}
 * // <button onclick={() => uploadFlow.startUpload()}>Upload</button>
 * ```
 */

import { uploadToBlob, computeFileHash, validateFile } from "./blob-upload.js";
import { type UploadPlan } from "./blob-types.js";
import {
  type MediaSummary,
  type MediaDetail,
  type CreateMediaRequest,
  type CheckDuplicateResponse,
  type InitiateUploadRequest,
  type InitiateUploadResponse,
  type FinaliseUploadRequest,
  type FinaliseUploadResponse,
  MediaKind,
  MediaVisibility,
} from "./media-types.js";

/**
 * Upload step in the state machine
 */
export type MediaUploadStep =
  | "select"
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

/**
 * Options for creating a media upload flow
 */
export interface MediaUploadFlowOptions {
  /** Check for duplicate files by hash */
  checkDuplicate: (sha256: string) => Promise<CheckDuplicateResponse>;

  /** Create a new media item (metadata only) */
  createMedia: (request: CreateMediaRequest) => Promise<MediaDetail>;

  /** Initiate upload for a media item */
  initiateUpload: (
    mediaId: string,
    request: InitiateUploadRequest
  ) => Promise<InitiateUploadResponse>;

  /** Finalise upload after blob is stored */
  finaliseUpload: (
    mediaId: string,
    versionId: string,
    request: FinaliseUploadRequest
  ) => Promise<FinaliseUploadResponse>;

  /** Called when upload completes successfully */
  onComplete?: (media: MediaDetail) => void;

  /** Called when an error occurs */
  onError?: (error: Error) => void;

  /** Maximum file size in bytes (default 25MB) */
  maxFileSize?: number;

  /** Default visibility for new media (default: Public) */
  defaultVisibility?: MediaVisibility;

  // For "replace file" flow (adding version to existing media)
  /** Existing media ID (for replace flow) */
  existingMediaId?: string;

  /** Existing version hashes to check against (for replace flow) */
  existingVersionHashes?: string[];
}

/**
 * Media upload flow controller
 */
export interface MediaUploadFlowController {
  // Reactive state (read-only from outside)
  readonly step: MediaUploadStep;
  readonly file: File | null;
  readonly fileError: string | null;
  readonly fileHash: string | null;
  readonly progress: number;
  readonly error: string | null;
  readonly duplicateMedia: MediaSummary | null;
  readonly createdMedia: MediaDetail | null;

  // Computed
  readonly canUpload: boolean;
  readonly isUploading: boolean;

  // Actions
  setFile: (file: File) => void;
  clearFile: () => void;
  startUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  proceedWithUpload: (metadata?: Partial<CreateMediaRequest>) => Promise<void>;
  useDuplicate: () => void;
  reset: () => void;
}

/**
 * Create a media upload flow controller with Svelte 5 runes.
 */
export function createMediaUploadFlow(
  options: MediaUploadFlowOptions
): MediaUploadFlowController {
  const {
    checkDuplicate,
    createMedia,
    initiateUpload,
    finaliseUpload,
    onComplete,
    onError,
    maxFileSize = 25 * 1024 * 1024,
    defaultVisibility = MediaVisibility.Public,
    existingMediaId,
    existingVersionHashes = [],
  } = options;

  // State
  let step = $state<MediaUploadStep>("select");
  let file = $state<File | null>(null);
  let fileError = $state<string | null>(null);
  let fileHash = $state<string | null>(null);
  let progress = $state(0);
  let error = $state<string | null>(null);
  let duplicateMedia = $state<MediaSummary | null>(null);
  let createdMedia = $state<MediaDetail | null>(null);

  // Computed
  const canUpload = $derived(file !== null && fileError === null);
  const isUploading = $derived(
    step === "checking" || step === "uploading" || step === "finalising"
  );

  // Helpers
  function detectMediaKind(f: File): MediaKind {
    if (f.type.startsWith("image/")) return MediaKind.Image;
    if (f.type === "application/pdf") return MediaKind.Pdf;
    return MediaKind.Image;
  }

  function setFile(f: File) {
    fileError = null;
    file = null;

    try {
      validateFile(f, maxFileSize);
      file = f;
    } catch (e) {
      fileError = e instanceof Error ? e.message : "Invalid file";
    }
  }

  function clearFile() {
    file = null;
    fileError = null;
    fileHash = null;
    step = "select";
    progress = 0;
    error = null;
    duplicateMedia = null;
  }

  function reset() {
    clearFile();
    createdMedia = null;
  }

  async function startUpload(metadata?: Partial<CreateMediaRequest>) {
    if (!file) return;

    step = "checking";
    error = null;

    try {
      // Step 1: Compute file hash
      fileHash = await computeFileHash(file);

      // Step 2: Check for duplicates
      // For replace flow, first check against existing versions
      if (existingVersionHashes.includes(fileHash)) {
        error = "This file has already been uploaded as a version of this media item.";
        step = "error";
        return;
      }

      // Check global duplicates (for new upload flow)
      if (!existingMediaId) {
        const duplicateCheck = await checkDuplicate(fileHash);

        if (duplicateCheck.exists && duplicateCheck.media) {
          duplicateMedia = duplicateCheck.media;
          step = "duplicate";
          return;
        }
      }

      // No duplicate - proceed with upload
      await proceedWithUpload(metadata);
    } catch (e) {
      console.error("Upload failed", e);
      error = e instanceof Error ? e.message : "Upload failed";
      step = "error";
      onError?.(e instanceof Error ? e : new Error(String(e)));
    }
  }

  async function proceedWithUpload(metadata?: Partial<CreateMediaRequest>) {
    if (!file || !fileHash) return;

    try {
      step = "uploading";
      progress = 0;

      let mediaId: string;
      let media: MediaDetail;

      if (existingMediaId) {
        // Replace flow - use existing media ID
        mediaId = existingMediaId;
        // We don't have the full media detail in replace flow, but we'll get it after finalise
        media = null as unknown as MediaDetail;
      } else {
        // New upload flow - create media item first
        media = await createMedia({
          kind: detectMediaKind(file),
          visibility: defaultVisibility,
          title: metadata?.title ?? file.name.replace(/\.[^/.]+$/, ""),
          originalFilename: metadata?.originalFilename ?? file.name,
          ...metadata,
        });
        mediaId = media.id;
      }

      // Initiate upload
      const uploadResponse = await initiateUpload(mediaId, {
        contentType: file.type,
        contentLength: file.size,
        sha256: fileHash,
      });

      // Upload file to blob storage
      const plan: UploadPlan = {
        uploadUrl: uploadResponse.uploadPlan.uploadUrl,
        method: uploadResponse.uploadPlan.method,
        requiredHeaders: uploadResponse.uploadPlan.headers,
        expiresAt: uploadResponse.uploadPlan.expiresAt,
        maxBytes: uploadResponse.uploadPlan.maxBytes || maxFileSize,
        allowedContentTypes: uploadResponse.uploadPlan.allowedContentTypes || [],
        objectKey: "",
      };

      await uploadToBlob(plan, file, {
        onProgress: (p) => {
          progress = p.percent;
        },
      });

      // Finalise upload
      step = "finalising";
      const finaliseResponse = await finaliseUpload(mediaId, uploadResponse.versionId, {
        sha256: fileHash,
      });

      // Done!
      createdMedia = finaliseResponse.media;
      step = "complete";
      onComplete?.(finaliseResponse.media);
    } catch (e) {
      console.error("Upload failed", e);
      error = e instanceof Error ? e.message : "Upload failed";
      step = "error";
      onError?.(e instanceof Error ? e : new Error(String(e)));
    }
  }

  function useDuplicate() {
    // This is a signal to the UI that the user wants to use the duplicate
    // The UI should handle navigation to the duplicate media
    // We just reset the flow state
    if (duplicateMedia) {
      // Keep duplicateMedia available for the UI to use
      step = "select";
    }
  }

  return {
    get step() {
      return step;
    },
    get file() {
      return file;
    },
    get fileError() {
      return fileError;
    },
    get fileHash() {
      return fileHash;
    },
    get progress() {
      return progress;
    },
    get error() {
      return error;
    },
    get duplicateMedia() {
      return duplicateMedia;
    },
    get createdMedia() {
      return createdMedia;
    },
    get canUpload() {
      return canUpload;
    },
    get isUploading() {
      return isUploading;
    },
    setFile,
    clearFile,
    startUpload,
    proceedWithUpload,
    useDuplicate,
    reset,
  };
}
