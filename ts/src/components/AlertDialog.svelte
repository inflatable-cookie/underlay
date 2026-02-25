<script lang="ts">
  import type { Snippet } from "svelte";
  import { tick } from "svelte";
  import { AlertDialog as BitsAlertDialog } from "bits-ui";
  import TextButton from "./TextButton.svelte";

  interface Props {
    open?: boolean;
    title: string;
    description?: string | null;
    showTrigger?: boolean;
    triggerLabel?: string;
    triggerAriaLabel?: string | null;
    triggerType?: "button" | "submit" | "reset";
    confirmLabel?: string;
    cancelLabel?: string;
    confirmVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    cancelVariant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
    contentClassName?: string;
    overlayClassName?: string;
    trapFocus?: boolean;
    preventScroll?: boolean;
    returnFocusOnClose?: boolean;
    trigger?: Snippet;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    title,
    description = null,
    showTrigger = true,
    triggerLabel = "Open",
    triggerAriaLabel = null,
    triggerType = "button",
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    confirmVariant = "danger",
    cancelVariant = "subtle",
    onConfirm,
    onCancel,
    contentClassName = "",
    overlayClassName = "",
    trapFocus,
    preventScroll,
    returnFocusOnClose = true,
    trigger,
    children
  }: Props = $props();

  let triggerRef: HTMLElement | null = $state(null);
  let lastOpen = $state(open);

  $effect(() => {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });

  let confirming = $state(false);

  async function handleConfirm() {
    if (confirming) return;
    confirming = true;

    try {
      await onConfirm?.();
      open = false;
    } catch {
      // Keep open so the user can retry.
    } finally {
      confirming = false;
    }
  }

  function handleCancel() {
    onCancel?.();
    open = false;
  }

  function getCancelTextVariant(): "default" | "danger" | "success" {
    if (cancelVariant === "danger" || cancelVariant === "danger-subtle") {
      return "danger";
    }

    return "default";
  }

</script>

<BitsAlertDialog.Root bind:open>
  {#if showTrigger}
    <BitsAlertDialog.Trigger
      bind:ref={triggerRef}
      class="underlay-alert-dialog-trigger"
      type={triggerType}
      aria-label={triggerAriaLabel ?? undefined}
    >
      {#if trigger}
        {@render trigger()}
      {:else}
        {triggerLabel}
      {/if}
    </BitsAlertDialog.Trigger>
  {/if}

  <BitsAlertDialog.Portal>
    <BitsAlertDialog.Overlay
      class={`underlay-alert-dialog-overlay ${overlayClassName}`}
    />

    <BitsAlertDialog.Content
      class={`underlay-alert-dialog-content ${contentClassName}`}
      {trapFocus}
      {preventScroll}
    >
      <div class="underlay-alert-dialog-header">
        <BitsAlertDialog.Title class="underlay-alert-dialog-title">
          {title}
        </BitsAlertDialog.Title>
        {#if description}
          <BitsAlertDialog.Description class="underlay-alert-dialog-description">
            {description}
          </BitsAlertDialog.Description>
        {/if}
      </div>

      <div class="underlay-alert-dialog-body">
        {@render children?.()}
      </div>

      <div class="underlay-alert-dialog-footer">
        <TextButton
          type="button"
          variant={getCancelTextVariant()}
          class="underlay-alert-dialog-cancel"
          disabled={confirming}
          onclick={handleCancel}
        >
          {cancelLabel}
        </TextButton>
        <BitsAlertDialog.Action
          class={`underlay-alert-dialog-action underlay-button underlay-button--pill underlay-button--${confirmVariant}`}
          disabled={confirming}
          onclick={handleConfirm}
        >
          {confirming ? "Working…" : confirmLabel}
        </BitsAlertDialog.Action>
      </div>
    </BitsAlertDialog.Content>
  </BitsAlertDialog.Portal>
</BitsAlertDialog.Root>

<style>
  :global(.underlay-alert-dialog-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    border-radius: 0.5rem;
    padding: 0.5rem 0.875rem;

    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35))
      );

    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(255, 255, 255, 0.03))
    );

    color: inherit;
    cursor: pointer;
  }

  :global(.underlay-alert-dialog-trigger:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.08))
    );
  }

  :global(.underlay-alert-dialog-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-alert-dialog-overlay) {
    position: fixed;
    inset: 0;
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
    z-index: 50;
  }

  :global(.underlay-alert-dialog-content) {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 51;

    width: min(42rem, calc(100vw - 2rem));
    max-height: min(80vh, 48rem);
    overflow: auto;

    border-radius: 0.75rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );
    background: var(
      --underlay-color-dialog-bg,
      var(--underlay-color-bg-surface, #020617)
    );
    padding: 1.25rem;

    box-shadow: var(
      --underlay-shadow-dialog,
      0 20px 40px rgba(0, 0, 0, 0.55)
    );
  }

  :global(.underlay-alert-dialog-header) {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  :global(.underlay-alert-dialog-title) {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 650;
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-alert-dialog-description) {
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    font-size: 0.9rem;
    line-height: 1.45;
  }

  :global(.underlay-alert-dialog-body) {
    margin-top: 1rem;
  }

  :global(.underlay-alert-dialog-footer) {
    margin-top: 1.25rem;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 1.25rem;
  }

  :global(.underlay-alert-dialog-action:disabled) {
    opacity: 0.6;
    cursor: not-allowed;
  }

  @media (max-width: 40rem) {
    :global(.underlay-alert-dialog-content) {
      width: calc(100vw - 1.25rem);
      padding: 1rem;
      border-radius: 0.625rem;
    }

    :global(.underlay-alert-dialog-footer) {
      justify-content: stretch;
    }

    :global(.underlay-alert-dialog-action.underlay-button) {
      flex: 1 1 auto;
      justify-content: center;
    }
  }
</style>
