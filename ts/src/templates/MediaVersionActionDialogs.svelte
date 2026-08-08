<script lang="ts" generics="TVersion extends { id: string; sha256?: string | null }">
  import { AlertDialog } from "@inflatable-cookie/poodle-svelte";

  interface Props {
    activateDialogOpen: boolean;
    deleteDialogOpen: boolean;
    selectedVersion: TVersion | null;
    activateTitle?: string;
    activateDescription?: string;
    activateConfirmLabel?: string;
    deleteTitle?: string;
    deleteDescription?: string;
    deleteConfirmLabel?: string;
    cancelLabel?: string;
    activateItemLabel?: string | null;
    deleteItemLabel?: string | null;
    onConfirmActivate: () => void;
    onCancelActivate: () => void;
    onConfirmDelete: () => void;
    onCancelDelete: () => void;
  }

  let {
    activateDialogOpen = $bindable(),
    deleteDialogOpen = $bindable(),
    selectedVersion,
    activateTitle = "Activate version?",
    activateDescription = "This will set this version as the current active version for this media item.",
    activateConfirmLabel = "Activate version",
    deleteTitle = "Permanently delete version?",
    deleteDescription = "This will permanently delete this version and its stored file. This cannot be undone.",
    deleteConfirmLabel = "Permanently delete version",
    cancelLabel = "Cancel",
    activateItemLabel = "Version",
    deleteItemLabel = "Version",
    onConfirmActivate,
    onCancelActivate,
    onConfirmDelete,
    onCancelDelete
  }: Props = $props();

  const selectedVersionLabel = $derived(
    selectedVersion ? `${selectedVersion.sha256?.slice(0, 16) ?? selectedVersion.id}...` : null
  );
</script>

<AlertDialog
  bind:open={activateDialogOpen}
  title={activateTitle}
  description={activateDescription}
  itemLabel={activateItemLabel}
  itemValue={selectedVersionLabel}
  confirmLabel={activateConfirmLabel}
  cancelLabel={cancelLabel}
  tone="warning"
  onConfirm={onConfirmActivate}
  onCancel={onCancelActivate}
/>

<AlertDialog
  bind:open={deleteDialogOpen}
  title={deleteTitle}
  description={deleteDescription}
  itemLabel={deleteItemLabel}
  itemValue={selectedVersionLabel}
  confirmLabel={deleteConfirmLabel}
  cancelLabel={cancelLabel}
  tone="danger"
  onConfirm={onConfirmDelete}
  onCancel={onCancelDelete}
/>
