<script lang="ts">
  import {
    ALLOWED_MEDIA_TYPES,
    REJECTED_VIDEO_TYPES,
    validateFileType,
    type UploadProgress
  } from "../runtime/media";
  import {
    Button,
    Callout,
    FileUpload,
    Progress,
    formatFileSize
  } from "@inflatable-cookie/poodle-svelte";
  import type { FileUploadItem } from "@inflatable-cookie/poodle-svelte";
  import CheckCircle from "lucide-svelte/icons/circle-check";
  import Upload from "lucide-svelte/icons/upload";

  type UploadStage = "idle" | "hashing" | "uploading" | "finalising" | "done";

  interface ReplaceUploadInput {
    file: File;
    mediaId: string;
    onProgress?: (progress: UploadProgress) => void;
    signal?: AbortSignal;
  }

  interface Props {
    mediaId: string;
    maxFileSize?: number;
    accept?: string;
    replaceUpload: (input: ReplaceUploadInput) => Promise<unknown>;
    onCancel?: () => void;
    onSuccess?: () => void | Promise<void>;
    onToast?: (variant: "success" | "error", message: string) => void;
  }

  let {
    mediaId,
    maxFileSize = 50 * 1024 * 1024,
    accept = ALLOWED_MEDIA_TYPES.join(","),
    replaceUpload,
    onCancel,
    onSuccess,
    onToast
  }: Props = $props();

  let files = $state<FileUploadItem[]>([]);
  let uploading = $state(false);
  let uploadAbort: AbortController | null = null;
  let error = $state<string | null>(null);
  let uploadStage = $state<UploadStage>("idle");
  let uploadProgress = $state(0);

  $effect(() => {
    if (files.length > 0) {
      validateSelectedFile();
    }
  });

  function validateSelectedFile(): void {
    const file = files[0]?.file;
    if (!file) return;

    if (REJECTED_VIDEO_TYPES.includes(file.type as (typeof REJECTED_VIDEO_TYPES)[number])) {
      error = "Video uploads are not supported. Please upload images or PDFs.";
      files = [];
      return;
    }

    if (!validateFileType(file, ALLOWED_MEDIA_TYPES)) {
      error = `Unsupported file type: ${file.type}. Supported types: Images (JPEG, PNG, GIF, WebP, AVIF) and PDF.`;
      files = [];
      return;
    }

    if (file.size > maxFileSize) {
      error = `File is too large (${formatFileSize(file.size)}). Maximum size is ${formatFileSize(maxFileSize)}.`;
      files = [];
      return;
    }

    error = null;
  }

  async function handleSubmit(): Promise<void> {
    const file = files[0]?.file;
    if (!file) {
      error = "Select a file to upload.";
      return;
    }

    uploading = true;
    uploadAbort = new AbortController();
    error = null;
    uploadProgress = 0;

    try {
      uploadStage = "hashing";
      await replaceUpload({
        file,
        mediaId,
        signal: uploadAbort.signal,
        onProgress: (progress) => {
          uploadStage = "uploading";
          uploadProgress = progress.percent;
        }
      });

      uploadStage = "done";
      onToast?.("success", "File replaced");
      await onSuccess?.();
    } catch (uploadError) {
      error = uploadError instanceof Error ? uploadError.message : "Failed to upload media";
      onToast?.("error", error);
      uploadStage = "idle";
    } finally {
      uploadAbort = null;
      uploading = false;
    }
  }

  function handleCancel(): void {
    if (uploading && uploadAbort) {
      uploadAbort.abort();
      return;
    }
    onCancel?.();
  }
</script>

<div class="underlay-media-replace-file-form">
  {#if error}
    <Callout tone="danger" title="Could not replace file" message={error} announceMode="polite" />
  {/if}

  {#if uploading}
    <section class="underlay-media-replace-file-form__progress">
      <div class="underlay-media-replace-file-form__progress-header">
        <span class="underlay-media-replace-file-form__progress-stage">
          {uploadStage === "hashing" ? "Computing file hash..." :
           uploadStage === "uploading" ? "Uploading file..." :
           uploadStage === "finalising" ? "Finalising..." :
           uploadStage === "done" ? "Done!" : ""}
        </span>
        {#if uploadStage === "uploading"}
          <span class="underlay-media-replace-file-form__progress-percent">{uploadProgress}%</span>
        {/if}
      </div>
      <Progress
        value={uploadStage === "uploading" ? uploadProgress : uploadStage === "done" ? 100 : 0}
        max={100}
        ariaLabel="Replace upload progress"
      />
      {#if uploadStage === "done"}
        <div class="underlay-media-replace-file-form__progress-done">
          <CheckCircle size={20} />
          <span>Upload complete.</span>
        </div>
      {/if}
    </section>
  {:else}
    <section class="underlay-media-replace-file-form__upload">
      <FileUpload
        bind:files
        {accept}
        maxSize={maxFileSize}
        disabled={uploading}
      />
      <p class="underlay-media-replace-file-form__hint">
        Supported formats: JPEG, PNG, GIF, WebP, AVIF, PDF. Maximum size: {formatFileSize(maxFileSize)}.
      </p>
    </section>
  {/if}

  <div class="underlay-media-replace-file-form__actions">
    <Button type="button" variant="ghost" onClick={handleCancel}>
      Cancel
    </Button>
    <Button
      type="button"
      variant="primary"
      disabled={uploading || files.length === 0 || !!error}
      onClick={handleSubmit}
    >
      {#snippet leading()}
        <Upload size={16} />
      {/snippet}
      {uploading ? "Uploading..." : "Replace File"}
    </Button>
  </div>
</div>

<style>
  .underlay-media-replace-file-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    --underlay-upload-border: 2px dashed var(--admin-color-border-strong);
    --underlay-upload-border-active: 2px dashed var(--admin-color-accent);
    --underlay-upload-bg: var(--admin-color-surface-card);
    --underlay-upload-bg-hover: var(--admin-color-surface-subtle);
    --color-border: var(--admin-color-border-subtle);
    --color-surface: var(--admin-color-surface-card);
    --color-surface-subtle: var(--admin-color-surface-subtle);
    --color-surface-hover: var(--admin-color-surface-subtle);
    --color-text-muted: var(--admin-color-text-muted);
    --color-primary: var(--admin-color-accent);
  }

  .underlay-media-replace-file-form__upload,
  .underlay-media-replace-file-form__progress {
    background: var(--admin-color-surface-card);
    border: 1px solid var(--admin-color-border-subtle);
    border-radius: 0.5rem;
    padding: 1.25rem;
  }

  .underlay-media-replace-file-form__hint {
    margin: 1rem 0 0;
    font-size: 0.875rem;
    color: var(--admin-color-text-muted);
    line-height: 1.5;
  }

  .underlay-media-replace-file-form__progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .underlay-media-replace-file-form__progress-stage,
  .underlay-media-replace-file-form__progress-percent {
    font-size: 0.875rem;
    color: var(--admin-color-text);
  }

  .underlay-media-replace-file-form__progress-percent {
    font-weight: 500;
  }

  .underlay-media-replace-file-form__progress-done {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
    color: #10b981;
    font-weight: 500;
  }

  .underlay-media-replace-file-form__actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }
</style>
