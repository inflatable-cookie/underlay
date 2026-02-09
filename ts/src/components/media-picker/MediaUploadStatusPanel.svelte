<script lang="ts">
  import AlertCircle from "lucide-svelte/icons/alert-circle";
  import Check from "lucide-svelte/icons/check";
  import { getMediaDisplayName, type MediaSummary } from "../../patterns/index.js";
  import Button from "../Button.svelte";

  type UploadStep =
    | "select"
    | "checking"
    | "duplicate"
    | "uploading"
    | "finalising"
    | "complete"
    | "error";

  interface Props {
    uploadStep: UploadStep;
    duplicateMedia: MediaSummary | null;
    uploadProgress: number;
    uploadError: string | null;
    onUploadAnyway: () => void;
    onSelectDuplicate: () => void;
    onClearUpload: () => void;
    onSelectUploaded: () => void;
  }

  let {
    uploadStep,
    duplicateMedia,
    uploadProgress,
    uploadError,
    onUploadAnyway,
    onSelectDuplicate,
    onClearUpload,
    onSelectUploaded
  }: Props = $props();
</script>

{#if uploadStep === "checking"}
  <div class="status-panel">
    <div class="spinner"></div>
    <p>Checking for duplicates...</p>
  </div>
{:else if uploadStep === "duplicate"}
  <div class="status-panel status-panel--warning">
    <AlertCircle size={32} />
    <p>This file already exists</p>
    {#if duplicateMedia}
      <span class="duplicate-name"
        >{getMediaDisplayName(duplicateMedia)}</span
      >
    {/if}
    <div class="upload-actions">
      <Button variant="secondary" onclick={onUploadAnyway}
        >Upload as new</Button
      >
      <Button variant="primary" onclick={onSelectDuplicate}
        >Use existing</Button
      >
    </div>
  </div>
{:else if uploadStep === "uploading"}
  <div class="status-panel">
    <div class="progress-bar">
      <div
        class="progress-bar__fill"
        style="width: {uploadProgress}%"
      ></div>
    </div>
    <p>Uploading... {uploadProgress.toFixed(0)}%</p>
  </div>
{:else if uploadStep === "finalising"}
  <div class="status-panel">
    <div class="spinner"></div>
    <p>Finalising...</p>
  </div>
{:else if uploadStep === "complete"}
  <div class="status-panel status-panel--success">
    <Check size={32} />
    <p>Upload complete!</p>
    <div class="upload-actions">
      <Button variant="secondary" onclick={onClearUpload}
        >Upload another</Button
      >
      <Button variant="primary" onclick={onSelectUploaded}
        >Use this media</Button
      >
    </div>
  </div>
{:else if uploadStep === "error"}
  <div class="status-panel status-panel--error">
    <AlertCircle size={32} />
    <p>{uploadError || "Upload failed"}</p>
    <Button variant="secondary" onclick={onClearUpload}>Try again</Button>
  </div>
{/if}

<style>
  .upload-actions {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .status-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 2rem;
    text-align: center;
  }

  .status-panel p {
    margin: 0;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .status-panel--warning {
    color: var(--underlay-color-warning, #f59e0b);
  }

  .status-panel--success {
    color: var(--underlay-color-success, #22c55e);
  }

  .status-panel--error {
    color: var(--underlay-color-danger, #ef4444);
  }

  .duplicate-name {
    font-weight: 500;
    color: var(--underlay-color-text, #f3f4f6);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--underlay-color-border, #374151);
    border-top-color: var(--underlay-color-primary, #3b82f6);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .progress-bar {
    width: 100%;
    max-width: 200px;
    height: 6px;
    background: var(--underlay-color-surface-raised, #374151);
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-bar__fill {
    height: 100%;
    background: var(--underlay-color-primary, #3b82f6);
    transition: width 0.1s ease-out;
  }
</style>
