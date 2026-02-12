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
  <div class="underlay-status-panel">
    <div class="underlay-spinner"></div>
    <p>Checking for duplicates...</p>
  </div>
{:else if uploadStep === "duplicate"}
  <div class="underlay-status-panel underlay-status-panel--warning">
    <AlertCircle size={32} />
    <p>This file already exists</p>
    {#if duplicateMedia}
      <span class="underlay-duplicate-name"
        >{getMediaDisplayName(duplicateMedia)}</span
      >
    {/if}
    <div class="underlay-upload-actions">
      <Button variant="secondary" onclick={onUploadAnyway}
        >Upload as new</Button
      >
      <Button variant="primary" onclick={onSelectDuplicate}
        >Use existing</Button
      >
    </div>
  </div>
{:else if uploadStep === "uploading"}
  <div class="underlay-status-panel">
    <div class="underlay-progress-bar">
      <div
        class="underlay-progress-bar__fill"
        style="width: {uploadProgress}%"
      ></div>
    </div>
    <p>Uploading... {uploadProgress.toFixed(0)}%</p>
  </div>
{:else if uploadStep === "finalising"}
  <div class="underlay-status-panel">
    <div class="underlay-spinner"></div>
    <p>Finalising...</p>
  </div>
{:else if uploadStep === "complete"}
  <div class="underlay-status-panel underlay-status-panel--success">
    <Check size={32} />
    <p>Upload complete!</p>
    <div class="underlay-upload-actions">
      <Button variant="secondary" onclick={onClearUpload}
        >Upload another</Button
      >
      <Button variant="primary" onclick={onSelectUploaded}
        >Use this media</Button
      >
    </div>
  </div>
{:else if uploadStep === "error"}
  <div class="underlay-status-panel underlay-status-panel--error">
    <AlertCircle size={32} />
    <p>{uploadError || "Upload failed"}</p>
    <Button variant="secondary" onclick={onClearUpload}>Try again</Button>
  </div>
{/if}

<style>
  .underlay-upload-actions {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .underlay-status-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 2rem;
    text-align: center;
  }

  .underlay-status-panel p {
    margin: 0;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-status-panel--warning {
    color: var(--underlay-color-warning, #f59e0b);
  }

  .underlay-status-panel--success {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-status-panel--error {
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-duplicate-name {
    font-weight: 500;
    color: var(--underlay-color-text, #f3f4f6);
  }

  .underlay-spinner {
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

  .underlay-progress-bar {
    width: 100%;
    max-width: 200px;
    height: 6px;
    background: var(--underlay-color-surface-raised, #374151);
    border-radius: 3px;
    overflow: hidden;
  }

  .underlay-progress-bar__fill {
    height: 100%;
    background: var(--underlay-color-primary, #3b82f6);
    transition: width 0.1s ease-out;
  }
</style>
