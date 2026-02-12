<script lang="ts">
  import Image from "lucide-svelte/icons/image";
  import FileText from "lucide-svelte/icons/file-text";
  import Upload from "lucide-svelte/icons/upload";
  import {
    ALLOWED_MEDIA_TYPES,
    formatFileSize,
    getFileTypeDescription
  } from "../../patterns/index.js";

  interface Props {
    selectedFile: File | null;
    hasError: boolean;
    onDrop: (event: DragEvent) => void;
    onDragOver: (event: DragEvent) => void;
    onFileSelect: (event: Event) => void;
  }

  let {
    selectedFile,
    hasError,
    onDrop,
    onDragOver,
    onFileSelect
  }: Props = $props();
</script>

<div
  class="underlay-dropzone"
  class:underlay-dropzone--has-file={selectedFile}
  class:underlay-dropzone--has-error={hasError}
  ondrop={onDrop}
  ondragover={onDragOver}
  role="button"
  tabindex="0"
>
  {#if selectedFile}
    <div class="underlay-selected-file">
      {#if selectedFile.type.startsWith("image/")}
        <Image size={32} />
      {:else}
        <FileText size={32} />
      {/if}
      <div class="underlay-selected-file__info">
        <span class="underlay-selected-file__name">{selectedFile.name}</span>
        <span class="underlay-selected-file__meta">
          {getFileTypeDescription(selectedFile.type)} &middot; {formatFileSize(
            selectedFile.size
          )}
        </span>
      </div>
    </div>
  {:else}
    <Upload size={32} />
    <p>Drop file here or click to browse</p>
    <p class="underlay-dropzone__hint">Images and PDFs up to 25MB</p>
    <input
      type="file"
      class="underlay-dropzone__input"
      accept={ALLOWED_MEDIA_TYPES.join(",")}
      onchange={onFileSelect}
    />
  {/if}
</div>

<style>
  .underlay-dropzone {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem;
    border: 2px dashed var(--underlay-color-border, #374151);
    border-radius: 0.5rem;
    background: var(--underlay-color-surface, #1f2937);
    cursor: pointer;
    transition: border-color 0.15s;
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-dropzone:hover {
    border-color: var(--underlay-color-primary, #3b82f6);
  }

  .underlay-dropzone--has-file {
    border-style: solid;
    border-color: var(--underlay-color-primary, #3b82f6);
    cursor: default;
  }

  .underlay-dropzone--has-error {
    border-color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-dropzone__hint {
    font-size: 0.75rem;
    opacity: 0.7;
  }

  .underlay-dropzone__input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
  }

  .underlay-dropzone--has-file .underlay-dropzone__input {
    display: none;
  }

  .underlay-selected-file {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--underlay-color-text, #f3f4f6);
  }

  .underlay-selected-file__info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.125rem;
  }

  .underlay-selected-file__name {
    font-weight: 500;
  }

  .underlay-selected-file__meta {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
