<script lang="ts">
  import { AlertDialog as PoodleAlertDialog, Button } from "@poodle/svelte-primitives";
  import type { Snippet } from "svelte";

  interface Props {
    open?: boolean;
    triggerLabel?: string;
    triggerVariant?: "default" | "danger";
    title: string;
    description?: string | null;
    confirmLabel?: string;
    cancelLabel?: string;
    confirmVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    cancelVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
    trigger?: Snippet;
  }

  let {
    open = $bindable(false),
    triggerLabel = "Open",
    triggerVariant = "default",
    title,
    description = null,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    confirmVariant = triggerVariant === "danger" ? "danger" : "primary",
    cancelVariant = "subtle",
    onConfirm,
    onCancel,
    trigger
  }: Props = $props();
</script>

<Button
  type="button"
  variant="ghost"
  tone={triggerVariant === "danger" ? "danger" : "default"}
  on:click={() => (open = true)}
>
  {#if trigger}
    {@render trigger()}
  {:else}
    {triggerLabel}
  {/if}
</Button>

<PoodleAlertDialog
  bind:open
  {title}
  {description}
  {confirmLabel}
  {cancelLabel}
  {onConfirm}
  {onCancel}
  tone={confirmVariant === "danger" || confirmVariant === "danger-subtle" ? "danger" : "warning"}
/>
