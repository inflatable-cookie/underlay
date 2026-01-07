<script lang="ts">
  import { Dialog as BitsDialog } from "bits-ui";

  export let open = false;

  export let title: string | null = null;
  export let description: string | null = null;

  export let showTrigger = true;
  export let triggerLabel = "Open";

  export let showCloseX = true;

  export let trapFocus: boolean | undefined = undefined;
  export let preventScroll: boolean | undefined = undefined;

</script>

<BitsDialog.Root bind:open>
  {#if showTrigger}
    <BitsDialog.Trigger class="underlay-dialog-trigger">
      <slot name="trigger">{triggerLabel}</slot>
    </BitsDialog.Trigger>
  {/if}

  <BitsDialog.Portal>
    <BitsDialog.Overlay class="underlay-dialog-overlay" />

    <BitsDialog.Content
      class="underlay-dialog-content"
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
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    color: inherit;
    cursor: pointer;
  }

  :global(.underlay-dialog-overlay) {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
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
        var(--froyo-color-border-subtle, rgba(148, 163, 184, 0.5))
      );
    background: var(
      --underlay-color-bg-surface,
      var(--froyo-color-bg-surface, #020617)
    );
    padding: 1rem;

    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.55);
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
    color: var(--underlay-color-text-muted, var(--froyo-color-text-muted, #9ca3af));
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
