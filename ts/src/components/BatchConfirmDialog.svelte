<script lang="ts">
  /**
   * BatchConfirmDialog - A confirmation dialog for batch actions.
   *
   * Displays dynamic content based on selection count and action type.
   * Integrates with useBatchActions for standardized batch action flows.
   *
   * @example
   * ```svelte
   * <BatchConfirmDialog
   *   bind:open={showConfirm}
   *   title="Delete Items"
   *   description={`Are you sure you want to delete ${count} items?`}
   *   confirmLabel="Delete"
   *   onConfirm={handleDelete}
   *   onCancel={() => showConfirm = false}
   * />
   * ```
   */
  import { AlertDialog as PoodleAlertDialog } from "@poodle/svelte-primitives";

  interface Props {
    /** Whether the dialog is open */
    open: boolean;
    /** Dialog title */
    title: string;
    /** Dialog description - displayed below title */
    description: string;
    /** Confirm button label */
    confirmLabel?: string;
    /** Cancel button label */
    cancelLabel?: string;
    /** Confirm button visual variant */
    confirmVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    /** Cancel button visual variant */
    cancelVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    /** Callback when confirmed */
    onConfirm: () => void;
    /** Callback when cancelled or dialog closed */
    onCancel: () => void;
  }

  let {
    open = $bindable(false),
    title,
    description,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    confirmVariant = "primary",
    cancelVariant = "subtle",
    onConfirm,
    onCancel
  }: Props = $props();
</script>

<PoodleAlertDialog
  bind:open
  {title}
  {description}
  {confirmLabel}
  {cancelLabel}
  tone={confirmVariant === "danger" || confirmVariant === "danger-subtle" ? "danger" : "warning"}
  onConfirm={() => {
    onConfirm();
  }}
  onCancel={() => {
    onCancel();
  }}
/>
