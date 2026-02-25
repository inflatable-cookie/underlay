<script lang="ts">
  import type { Snippet } from "svelte";
  import { tick } from "svelte";
  import { AlertDialog as BitsAlertDialog } from "bits-ui";
  import TextButton from "./TextButton.svelte";

  interface Props {
    open?: boolean;
    title: string;
    description?: string | null;
    itemLabel?: string | null;
    itemValue?: string | null;
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
    itemLabel = null,
    itemValue = null,
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

      {#if itemLabel || itemValue || children}
        <div class="underlay-alert-dialog-body">
          {#if itemLabel || itemValue}
            <div class="underlay-alert-dialog-item-card">
              {#if itemLabel}
                <p class="underlay-alert-dialog-item-label">{itemLabel}</p>
              {/if}
              {#if itemValue}
                <p class="underlay-alert-dialog-item-value">{itemValue}</p>
              {/if}
              {#if children}
                <div class="underlay-alert-dialog-item-extra">
                  {@render children?.()}
                </div>
              {/if}
            </div>
          {:else if children}
            {@render children?.()}
          {/if}
        </div>
      {/if}

      <div class="underlay-alert-dialog-footer">
        <BitsAlertDialog.Action
          class={`underlay-alert-dialog-action underlay-button underlay-button--pill underlay-button--${confirmVariant}`}
          disabled={confirming}
          onclick={handleConfirm}
        >
          {confirming ? "Working…" : confirmLabel}
        </BitsAlertDialog.Action>
        <TextButton
          type="button"
          variant={getCancelTextVariant()}
          class="underlay-alert-dialog-cancel"
          disabled={confirming}
          onclick={handleCancel}
        >
          {cancelLabel}
        </TextButton>
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

  :global(.underlay-alert-dialog-overlay[data-state="open"]) {
    animation:
      underlay-alert-dialog-overlay-in
      var(--underlay-dialog-overlay-in-duration, 160ms)
      var(--underlay-dialog-overlay-in-easing, ease-out);
  }

  :global(.underlay-alert-dialog-overlay[data-state="closed"]) {
    animation:
      underlay-alert-dialog-overlay-out
      var(--underlay-dialog-overlay-out-duration, 120ms)
      var(--underlay-dialog-overlay-out-easing, ease-in);
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
        --underlay-color-dialog-border,
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

  :global(.underlay-alert-dialog-content[data-state="open"]) {
    animation:
      underlay-alert-dialog-content-in
      var(--underlay-dialog-content-in-duration, 190ms)
      var(--underlay-dialog-content-in-easing, cubic-bezier(0.16, 1, 0.3, 1));
  }

  :global(.underlay-alert-dialog-content[data-state="closed"]) {
    animation:
      underlay-alert-dialog-content-out
      var(--underlay-dialog-content-out-duration, 130ms)
      var(--underlay-dialog-content-out-easing, ease-in);
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

  :global(.underlay-alert-dialog-item-card) {
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35));
    border-radius: 0.75rem;
    background: var(--underlay-color-field-bg, rgba(255, 255, 255, 0.03));
    padding: 0.75rem 0.875rem;
  }

  :global(.underlay-alert-dialog-item-card p) {
    margin: 0;
    line-height: 1.45;
  }

  :global(.underlay-alert-dialog-item-card p + p) {
    margin-top: 0.35rem;
  }

  :global(.underlay-alert-dialog-item-label) {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  :global(.underlay-alert-dialog-item-value) {
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 1rem;
    font-weight: 600;
    word-break: break-word;
  }

  :global(.underlay-alert-dialog-item-extra) {
    margin-top: 0.5rem;
  }

  :global(.underlay-alert-dialog-footer) {
    margin-top: 1.75rem;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 0.75rem;
  }

  :global(.underlay-alert-dialog-cancel) {
    margin-left: auto;
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
      margin-top: 1.5rem;
    }
  }

  @keyframes underlay-alert-dialog-overlay-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes underlay-alert-dialog-overlay-out {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }

  @keyframes underlay-alert-dialog-content-in {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(var(--underlay-dialog-content-in-scale, 0.98));
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
  }

  @keyframes underlay-alert-dialog-content-out {
    from {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
    }
    to {
      opacity: 0;
      transform: translate(-50%, -50%) scale(var(--underlay-dialog-content-out-scale, 0.985));
    }
  }

  @media (prefers-reduced-motion: reduce) {
    :global(.underlay-alert-dialog-overlay),
    :global(.underlay-alert-dialog-content) {
      animation: none !important;
    }
  }
</style>
