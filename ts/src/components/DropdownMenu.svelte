<script lang="ts">
  import { tick } from "svelte";
  import type { Snippet } from "svelte";
  import { DropdownMenu as BitsDropdownMenu } from "bits-ui";
  import Ellipsis from "lucide-svelte/icons/ellipsis";

  type DropdownMenuItem = {
    label?: string;
    onSelect?: (() => void) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  interface Props {
    open?: boolean;
    showTrigger?: boolean;
    triggerLabel?: string;
    triggerAriaLabel?: string;
    triggerType?: "button" | "submit" | "reset";
    items?: DropdownMenuItem[] | null | undefined;
    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    avoidCollisions?: boolean;
    collisionPadding?: number;
    contentClassName?: string;
    returnFocusOnClose?: boolean;
    trigger?: Snippet;
    children?: Snippet;
    class?: string;
  }

  let {
    open = $bindable(false),
    showTrigger = true,
    triggerLabel,
    triggerAriaLabel = "Open menu",
    triggerType = "button",
    items = undefined,
    side = "bottom",
    sideOffset = 6,
    align = "end",
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8,
    contentClassName = "",
    returnFocusOnClose = true,
    trigger,
    children,
    class: className,
  }: Props = $props();
  let hasTrigger = $derived(typeof trigger === "function");
  let hasChildren = $derived(typeof children === "function");

  let triggerRef: HTMLElement | null = $state(null);
  let lastOpen = $state(open);

  $effect(() => {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });
</script>

<BitsDropdownMenu.Root bind:open>
  {#if showTrigger}
    <BitsDropdownMenu.Trigger
      bind:ref={triggerRef}
      type={triggerType}
      class={`underlay-dropdown-menu-trigger ${className ?? ""}`}
      aria-label={triggerAriaLabel}
    >
      {#if hasTrigger}
        {@render trigger?.()}
      {:else if triggerLabel}
        {triggerLabel}
      {:else}
        <Ellipsis size={16} />
      {/if}
    </BitsDropdownMenu.Trigger>
  {/if}

  <BitsDropdownMenu.Portal>
    <BitsDropdownMenu.Content
      class={`underlay-dropdown-menu-content ${contentClassName}`}
      {side}
      {sideOffset}
      {align}
      {alignOffset}
      {avoidCollisions}
      {collisionPadding}
    >
      {#if items?.length}
        <BitsDropdownMenu.Group aria-label="Actions">
          {#each items as item, index (index)}
            {#if item.separator}
              <BitsDropdownMenu.Separator class="underlay-dropdown-menu-separator" />
            {:else}
              <BitsDropdownMenu.Item
                class={`underlay-dropdown-menu-item ${item.destructive ? "underlay-dropdown-menu-item--destructive" : ""}`}
                disabled={item.disabled}
                textValue={item.label ?? "Item"}
                onSelect={item.onSelect}
              >
                {item.label ?? ""}
              </BitsDropdownMenu.Item>
            {/if}
          {/each}
        </BitsDropdownMenu.Group>
      {:else}
        {#if hasChildren}
          {@render children?.()}
        {/if}
      {/if}
    </BitsDropdownMenu.Content>
  </BitsDropdownMenu.Portal>
</BitsDropdownMenu.Root>

<style>
  :global(.underlay-dropdown-menu-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border-radius: 0.5rem;

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
    font-size: 1.1rem;
    line-height: 1;
  }

  :global(.underlay-dropdown-menu-trigger:hover) {
    background: var(
      --underlay-color-field-bg-hover,
      var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.08))
    );
  }

  :global(.underlay-dropdown-menu-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, rgba(59, 130, 246, 0.9)));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-dropdown-menu-content) {
    z-index: 60;
    border-radius: 0.75rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );

    background: var(
      --underlay-color-menu-bg,
      var(--underlay-color-bg-surface, #020617)
    );

    box-shadow: var(
      --underlay-shadow-menu,
      0 16px 40px rgba(0, 0, 0, 0.55)
    );

    padding: 0.35rem;

    /* Readable menus, but never tiny */
    min-width: 14rem;
    width: max(14rem, var(--bits-dropdown-menu-anchor-width, 0px));
    max-width: min(26rem, calc(100vw - 1.5rem));
  }

  :global(.underlay-dropdown-menu-item) {
    display: flex;
    align-items: center;
    gap: 0.5rem;

    padding: 0.45rem 0.6rem;
    border-radius: 0.5rem;
    cursor: pointer;
    user-select: none;

    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    font-size: 0.9rem;
    line-height: 1.2;
  }

  :global(.underlay-dropdown-menu-item[data-highlighted]) {
    background: rgba(148, 163, 184, 0.16);
  }

  :global(.underlay-dropdown-menu-item[data-disabled]) {
    opacity: 0.55;
    cursor: default;
  }

  :global(.underlay-dropdown-menu-item--destructive) {
    color: rgb(248, 113, 113);
  }

  :global(.underlay-dropdown-menu-item--destructive[data-highlighted]) {
    background: rgba(239, 68, 68, 0.14);
  }

  :global(.underlay-dropdown-menu-separator) {
    height: 1px;
    margin: 0.35rem 0.25rem;
    background: rgba(148, 163, 184, 0.22);
  }
</style>
