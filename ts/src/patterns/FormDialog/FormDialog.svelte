<script lang="ts">
  import type { Snippet } from "svelte";
  import { Dialog as BitsDialog } from "bits-ui";
  import X from "lucide-svelte/icons/x";

  interface Props {
    /** Whether the dialog is open */
    open?: boolean;
    /** Dialog title */
    title: string;
    /** Optional subtitle/description */
    subtitle?: string;
    /** Whether form is currently submitting */
    submitting?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Success message to display */
    success?: string | null;
    /** Width of the dialog (default: 36rem) */
    width?: string;
    /** Called when cancel/close is requested */
    onCancel?: () => void;
    /** Rich subtitle content snippet - rendered in the header below the title */
    subtitleContent?: Snippet;
    /** Form content snippet - receives (submitting: boolean) */
    children?: Snippet<[boolean]>;
  }

  let {
    open = $bindable(false),
    title,
    subtitle = undefined,
    submitting = false,
    error = null,
    success = null,
    width = "36rem",
    onCancel,
    subtitleContent,
    children
  }: Props = $props();

  function handleClose() {
    if (!submitting) {
      onCancel?.();
    }
  }

  function handleOpenChange(isOpen: boolean) {
    if (!isOpen && !submitting) {
      onCancel?.();
    }
  }
</script>

<BitsDialog.Root bind:open onOpenChange={handleOpenChange}>
  <BitsDialog.Portal>
    <BitsDialog.Overlay class="form-dialog__overlay" />

    <BitsDialog.Content
      class="form-dialog__content"
      style="--form-dialog-width: {width}"
    >
      <div class="form-dialog__header">
        <div class="form-dialog__titles">
          <BitsDialog.Title class="form-dialog__title">{title}</BitsDialog.Title>
          {#if subtitleContent}
            <BitsDialog.Description class="form-dialog__subtitle">
              {@render subtitleContent()}
            </BitsDialog.Description>
          {:else if subtitle}
            <BitsDialog.Description class="form-dialog__subtitle">
              {subtitle}
            </BitsDialog.Description>
          {/if}
        </div>

        <button
          type="button"
          class="form-dialog__close"
          onclick={handleClose}
          disabled={submitting}
          aria-label="Close"
        >
          <X size={18} />
        </button>
      </div>

      {#if error}
        <div class="form-dialog__error" role="alert">
          {error}
        </div>
      {/if}

      {#if success}
        <div class="form-dialog__success" role="status">
          {success}
        </div>
      {/if}

      <div class="form-dialog__body">
        {@render children?.(submitting)}
      </div>
    </BitsDialog.Content>
  </BitsDialog.Portal>
</BitsDialog.Root>

<style>
  /* All styles must be global because BitsDialog.Portal renders outside the component tree */
  :global(.form-dialog__overlay) {
    position: fixed;
    inset: 0;
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
    z-index: 50;
  }

  :global(.form-dialog__overlay[data-state="open"]) {
    animation:
      underlay-form-dialog-overlay-in
      var(--underlay-dialog-overlay-in-duration, 160ms)
      var(--underlay-dialog-overlay-in-easing, ease-out);
  }

  :global(.form-dialog__overlay[data-state="closed"]) {
    animation:
      underlay-form-dialog-overlay-out
      var(--underlay-dialog-overlay-out-duration, 120ms)
      var(--underlay-dialog-overlay-out-easing, ease-in);
  }

  :global(.form-dialog__content) {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 51;

    width: min(var(--form-dialog-width, 36rem), calc(100vw - 2rem));
    max-height: min(85vh, 52rem);
    overflow: auto;

    border-radius: var(--underlay-radius-md, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background-color: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    padding: var(--underlay-card-padding, 1rem);

    box-shadow: var(--underlay-shadow-dialog, 0 20px 40px rgba(0, 0, 0, 0.55));
  }

  :global(.form-dialog__content[data-state="open"]) {
    animation:
      underlay-form-dialog-content-in
      var(--underlay-dialog-content-in-duration, 190ms)
      var(--underlay-dialog-content-in-easing, cubic-bezier(0.16, 1, 0.3, 1));
  }

  :global(.form-dialog__content[data-state="closed"]) {
    animation:
      underlay-form-dialog-content-out
      var(--underlay-dialog-content-out-duration, 130ms)
      var(--underlay-dialog-content-out-easing, ease-in);
  }

  :global(.form-dialog__header) {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  :global(.form-dialog__titles) {
    flex: 1;
    min-width: 0;
  }

  :global(.form-dialog__title) {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.form-dialog__subtitle) {
    margin: 0.25rem 0 0;
    font-size: 0.875rem;
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.form-dialog__close) {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    padding: 0;
    border: none;
    border-radius: 0.375rem;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
  }

  :global(.form-dialog__close:hover:not(:disabled)) {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.08));
    color: var(--underlay-color-text, #e5e7eb);
  }

  :global(.form-dialog__close:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  :global(.form-dialog__error) {
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 0.5rem;
    background: var(--underlay-color-danger-muted, rgba(239, 68, 68, 0.15));
    color: var(--underlay-color-danger, #ef4444);
    font-size: 0.875rem;
  }

  :global(.form-dialog__success) {
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 0.5rem;
    background: var(--underlay-color-success-muted, rgba(34, 197, 94, 0.15));
    color: var(--underlay-color-success, #22c55e);
    font-size: 0.875rem;
  }

  /* .form-dialog__body: Form content renders here, no additional global styles needed */

  @keyframes underlay-form-dialog-overlay-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes underlay-form-dialog-overlay-out {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }

  @keyframes underlay-form-dialog-content-in {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }

  @keyframes underlay-form-dialog-content-out {
    from {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
    to {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.985);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(.form-dialog__overlay),
    :global(.form-dialog__content) {
      animation: none !important;
    }
  }
</style>
