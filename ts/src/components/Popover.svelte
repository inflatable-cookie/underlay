<script lang="ts">
  import { tick } from "svelte";
  import type { Snippet } from "svelte";
  import { Popover as BitsPopover } from "bits-ui";

  interface Props {
    open?: boolean;
    showTrigger?: boolean;
    triggerLabel?: string;
    triggerAriaLabel?: string;
    triggerType?: "button" | "submit" | "reset";
    contentClassName?: string;
    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    avoidCollisions?: boolean;
    collisionPadding?: number;
    trapFocus?: boolean | undefined;
    preventScroll?: boolean | undefined;
    returnFocusOnClose?: boolean;
    trigger?: Snippet;
    children?: Snippet;
    class?: string;
  }

  let {
    open = $bindable(false),
    showTrigger = true,
    triggerLabel = "Open",
    triggerAriaLabel = "Open popover",
    triggerType = "button",
    contentClassName = "",
    side = "bottom",
    sideOffset = 6,
    align = "center",
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8,
    trapFocus = undefined,
    preventScroll = undefined,
    returnFocusOnClose = true,
    trigger,
    children,
    class: className,
  }: Props = $props();

  let triggerRef: HTMLElement | null = $state(null);
  let lastOpen = $state(open);

  $effect(() => {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });
</script>

<BitsPopover.Root bind:open>
  {#if showTrigger}
    <BitsPopover.Trigger
      bind:ref={triggerRef}
      type={triggerType}
      class={`underlay-popover-trigger ${className ?? ""}`}
      aria-label={triggerAriaLabel}
    >
      {#if trigger}
        {@render trigger()}
      {:else}
        {triggerLabel}
      {/if}
    </BitsPopover.Trigger>
  {/if}

  {#if open}
    <BitsPopover.Portal>
      <BitsPopover.Content
        class={`underlay-popover-content ${contentClassName}`}
        {side}
        {sideOffset}
        {align}
        {alignOffset}
        {avoidCollisions}
        {collisionPadding}
        {trapFocus}
        {preventScroll}
      >
        {@render children?.()}
      </BitsPopover.Content>
    </BitsPopover.Portal>
  {/if}
</BitsPopover.Root>

<style>
  :global(.underlay-popover-trigger) {
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

  :global(.underlay-popover-trigger:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.08))
    );
  }

  :global(.underlay-popover-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, rgba(59, 130, 246, 0.9)));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-popover-content) {
    z-index: 150;
    border-radius: 0.75rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );

    background: var(
      --underlay-color-popover-bg,
      var(--underlay-color-bg-surface, #020617)
    );

    box-shadow: var(
      --underlay-shadow-popover,
      0 18px 48px rgba(0, 0, 0, 0.6)
    );

    padding: 0.75rem;
    max-width: min(32rem, calc(100vw - 2rem));
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }
</style>
