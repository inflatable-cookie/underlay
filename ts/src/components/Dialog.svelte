<script lang="ts">
  import { tick } from "svelte";
  import { Dialog as BitsDialog } from "bits-ui";

  export let open = false;

  export let title: string | null = null;
  export let description: string | null = null;

  export let showTrigger = true;
  export let triggerLabel = "Open";
  export let triggerAriaLabel: string | null = null;
  export let triggerType: "button" | "submit" | "reset" = "button";

  export let showCloseX = true;

  export let contentClassName = "";
  export let overlayClassName = "";

  export let trapFocus: boolean | undefined = undefined;
  export let preventScroll: boolean | undefined = undefined;

  export let returnFocusOnClose = true;

  let triggerRef: HTMLElement | null = null;
  let lastOpen = open;

  $: {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  }

</script>

<BitsDialog.Root bind:open>
  {#if showTrigger}
    <BitsDialog.Trigger
      bind:ref={triggerRef}
      class="underlay-dialog-trigger"
      type={triggerType}
      aria-label={triggerAriaLabel ?? undefined}
    >
      <slot name="trigger">{triggerLabel}</slot>
    </BitsDialog.Trigger>
  {/if}

  <BitsDialog.Portal>
    <BitsDialog.Overlay
      class={`underlay-dialog-overlay ${overlayClassName}`}
    />

    <BitsDialog.Content
      class={`underlay-dialog-content ${contentClassName}`}
      {trapFocus}
      {preventScroll}
    >
      {#if showCloseX}
        <BitsDialog.Close class="underlay-dialog-close-x" aria-label="Close">
          <span aria-hidden="true">×</span>
        </BitsDialog.Close>
      {/if}

      {#if title}
        <BitsDialog.Title class="underlay-dialog-title">{title}</BitsDialog.Title>
      {/if}

      {#if description}
        <BitsDialog.Description class="underlay-dialog-description">
          {description}
        </BitsDialog.Description>
      {/if}

      <div class="underlay-dialog-body">
        <slot />
      </div>

      <div class="underlay-dialog-footer">
        <slot name="footer" />
      </div>
    </BitsDialog.Content>
  </BitsDialog.Portal>
</BitsDialog.Root>

<style>
  :global(.underlay-dialog-trigger) {
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

  :global(.underlay-dialog-trigger:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.08))
    );
  }

  :global(.underlay-dialog-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-dialog-overlay) {
    position: fixed;
    inset: 0;
    background: var(--underlay-color-overlay-backdrop, rgba(0, 0, 0, 0.65));
    z-index: 50;
  }

  :global(.underlay-dialog-content) {
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

  :global(.underlay-dialog-close-x) {
    position: absolute;
    top: 0.65rem;
    right: 0.65rem;
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    color: inherit;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    font-size: 1.25rem;
    line-height: 1;
  }

  :global(.underlay-dialog-title) {
    margin: 0;
    font-size: 1.05rem;
    font-weight: 650;
  }

  :global(.underlay-dialog-description) {
    margin-top: 0.25rem;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    font-size: 0.9rem;
  }

  :global(.underlay-dialog-body) {
    margin-top: 0.9rem;
  }

  :global(.underlay-dialog-footer) {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
  }
</style>
