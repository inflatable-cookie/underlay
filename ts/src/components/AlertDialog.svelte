<script lang="ts">
  import type { Snippet } from "svelte";
  import { tick } from "svelte";
  import { AlertDialog as BitsAlertDialog } from "bits-ui";

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
        <BitsAlertDialog.Cancel
          class="underlay-alert-dialog-cancel"
          disabled={confirming}
          onclick={handleCancel}
        >
          {cancelLabel}
        </BitsAlertDialog.Cancel>
        <BitsAlertDialog.Action
          class="underlay-alert-dialog-action"
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

    width: min(40rem, calc(100vw - 2rem));
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
    padding: 1rem;

    box-shadow: var(
      --underlay-shadow-dialog,
      0 20px 40px rgba(0, 0, 0, 0.55)
    );
  }

  :global(.underlay-alert-dialog-header) {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
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
  }

  :global(.underlay-alert-dialog-body) {
    margin-top: 0.9rem;
  }

  :global(.underlay-alert-dialog-footer) {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  :global(.underlay-alert-dialog-cancel),
  :global(.underlay-alert-dialog-action) {
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

    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    cursor: pointer;
  }

  :global(.underlay-alert-dialog-cancel:hover),
  :global(.underlay-alert-dialog-action:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.08))
    );
  }

  :global(.underlay-alert-dialog-cancel:focus-visible),
  :global(.underlay-alert-dialog-action:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-alert-dialog-action) {
    border-color: rgba(239, 68, 68, 0.5);
    background: rgba(239, 68, 68, 0.16);
  }

  :global(.underlay-alert-dialog-cancel:disabled),
  :global(.underlay-alert-dialog-action:disabled) {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
