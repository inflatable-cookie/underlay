<script lang="ts">
  import type { Snippet } from "svelte";
  import { formatFileSize } from "./helpers";

  interface Props {
    accept: string;
    multiple: boolean;
    disabled: boolean;
    isDragging: boolean;
    maxSize: number;
    dropzone?: Snippet;
    inputElement?: HTMLInputElement;
    onClick: () => void;
    onKeydown: (event: KeyboardEvent) => void;
    onDragEnter: (event: DragEvent) => void;
    onDragLeave: (event: DragEvent) => void;
    onDragOver: (event: DragEvent) => void;
    onDrop: (event: DragEvent) => void;
    onInputChange: (event: Event) => void;
  }

  let {
    accept,
    multiple,
    disabled,
    isDragging,
    maxSize,
    dropzone,
    inputElement = $bindable(),
    onClick,
    onKeydown,
    onDragEnter,
    onDragLeave,
    onDragOver,
    onDrop,
    onInputChange
  }: Props = $props();
</script>

<div
  class="drop-zone"
  class:dragging={isDragging}
  role="button"
  tabindex={disabled ? -1 : 0}
  aria-disabled={disabled}
  onclick={onClick}
  onkeydown={onKeydown}
  ondragenter={onDragEnter}
  ondragleave={onDragLeave}
  ondragover={onDragOver}
  ondrop={onDrop}
>
  <input
    bind:this={inputElement}
    type="file"
    {accept}
    {multiple}
    {disabled}
    class="visually-hidden"
    onchange={onInputChange}
    aria-label="File upload"
  />

  {#if dropzone}
    {@render dropzone()}
  {:else}
    <div class="drop-zone-content">
      <div class="drop-zone-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
      </div>
      <p class="drop-zone-text">
        {#if isDragging}
          Drop files here
        {:else}
          <span class="drop-zone-link">Click to upload</span> or drag and drop
        {/if}
      </p>
      <p class="drop-zone-hint">
        {#if accept !== "*"}
          {accept.replace(/\./g, "").replace(/,/g, ", ")}
        {/if}
        {#if maxSize}
          (max {formatFileSize(maxSize)})
        {/if}
      </p>
    </div>
  {/if}
</div>

<style>
  .drop-zone {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 150px;
    padding: 2rem;
    border: var(--fu-border);
    border-radius: var(--fu-radius);
    background: var(--fu-bg);
    cursor: pointer;
    transition:
      border-color 0.2s,
      background-color 0.2s;
  }

  .drop-zone:hover[aria-disabled="false"] {
    background: var(--fu-bg-hover);
  }

  .drop-zone[aria-disabled="true"] {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .drop-zone.dragging {
    border: var(--fu-border-active);
    background: var(--fu-bg-hover);
  }

  .drop-zone-content {
    text-align: center;
  }

  .drop-zone-icon {
    color: var(--color-text-muted, #64748b);
    margin-bottom: 0.5rem;
  }

  .drop-zone-text {
    margin: 0;
    font-size: 0.875rem;
    color: var(--color-text-muted, #64748b);
  }

  .drop-zone-link {
    color: var(--color-primary, #3b82f6);
    font-weight: 500;
  }

  .drop-zone-hint {
    margin: 0.25rem 0 0;
    font-size: 0.75rem;
    color: var(--color-text-muted, #64748b);
  }

  .visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
