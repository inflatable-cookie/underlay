<script lang="ts" generics="TVersion extends { url?: string | null }">
  import { Dialog as PoodleDialog } from "@poodle/svelte";

  interface Props {
    open: boolean;
    previewVersion: TVersion | null;
    mediaKind: string;
    getPreviewUrl: (version: TVersion) => string | null;
    isImage: (kind: string) => boolean;
    isPdf: (kind: string) => boolean;
    title?: string;
    unavailableMessage?: string;
    unsupportedMessage?: string;
  }

  let {
    open = $bindable(),
    previewVersion,
    mediaKind,
    getPreviewUrl,
    isImage,
    isPdf,
    title = "Version preview",
    unavailableMessage = "Preview not available for this version.",
    unsupportedMessage = "Preview not available for this file type."
  }: Props = $props();

  function closePreview(): void {
    open = false;
  }
</script>

<PoodleDialog
  bind:open
  {title}
  contentClassName="underlay-media-version-preview-dialog"
  showCloseButton
  closeButtonSize="xs"
  onRequestClose={closePreview}
  onOpenChange={(nextOpen) => {
    open = nextOpen;
  }}
>
  {#if previewVersion}
    {@const previewUrl = getPreviewUrl(previewVersion)}
    {#if previewUrl}
      {#if isImage(mediaKind)}
        <img
          class="underlay-media-version-preview-dialog__image"
          src={previewUrl}
          alt="Version preview"
        />
      {:else if isPdf(mediaKind)}
        <iframe
          class="underlay-media-version-preview-dialog__frame"
          title="Version preview"
          src={previewUrl}
        ></iframe>
      {:else}
        <p>{unsupportedMessage}</p>
      {/if}
    {:else}
      <p>{unavailableMessage}</p>
    {/if}
  {/if}
</PoodleDialog>

<style>
  :global(.underlay-media-version-preview-dialog) {
    max-width: min(92vw, 1000px);
  }

  :global(.underlay-media-version-preview-dialog .poodle-dialog__header-row) {
    align-items: center;
  }

  :global(.underlay-media-version-preview-dialog .poodle-dialog__close) {
    margin-top: 0;
    margin-right: -0.125rem;
  }

  .underlay-media-version-preview-dialog__image {
    display: block;
    max-width: 100%;
    max-height: 70vh;
    margin: 0 auto;
    border-radius: 0.5rem;
  }

  .underlay-media-version-preview-dialog__frame {
    width: 100%;
    height: 70vh;
    border: 0;
    border-radius: 0.5rem;
    background: var(--color-surface-subtle, #f8fafc);
  }
</style>
