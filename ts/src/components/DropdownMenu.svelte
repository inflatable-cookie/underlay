<script lang="ts">
  import { DropdownMenu as BitsDropdownMenu } from "bits-ui";

  type DropdownMenuItem = {
    label?: string;
    onSelect?: (() => void) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  export let open = false;

  export let showTrigger = true;
  export let triggerLabel = "⋯";

  export let items: DropdownMenuItem[] | null | undefined = undefined;

  export let sideOffset = 6;
  export let align: "start" | "center" | "end" = "end";
  export let side: "top" | "right" | "bottom" | "left" = "bottom";

</script>

<BitsDropdownMenu.Root bind:open>
  {#if showTrigger}
    <BitsDropdownMenu.Trigger
      {...$$restProps}
      class={`underlay-dropdown-menu-trigger ${$$restProps.class ?? ""}`}
      aria-label="Open menu"
    >
      <slot name="trigger">{triggerLabel}</slot>
    </BitsDropdownMenu.Trigger>
  {/if}

  <BitsDropdownMenu.Portal>
    <BitsDropdownMenu.Content
      class="underlay-dropdown-menu-content"
      {sideOffset}
      {align}
      {side}
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
        <slot />
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
    border: 1px solid rgba(148, 163, 184, 0.35);
    background: rgba(255, 255, 255, 0.03);
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    cursor: pointer;
    font-size: 1.1rem;
    line-height: 1;
  }

  :global(.underlay-dropdown-menu-trigger:hover) {
    background: rgba(148, 163, 184, 0.08);
  }

  :global(.underlay-dropdown-menu-trigger:focus-visible) {
    outline: 2px solid rgba(59, 130, 246, 0.9);
    outline-offset: 2px;
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
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.55);
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
