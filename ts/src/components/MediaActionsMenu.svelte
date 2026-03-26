<script lang="ts">
  /**
   * MediaActionsMenu - Context menu for media item actions.
   *
   * This is a generic component that accepts callback props for media operations,
   * allowing it to work with any backend implementation.
   *
   * Usage:
   * ```svelte
   * <MediaActionsMenu
   *   {media}
   *   softDelete={...}
   *   restore={...}
   *   purge={...}
   *   onSoftDeleteSuccess={() => refetch()}
   * />
   * ```
  */
  import type { Snippet } from "svelte";
  import { AlertDialog as PoodleAlertDialog } from "@poodle/svelte-primitives";
  import {
    CopyActionsMenu,
    useToasts,
    type NavigationContext,
  } from "../patterns/index.js";
  import {
    type MediaSummary,
    type MediaDetail,
    getMediaDisplayName,
  } from "../patterns/media-types.js";
  type MediaItem = MediaSummary | MediaDetail;

  interface Props {
    /** Media data */
    media: MediaItem;
    /** Navigation context for actions */
    sourceContext?: NavigationContext;
    /** Optional custom trigger snippet (for ListCard usage) */
    trigger?: Snippet;

    // =========================================================================
    // Media operation callbacks
    // =========================================================================

    /** Soft delete the media item */
    softDelete?: (mediaId: string) => Promise<void>;

    /** Restore a soft-deleted media item */
    restore?: (mediaId: string) => Promise<void>;

    /** Permanently delete the media item */
    purge?: (mediaId: string) => Promise<void>;

    /** Navigate to replace file page */
    navigateToReplace?: (mediaId: string) => void;

    // =========================================================================
    // Events
    // =========================================================================

    /** Callback after successful soft delete */
    onSoftDeleteSuccess?: () => void;

    /** Callback after successful restore */
    onRestoreSuccess?: () => void;

    /** Callback after successful purge */
    onPurgeSuccess?: () => void;

    /** Callback when edit is requested */
    onEditRequest?: () => void;
  }

  let {
    media,
    sourceContext,
    trigger,
    softDelete,
    restore,
    purge,
    navigateToReplace,
    onSoftDeleteSuccess,
    onRestoreSuccess,
    onPurgeSuccess,
    onEditRequest,
  }: Props = $props();

  const toastStore = useToasts();
  const mediaDisplayName = $derived(getMediaDisplayName(media));
  const isDeleted = $derived(media.deletedAt !== null);

  // Soft delete state
  let softDeleteOpen = $state(false);

  function requestSoftDelete() {
    softDeleteOpen = true;
  }

  async function confirmSoftDelete() {
    if (!softDelete) return;

    try {
      await softDelete(media.id);
      softDeleteOpen = false;
      toastStore.push({ variant: "success", message: "Media soft-deleted" });
      onSoftDeleteSuccess?.();
    } catch (e) {
      console.error("Failed to soft-delete media", e);
      toastStore.push({
        variant: "error",
        message: "Failed to soft-delete media",
      });
    }
  }

  // Restore state
  let restoreOpen = $state(false);

  function requestRestore() {
    restoreOpen = true;
  }

  async function confirmRestore() {
    if (!restore) return;

    try {
      await restore(media.id);
      restoreOpen = false;
      toastStore.push({ variant: "success", message: "Media restored" });
      onRestoreSuccess?.();
    } catch (e) {
      console.error("Failed to restore media", e);
      toastStore.push({ variant: "error", message: "Failed to restore media" });
    }
  }

  // Purge state
  let purgeOpen = $state(false);

  function requestPurge() {
    purgeOpen = true;
  }

  async function confirmPurge() {
    if (!purge) return;

    try {
      await purge(media.id);
      purgeOpen = false;
      toastStore.push({
        variant: "success",
        message: "Media permanently deleted",
      });
      onPurgeSuccess?.();
    } catch (e) {
      console.error("Failed to purge media", e);
      toastStore.push({
        variant: "error",
        message: "Failed to permanently delete media",
      });
    }
  }

  // Build copies array
  const copies = $derived([
    { label: "Copy ID", text: media.id, successMessage: "Copied ID" },
    ...(media.originalFilename
      ? [
          {
            label: "Copy filename",
            text: media.originalFilename,
            successMessage: "Copied filename",
          },
        ]
      : []),
  ]);

  // Build actions array based on deleted state
  const actions = $derived(
    isDeleted
      ? [
          ...(restore
            ? [
                {
                  label: "Restore media",
                  onSelect: () => requestRestore(),
                },
              ]
            : []),
          ...(purge
            ? [
                {
                  label: "Permanently delete",
                  destructive: true,
                  onSelect: () => requestPurge(),
                },
              ]
            : []),
        ]
      : [
          ...(onEditRequest
            ? [{ label: "Edit", onSelect: () => onEditRequest() }]
            : []),
          ...(navigateToReplace
            ? [
                {
                  label: "Replace file",
                  onSelect: () => navigateToReplace(media.id),
                },
              ]
            : []),
          ...(softDelete
            ? [
                {
                  label: "Soft delete",
                  destructive: true,
                  onSelect: () => requestSoftDelete(),
                },
              ]
            : []),
        ]
  );
</script>

<CopyActionsMenu {toastStore} {trigger} {copies} {actions} />

<!-- Soft Delete Dialog -->
{#if softDelete}
  <PoodleAlertDialog
    bind:open={softDeleteOpen}
    title="Soft delete media?"
    description="Soft deleting will hide this media from listings. You can restore it later from trash."
    confirmLabel="Soft delete"
    cancelLabel="Cancel"
    onConfirm={confirmSoftDelete}
    onCancel={() => (softDeleteOpen = false)}
    tone="danger"
  >
    <p>
      Media: <strong>{mediaDisplayName}</strong>
    </p>
  </PoodleAlertDialog>
{/if}

<!-- Restore Dialog -->
{#if restore}
  <PoodleAlertDialog
    bind:open={restoreOpen}
    title="Restore media?"
    description="This will restore the media back to the library."
    confirmLabel="Restore"
    cancelLabel="Cancel"
    onConfirm={confirmRestore}
    onCancel={() => (restoreOpen = false)}
    tone="warning"
  >
    <p>
      Media: <strong>{mediaDisplayName}</strong>
    </p>
  </PoodleAlertDialog>
{/if}

<!-- Purge Dialog -->
{#if purge}
  <PoodleAlertDialog
    bind:open={purgeOpen}
    title="Permanently delete media?"
    description="This will permanently delete the media and all its versions. This cannot be undone."
    confirmLabel="Permanently delete"
    cancelLabel="Cancel"
    onConfirm={confirmPurge}
    onCancel={() => (purgeOpen = false)}
    tone="danger"
  >
    <p>
      Media: <strong>{mediaDisplayName}</strong>
    </p>
  </PoodleAlertDialog>
{/if}
