<script lang="ts">
  import { Button, Callout, Progress } from "@poodle/svelte";
  import type { MediaUploadDisplayStep } from "../patterns/media-workflow";

  interface Props {
    uploadStep: MediaUploadDisplayStep;
    uploadProgress?: number;
    uploadError?: string | null;
    duplicateLabel?: string | null;
    uploadedLabel?: string | null;
    onUploadAnyway?: () => void;
    onSelectDuplicate?: () => void;
    onClearUpload?: () => void;
    onSelectUploaded?: () => void;
  }

  let {
    uploadStep,
    uploadProgress = 0,
    uploadError = null,
    duplicateLabel = null,
    uploadedLabel = null,
    onUploadAnyway,
    onSelectDuplicate,
    onClearUpload,
    onSelectUploaded
  }: Props = $props();

  const title = $derived.by(() => {
    switch (uploadStep) {
      case "checking":
        return "Checking file";
      case "uploading":
        return "Uploading file";
      case "finalising":
        return "Finalising upload";
      case "duplicate":
        return "Duplicate found";
      case "complete":
        return "Upload complete";
      case "error":
        return "Upload failed";
      default:
        return "Ready to upload";
    }
  });

  const busy = $derived(
    uploadStep === "checking" || uploadStep === "uploading" || uploadStep === "finalising"
  );
  const progressValue = $derived(
    uploadStep === "uploading" ? uploadProgress : uploadStep === "complete" ? 100 : 0
  );
</script>

<section class="underlay-media-upload-status-panel" aria-live="polite">
  <div class="underlay-media-upload-status-panel__header">
    <h3>{title}</h3>
    {#if uploadStep === "uploading"}
      <span>{Math.round(uploadProgress)}%</span>
    {/if}
  </div>

  {#if busy || uploadStep === "complete"}
    <Progress value={progressValue} max={100} ariaLabel="Media upload progress" />
  {/if}

  {#if uploadStep === "duplicate"}
    <Callout
      tone="warning"
      title="This file already exists"
      message={duplicateLabel ? `Duplicate of ${duplicateLabel}.` : "A matching media item already exists."}
    />
    <div class="underlay-media-upload-status-panel__actions">
      {#if onClearUpload}
        <Button type="button" variant="ghost" onClick={onClearUpload}>Choose another file</Button>
      {/if}
      {#if onUploadAnyway}
        <Button type="button" variant="secondary" onClick={onUploadAnyway}>Upload anyway</Button>
      {/if}
      {#if onSelectDuplicate}
        <Button type="button" variant="primary" onClick={onSelectDuplicate}>Use existing</Button>
      {/if}
    </div>
  {:else if uploadStep === "complete"}
    <Callout
      tone="success"
      title="Upload complete"
      message={uploadedLabel ? `${uploadedLabel} is ready.` : "The media item is ready."}
    />
    <div class="underlay-media-upload-status-panel__actions">
      {#if onClearUpload}
        <Button type="button" variant="ghost" onClick={onClearUpload}>Upload another</Button>
      {/if}
      {#if onSelectUploaded}
        <Button type="button" variant="primary" onClick={onSelectUploaded}>Use uploaded</Button>
      {/if}
    </div>
  {:else if uploadStep === "error"}
    <Callout
      tone="danger"
      title="Upload failed"
      message={uploadError ?? "The file could not be uploaded."}
    />
    {#if onClearUpload}
      <div class="underlay-media-upload-status-panel__actions">
        <Button type="button" variant="primary" onClick={onClearUpload}>Try again</Button>
      </div>
    {/if}
  {/if}
</section>

<style>
  .underlay-media-upload-status-panel {
    display: grid;
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
  }

  .underlay-media-upload-status-panel__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
  }

  .underlay-media-upload-status-panel__header h3 {
    margin: 0;
    font-size: var(--poodle-typography-heading-4-size);
    line-height: var(--poodle-typography-heading-line-height);
  }

  .underlay-media-upload-status-panel__actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--poodle-space-inline-sm);
  }
</style>
