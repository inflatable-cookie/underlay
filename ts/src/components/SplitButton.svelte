<script lang="ts">
  import Button from "./Button.svelte";

  type SplitButtonOption = {
    value: string;
    label: string;
  };

  export let options: SplitButtonOption[] = [];
  export let value: string;
  export let type: "button" | "submit" = "button";
  export let variant: "primary" | "secondary" = "primary";

  const menuId = `underlay-split-menu-${Math.random().toString(36).slice(2)}`;

  $: current = options.find((opt) => opt.value === value) ?? options[0] ?? null;

  function choose(next: string) {
    value = next;

    const el = document?.getElementById(menuId) as any;
    if (el && typeof el.hidePopover === "function") {
      el.hidePopover();
    }
  }
</script>

<div class="underlay-split-button">
  <Button
    {type}
    {variant}
    className="underlay-split-button__main"
    popovertarget={menuId}
    popovertargetaction="toggle"
  >
    {#if current}
      {current.label}
    {/if}
  </Button>

  <button
    type="button"
    class="underlay-split-button__toggle"
    popovertarget={menuId}
    popovertargetaction="toggle"
  >
    ▾
  </button>

  <div id={menuId} popover="auto" class="underlay-split-button__menu">
    {#each options as opt}
      <button
        type="button"
        class="underlay-split-button__menu-item"
        on:click={() => choose(opt.value)}
      >
        {opt.label}
      </button>
    {/each}
  </div>
</div>

<style>
  .underlay-split-button {
    --underlay-radius-pill: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    display: inline-flex;
  }

  .underlay-split-button__toggle {
    --underlay-radius-pill: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    border-radius: 0 var(--underlay-radius-pill) var(--underlay-radius-pill) 0;
    margin-left: 0;
    padding: 0 var(--underlay-space-3, var(--underlay-space-3, 0.75rem));
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    cursor: pointer;
    font-size: var(--underlay-font-size-xs, var(--underlay-font-size-xs, 0.75rem));
  }

  .underlay-split-button__toggle:focus-visible,
  :global(.underlay-split-button__main:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .underlay-split-button__menu {
    position: absolute;
    background: var(
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );
    box-shadow: var(--underlay-shadow-popover, var(--underlay-shadow-popover, none));
    padding: var(--underlay-space-1, var(--underlay-space-1, 0.25rem)) 0;
    z-index: 20;
    min-width: var(--underlay-split-button-menu-min-width, var(--underlay-split-button-menu-min-width, 7rem));
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
  }

  @supports (position-anchor: --test) {
    .underlay-split-button {
      anchor-name: --underlay-split-anchor;
    }

    .underlay-split-button__menu {
      position-anchor: --underlay-split-anchor;
      margin: 0;
      position-area: bottom span-all;
      position-try-fallbacks: flip-block;
      width: anchor-size(width);
    }
  }

  :global(.underlay-split-button__main) {
    border-radius: var(--underlay-radius-pill) 0 0 var(--underlay-radius-pill);
    border-right: none;
  }

  .underlay-split-button__menu-item {
    display: block;
    width: 100%;
    padding: var(--underlay-space-1, var(--underlay-space-1, 0.25rem))
      var(--underlay-space-3, var(--underlay-space-3, 0.75rem));
    border: none;
    background: transparent;
    color: inherit;
    font-size: var(--underlay-font-size-sm, var(--underlay-font-size-sm, 0.8rem));
    text-align: left;
    cursor: pointer;
  }

  .underlay-split-button__menu-item:hover {
    background: var(--underlay-color-hover-bg, var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2)));
  }
</style>
