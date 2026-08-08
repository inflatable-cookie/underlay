<script lang="ts">
  import { useToasts } from "../runtime/feedback";
  import type { NavigationContext } from "../runtime/navigation";
  import { AlertDialog } from "@inflatable-cookie/poodle-svelte";
  import EntityActionsMenu from "./EntityActionsMenu.svelte";
  import type { MediaActionsMenuItem } from "./template.types";
  import type { TemplateSurface } from "./template-types/primitives";

  interface Props {
    media: MediaActionsMenuItem;
    sourceContext?: NavigationContext;
    trigger?: TemplateSurface;
    softDeleteAction?: (media: MediaActionsMenuItem) => Promise<void>;
    restoreAction?: (media: MediaActionsMenuItem) => Promise<void>;
    purgeAction?: (media: MediaActionsMenuItem) => Promise<void>;
    onReplaceRequest?: (media: MediaActionsMenuItem, context: NavigationContext) => void;
    onSoftDeleteSuccess?: () => void;
    onRestoreSuccess?: () => void;
    onPurgeSuccess?: () => void;
    onEditRequest?: () => void;
  }

  let {
    media,
    sourceContext,
    trigger,
    softDeleteAction,
    restoreAction,
    purgeAction,
    onReplaceRequest,
    onSoftDeleteSuccess,
    onRestoreSuccess,
    onPurgeSuccess,
    onEditRequest
  }: Props = $props();

  const toastStore = useToasts();
  const mediaDisplayName = $derived(media.title || media.originalFilename || "Untitled");
  const isDeleted = $derived(media.deletedAt !== null);
  const defaultContext = $derived<NavigationContext>({
    label: "Media",
    href: `/media/${media.id}`,
    type: "detail"
  });
  const replaceContext = $derived(sourceContext ?? defaultContext);

  let restoreOpen = $state(false);

  const copies = $derived(
    media.originalFilename
      ? [
          {
            label: "Copy ID",
            text: media.id,
            successMessage: "Copied ID",
            failureMessage: "Failed to copy ID"
          },
          {
            label: "Copy filename",
            text: media.originalFilename,
            successMessage: "Copied filename",
            failureMessage: "Failed to copy filename"
          }
        ]
      : [
          {
            label: "Copy ID",
            text: media.id,
            successMessage: "Copied ID",
            failureMessage: "Failed to copy ID"
          }
        ]
  );

  const customActions = $derived.by(() => {
    const actions: Array<{
      label: string;
      destructive?: boolean;
      onSelect: () => void | Promise<void>;
    }> = [];

    if (!isDeleted && onReplaceRequest) {
      actions.push({
        label: "Replace file",
        onSelect: () => onReplaceRequest(media, replaceContext)
      });
    }
    if (isDeleted && restoreAction) {
      actions.push({
        label: "Restore media",
        onSelect: () => {
          restoreOpen = true;
        }
      });
    }

    return actions;
  });

  const deleteConfig = $derived.by(() => {
    if (!isDeleted && softDeleteAction) {
      return {
        entityLabel: mediaDisplayName,
        title: "Move media to trash?",
        description:
          "This hides the media from the main library. You can restore it later from trash.",
        confirmLabel: "Move to trash",
        execute: async () => {
          try {
            await softDeleteAction(media);
          } catch (error) {
            console.error("Failed to move media to trash", error);
            throw new Error("Failed to move media to trash");
          }
        }
      };
    }
    if (isDeleted && purgeAction) {
      return {
        entityLabel: mediaDisplayName,
        title: "Permanently delete media?",
        description: "This removes the media and all versions permanently. This cannot be undone.",
        confirmLabel: "Permanently delete",
        execute: async () => {
          try {
            await purgeAction(media);
          } catch (error) {
            console.error("Failed to permanently delete media", error);
            throw new Error("Failed to permanently delete media");
          }
        }
      };
    }
    return undefined;
  });

  function handleDeleteSuccess() {
    if (isDeleted) {
      toastStore.push({ variant: "success", message: "Media permanently deleted" });
      onPurgeSuccess?.();
    } else {
      toastStore.push({ variant: "success", message: "Media moved to trash" });
      onSoftDeleteSuccess?.();
    }
  }

  async function confirmRestore() {
    if (!restoreAction) return;

    try {
      await restoreAction(media);
      restoreOpen = false;
      toastStore.push({ variant: "success", message: "Media restored" });
      onRestoreSuccess?.();
    } catch (error) {
      console.error("Failed to restore media", error);
      toastStore.push({ variant: "error", message: "Failed to restore media" });
    }
  }
</script>

<EntityActionsMenu
  {copies}
  onEdit={!isDeleted ? onEditRequest : undefined}
  editLabel="Edit"
  {customActions}
  {deleteConfig}
  onDeleteSuccess={handleDeleteSuccess}
  triggerAriaLabel="Media actions"
  triggerTooltip="Actions"
>
  {#if trigger}
    {#snippet trigger()}
      {@render trigger()}
    {/snippet}
  {/if}
</EntityActionsMenu>

<AlertDialog
  bind:open={restoreOpen}
  title="Restore media?"
  description="This returns the media to the active library."
  confirmLabel="Restore"
  cancelLabel="Cancel"
  onConfirm={confirmRestore}
  onCancel={() => (restoreOpen = false)}
  tone="warning"
>
  <p>Media: <strong>{mediaDisplayName}</strong></p>
</AlertDialog>
