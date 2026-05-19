<script lang="ts">
  import type { Snippet } from "svelte";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import type { NavigationContext } from "../runtime/navigation";
  import { AlertDialog, Button, Menu, type MenuItem } from "@poodle/svelte";
  import type { MediaActionsMenuItem } from "./template.types";

  type ActionEntry =
    | {
        key: string;
        label: string;
        tone?: "default" | "danger";
        onSelect: () => void | Promise<void>;
      }
    | { separator: true; key: string };

  interface Props {
    media: MediaActionsMenuItem;
    sourceContext?: NavigationContext;
    trigger?: Snippet;
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

  let softDeleteOpen = $state(false);
  let restoreOpen = $state(false);
  let purgeOpen = $state(false);

  async function handleCopy(text: string, successMessage: string, failureMessage: string) {
    await copyToClipboard(toastStore, text, successMessage, failureMessage);
  }

  async function confirmSoftDelete() {
    if (!softDeleteAction) return;

    try {
      await softDeleteAction(media);
      softDeleteOpen = false;
      toastStore.push({ variant: "success", message: "Media moved to trash" });
      onSoftDeleteSuccess?.();
    } catch (error) {
      console.error("Failed to move media to trash", error);
      toastStore.push({ variant: "error", message: "Failed to move media to trash" });
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

  async function confirmPurge() {
    if (!purgeAction) return;

    try {
      await purgeAction(media);
      purgeOpen = false;
      toastStore.push({ variant: "success", message: "Media permanently deleted" });
      onPurgeSuccess?.();
    } catch (error) {
      console.error("Failed to permanently delete media", error);
      toastStore.push({ variant: "error", message: "Failed to permanently delete media" });
    }
  }

  const menuEntries = $derived.by<ActionEntry[]>(() => {
    const entries: ActionEntry[] = [];

    if (onEditRequest && !isDeleted) {
      entries.push({ key: "edit", label: "Edit", onSelect: () => onEditRequest() });
    }
    if (!isDeleted && onReplaceRequest) {
      entries.push({
        key: "replace",
        label: "Replace file",
        onSelect: () => onReplaceRequest(media, replaceContext)
      });
    }
    if (!isDeleted && softDeleteAction) {
      entries.push({
        key: "soft-delete",
        label: "Move to trash",
        tone: "danger",
        onSelect: () => {
          softDeleteOpen = true;
        }
      });
    }
    if (isDeleted && restoreAction) {
      entries.push({
        key: "restore",
        label: "Restore media",
        onSelect: () => {
          restoreOpen = true;
        }
      });
    }
    if (isDeleted && purgeAction) {
      entries.push({
        key: "purge",
        label: "Permanently delete",
        tone: "danger",
        onSelect: () => {
          purgeOpen = true;
        }
      });
    }

    const copyEntries: ActionEntry[] = [
      {
        key: "copy-id",
        label: "Copy ID",
        onSelect: () => handleCopy(media.id, "Copied ID", "Failed to copy ID")
      }
    ];

    if (media.originalFilename) {
      copyEntries.push({
        key: "copy-filename",
        label: "Copy filename",
        onSelect: () =>
          handleCopy(media.originalFilename!, "Copied filename", "Failed to copy filename")
      });
    }

    if (entries.length && copyEntries.length) {
      entries.push({ separator: true, key: "separator-copy" });
    }

    entries.push(...copyEntries);
    return entries;
  });

  const menuItems = $derived<MenuItem[]>(
    menuEntries.map((entry) =>
      "separator" in entry
        ? { value: entry.key, label: "", kind: "separator" }
        : {
            value: entry.key,
            label: entry.label,
            tone: entry.tone
          }
    )
  );

  async function handleAction(value: string) {
    const entry = menuEntries.find((item) => !("separator" in item) && item.key === value);
    if (entry && !("separator" in entry)) {
      await entry.onSelect();
    }
  }
</script>

<Menu
  items={menuItems}
  placement="bottom-end"
  ariaLabel="Media actions"
  triggerAriaLabel="Media actions"
  onAction={(value) => void handleAction(value)}
>
  {#snippet trigger()}
    {#if trigger}
      {@render trigger()}
    {:else}
      <Button variant="secondary">Actions</Button>
    {/if}
  {/snippet}
</Menu>

<AlertDialog
  bind:open={softDeleteOpen}
  title="Move media to trash?"
  description="This hides the media from the main library. You can restore it later from trash."
  confirmLabel="Move to trash"
  cancelLabel="Cancel"
  onConfirm={confirmSoftDelete}
  onCancel={() => (softDeleteOpen = false)}
  tone="danger"
>
  <p>Media: <strong>{mediaDisplayName}</strong></p>
</AlertDialog>

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

<AlertDialog
  bind:open={purgeOpen}
  title="Permanently delete media?"
  description="This removes the media and all versions permanently. This cannot be undone."
  confirmLabel="Delete permanently"
  cancelLabel="Cancel"
  onConfirm={confirmPurge}
  onCancel={() => (purgeOpen = false)}
  tone="danger"
>
  <p>Media: <strong>{mediaDisplayName}</strong></p>
</AlertDialog>
