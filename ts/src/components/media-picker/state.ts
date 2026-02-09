import { validateFile, type MediaSummary } from "../../patterns/index.js";

export type MediaPickerUploadStep =
  | "select"
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

export interface MediaPickerUploadState {
  selectedFile: File | null;
  fileError: string | null;
  uploadStep: MediaPickerUploadStep;
  uploadProgress: number;
  uploadError: string | null;
  duplicateMedia: MediaSummary | null;
  fileHash: string | null;
  createdMedia: MediaSummary | null;
}

export interface MediaPickerBrowseState {
  browseItems: MediaSummary[];
  browseNextCursor: string | null;
  browseHasMore: boolean;
}

export function createClearedUploadState(): MediaPickerUploadState {
  return {
    selectedFile: null,
    fileError: null,
    uploadStep: "select",
    uploadProgress: 0,
    uploadError: null,
    duplicateMedia: null,
    fileHash: null,
    createdMedia: null
  };
}

export function createResetBrowseState(): MediaPickerBrowseState {
  return {
    browseItems: [],
    browseNextCursor: null,
    browseHasMore: false
  };
}

export function validateMediaPickerFile(
  file: File,
  maxFileSize: number
): { selectedFile: File | null; fileError: string | null } {
  try {
    validateFile(file, maxFileSize);
    return {
      selectedFile: file,
      fileError: null
    };
  } catch (error) {
    return {
      selectedFile: null,
      fileError: error instanceof Error ? error.message : "Invalid file"
    };
  }
}
