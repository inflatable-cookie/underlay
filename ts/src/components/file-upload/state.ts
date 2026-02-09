import { compressImage, type ImageCompressionOptions } from "./compression";
import { generateFileUploadId, validateUploadFile } from "./helpers";
import type { FileUploadItem, FileUploadValidationError } from "./types";

interface ProcessUploadFilesOptions {
  fileList: FileList | null;
  currentFiles: FileUploadItem[];
  accept: string;
  maxSize: number;
  multiple: boolean;
  maxFiles: number;
  showPreview: boolean;
  validate?: (file: File) => string | null;
  compress: boolean;
  compressionOptions: ImageCompressionOptions;
  onValidationError?: (event: FileUploadValidationError) => void;
}

interface ProcessUploadFilesResult {
  nextFiles: FileUploadItem[];
  filesToUpload: File[];
  replacedPreviewUrls: string[];
}

export async function processUploadFiles({
  fileList,
  currentFiles,
  accept,
  maxSize,
  multiple,
  maxFiles,
  showPreview,
  validate,
  compress,
  compressionOptions,
  onValidationError
}: ProcessUploadFilesOptions): Promise<ProcessUploadFilesResult> {
  if (!fileList || fileList.length === 0) {
    return {
      nextFiles: currentFiles,
      filesToUpload: [],
      replacedPreviewUrls: []
    };
  }

  const newFiles: FileUploadItem[] = [];
  const filesToUpload: File[] = [];

  const availableSlots = multiple ? maxFiles - currentFiles.length : 1;
  const filesToProcess = Array.from(fileList).slice(0, availableSlots);

  for (const file of filesToProcess) {
    const error = validateUploadFile({
      file,
      maxSize,
      accept,
      validate
    });

    if (error) {
      onValidationError?.({ file, message: error });
      continue;
    }

    let processedFile = file;
    let originalFile: File | undefined;

    if (compress && file.type.startsWith("image/")) {
      const compressed = await compressImage(file, compressionOptions);
      if (compressed !== file) {
        originalFile = file;
        processedFile = compressed;
      }
    }

    const item: FileUploadItem = {
      file: processedFile,
      id: generateFileUploadId(),
      progress: 0,
      status: "pending",
      originalFile
    };

    if (showPreview && processedFile.type.startsWith("image/")) {
      item.previewUrl = URL.createObjectURL(processedFile);
    }

    newFiles.push(item);
    filesToUpload.push(processedFile);
  }

  if (newFiles.length === 0) {
    return {
      nextFiles: currentFiles,
      filesToUpload: [],
      replacedPreviewUrls: []
    };
  }

  if (multiple) {
    return {
      nextFiles: [...currentFiles, ...newFiles],
      filesToUpload,
      replacedPreviewUrls: []
    };
  }

  return {
    nextFiles: newFiles,
    filesToUpload,
    replacedPreviewUrls: currentFiles[0]?.previewUrl ? [currentFiles[0].previewUrl] : []
  };
}

export function revokePreviewUrls(items: FileUploadItem[]): void {
  for (const item of items) {
    if (item.previewUrl) {
      URL.revokeObjectURL(item.previewUrl);
    }
  }
}

export function removeUploadItem(
  currentFiles: FileUploadItem[],
  item: FileUploadItem
): FileUploadItem[] {
  if (item.previewUrl) {
    URL.revokeObjectURL(item.previewUrl);
  }

  return currentFiles.filter((fileItem) => fileItem.id !== item.id);
}

export function retryUploadItem(
  currentFiles: FileUploadItem[],
  item: FileUploadItem
): { nextFiles: FileUploadItem[]; retryFile: File } {
  const nextFiles = currentFiles.map((current) =>
    current.id === item.id
      ? {
          ...current,
          status: "pending" as const,
          error: undefined,
          progress: 0
        }
      : current
  );

  return {
    nextFiles,
    retryFile: item.file
  };
}

export function updateUploadProgress(
  currentFiles: FileUploadItem[],
  id: string,
  progress: number
): FileUploadItem[] {
  return currentFiles.map((item) =>
    item.id === id
      ? {
          ...item,
          progress,
          status: progress < 100 ? "uploading" : "complete"
        }
      : item
  );
}

export function setUploadError(
  currentFiles: FileUploadItem[],
  id: string,
  message: string
): FileUploadItem[] {
  return currentFiles.map((item) =>
    item.id === id
      ? {
          ...item,
          status: "error",
          error: message
        }
      : item
  );
}
