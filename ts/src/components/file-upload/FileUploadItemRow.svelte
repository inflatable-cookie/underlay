<script lang="ts">
  import { formatFileSize } from "./helpers";
  import type { FileUploadItem } from "./types";

  interface Props {
    item: FileUploadItem;
    onRetry: (item: FileUploadItem) => void;
    onRemove: (item: FileUploadItem) => void;
  }

  let {
    item,
    onRetry,
    onRemove
  }: Props = $props();
</script>

<li class="file-item" class:error={item.status === "error"}>
  {#if item.previewUrl}
    <div class="file-preview">
      <img src={item.previewUrl} alt={item.file.name} />
    </div>
  {:else}
    <div class="file-icon">
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14 2 14 8 20 8" />
      </svg>
    </div>
  {/if}

  <div class="file-info">
    <span class="file-name">{item.file.name}</span>
    <span class="file-size">{formatFileSize(item.file.size)}</span>
    {#if item.error}
      <span class="file-error">{item.error}</span>
    {/if}
  </div>

  {#if item.status === "uploading"}
    <div class="file-progress">
      <div class="progress-bar">
        <div class="progress-fill" style:width="{item.progress}%"></div>
      </div>
      <span class="progress-text">{item.progress}%</span>
    </div>
  {:else if item.status === "complete"}
    <div class="file-status complete">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="20 6 9 17 4 12" />
      </svg>
    </div>
  {:else if item.status === "error"}
    <button type="button" class="retry-button" onclick={() => onRetry(item)} aria-label="Retry upload">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M23 4v6h-6" />
        <path d="M1 20v-6h6" />
        <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
      </svg>
    </button>
  {/if}

  <button
    type="button"
    class="remove-button"
    onclick={() => onRemove(item)}
    aria-label="Remove file"
    disabled={item.status === "uploading"}
  >
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <line x1="18" y1="6" x2="6" y2="18" />
      <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  </button>
</li>

<style>
  .file-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--color-surface-subtle, #f8fafc);
    border-radius: var(--radius-md, 0.375rem);
    border: 1px solid var(--color-border, #e2e8f0);
  }

  .file-item.error {
    border-color: var(--color-danger, #ef4444);
    background: var(--color-danger-subtle, #fef2f2);
  }

  .file-preview {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-sm, 0.25rem);
    overflow: hidden;
    flex-shrink: 0;
  }

  .file-preview img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .file-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted, #64748b);
    flex-shrink: 0;
  }

  .file-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .file-name {
    font-size: 0.875rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size {
    font-size: 0.75rem;
    color: var(--color-text-muted, #64748b);
  }

  .file-error {
    font-size: 0.75rem;
    color: var(--color-danger, #ef4444);
  }

  .file-progress {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 120px;
  }

  .progress-bar {
    flex: 1;
    height: 4px;
    background: var(--color-border, #e2e8f0);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--color-primary, #3b82f6);
    transition: width 0.2s;
  }

  .progress-text {
    font-size: 0.75rem;
    color: var(--color-text-muted, #64748b);
    width: 35px;
    text-align: right;
  }

  .file-status.complete {
    color: var(--color-success, #22c55e);
  }

  .retry-button,
  .remove-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: var(--radius-sm, 0.25rem);
    color: var(--color-text-muted, #64748b);
    transition: background-color 0.2s;
  }

  .retry-button:hover,
  .remove-button:hover {
    background: var(--color-surface-hover, #f1f5f9);
  }

  .remove-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
