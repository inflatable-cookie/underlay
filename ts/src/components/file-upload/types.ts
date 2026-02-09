export interface FileUploadItem {
  /** The file object */
  file: File;
  /** Unique ID for this upload */
  id: string;
  /** Upload progress (0-100) */
  progress: number;
  /** Current status */
  status: "pending" | "uploading" | "complete" | "error";
  /** Error message if status is 'error' */
  error?: string;
  /** Preview URL (for images) */
  previewUrl?: string;
  /** Original file (if compressed) */
  originalFile?: File;
}

export interface FileUploadValidationError {
  file: File;
  message: string;
}
