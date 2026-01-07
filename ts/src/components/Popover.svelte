<script lang="ts">
  import { Popover as BitsPopover } from "bits-ui";

  export let open = false;

  export let showTrigger = true;
  export let triggerLabel = "Open";

  export let side: "top" | "right" | "bottom" | "left" = "bottom";
  export let sideOffset = 6;
  export let align: "start" | "center" | "end" = "center";
  export let alignOffset = 0;
  export let avoidCollisions = true;
  export let collisionPadding = 8;

  export let trapFocus: boolean | undefined = undefined;
  export let preventScroll: boolean | undefined = undefined;
</script>

<BitsPopover.Root bind:open>
  {#if showTrigger}
    <BitsPopover.Trigger
      {...$$restProps}
      class={`underlay-popover-trigger ${$$restProps.class ?? ""}`}
      aria-label="Open popover"
    >
      <slot name="trigger">{triggerLabel}</slot>
    </BitsPopover.Trigger>
  {/if}

  <BitsPopover.Portal>
    <BitsPopover.Content
      class="underlay-popover-content"
      {side}
      {sideOffset}
      {align}
      {alignOffset}
      {avoidCollisions}
      {collisionPadding}
      {trapFocus}
      {preventScroll}
    >
      <slot />
    </BitsPopover.Content>
  </BitsPopover.Portal>
</BitsPopover.Root>

<style>
  :global(.underlay-popover-trigger) {
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

  :global(.underlay-popover-trigger:hover) {
    background: rgba(148, 163, 184, 0.08);
  }

  :global(.underlay-popover-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
  }

  :global(.underlay-popover-content) {
    z-index: 70;
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
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.55);
    padding: 0.75rem;
    max-width: min(32rem, calc(100vw - 2rem));
    color: var(--underlay-color-text, var(--froyo-color-text, #e5e7eb));
  }
</style>
