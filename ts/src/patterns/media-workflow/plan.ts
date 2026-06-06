import type { UploadPlan } from "../blob-types";
import type { MediaWorkflowUploadPlan } from "./types";

export const DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE = 50 * 1024 * 1024;

export function toUploadPlan(
  plan: MediaWorkflowUploadPlan,
  maxFileSize: number,
): UploadPlan {
  return {
    uploadUrl: plan.uploadUrl,
    method: plan.method,
    requiredHeaders: plan.headers ?? {},
    expiresAt: plan.expiresAt,
    maxBytes: plan.maxBytes ?? maxFileSize,
    allowedContentTypes: plan.allowedContentTypes ?? [],
    objectKey: plan.objectKey ?? "",
  };
}
