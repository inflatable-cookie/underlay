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
  class="underlay-drop-zone"
  class:underlay-dragging={isDragging}
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
    class="underlay-visually-hidden"
    onchange={onInputChange}
    aria-label="File upload"
  />

  {#if dropzone}
    {@render dropzone()}
  {:else}
    <div class="underlay-drop-zone-content">
      <div class="underlay-drop-zone-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
      </div>
      <p class="underlay-drop-zone-text">
        {#if isDragging}
          Drop files here
        {:else}
          <span class="underlay-drop-zone-link">Click to upload</span> or drag and drop
        {/if}
      </p>
      <p class="underlay-drop-zone-hint">
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
  .underlay-drop-zone {
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

  .underlay-drop-zone:hover[aria-disabled="false"] {
    background: var(--fu-bg-hover);
  }

  .underlay-drop-zone[aria-disabled="true"] {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .underlay-drop-zone.underlay-dragging {
    border: var(--fu-border-active);
    background: var(--fu-bg-hover);
  }

  .underlay-drop-zone-content {
    text-align: center;
  }

  .underlay-drop-zone-icon {
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.5rem;
  }

  .underlay-drop-zone-text {
    margin: 0;
    font-size: 0.875rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-drop-zone-link {
    color: var(--underlay-color-primary, #3b82f6);
    font-weight: 500;
  }

  .underlay-drop-zone-hint {
    margin: 0.25rem 0 0;
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-visually-hidden {
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
