<script lang="ts">
  import { tick } from "svelte";
  import { Select as BitsSelect } from "bits-ui";
  import type { Snippet } from "svelte";

  type SelectItem = {
    value: string;
    label: string;
    disabled?: boolean;
  };

  interface Props {
    value?: string;
    open?: boolean;
    items?: SelectItem[] | null;
    placeholder?: string;
    name?: string;
    required?: boolean;
    disabled?: boolean;
    triggerType?: "button" | "submit" | "reset";
    triggerAriaLabel?: string | null;
    contentClassName?: string;
    returnFocusOnClose?: boolean;
    side?: "top" | "right" | "bottom" | "left";
    sideOffset?: number;
    align?: "start" | "center" | "end";
    alignOffset?: number;
    avoidCollisions?: boolean;
    collisionPadding?: number;
    onchange?: (value: string) => void;
    oninput?: (value: string) => void;
    children?: Snippet;
    class?: string;
  }

  let {
    value = $bindable(""),
    open = $bindable(false),
    items = undefined,
    placeholder = "Select…",
    name = undefined,
    required = false,
    disabled = false,
    triggerType = "button",
    triggerAriaLabel = null,
    contentClassName = "",
    returnFocusOnClose = true,
    side = "bottom",
    sideOffset = 6,
    align = "end",
    alignOffset = 0,
    avoidCollisions = true,
    collisionPadding = 8,
    onchange,
    oninput,
    children,
    class: className,
  }: Props = $props();

  let triggerRef: HTMLElement | null = $state(null);
  let lastOpen = $state(open);
  let lastDispatchedValue = $state(value);

  $effect(() => {
    if (lastOpen && !open && returnFocusOnClose && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });

  // BitsSelect doesn't emit native "change" events in a way we can forward
  // without opting into internal types, so we dispatch when bound value changes.
  $effect(() => {
    if (items?.length && value !== lastDispatchedValue) {
      lastDispatchedValue = value;
      onchange?.(value);
      oninput?.(value);
    }
  });

  let selectedLabel = $derived((items ?? []).find((item) => item.value === value)?.label);
  let hasSelection = $derived(typeof selectedLabel === "string" && selectedLabel.length > 0);

  let alignShift = $derived(
    align === "end"
      ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
      : align === "start"
        ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
        : "0px"
  );

  let alignShiftRtl = $derived(
    align === "end"
      ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
      : align === "start"
        ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
        : "0px"
  );

  function handleNativeChange(event: Event) {
    const target = event.currentTarget as HTMLSelectElement | null;
    const next = target ? target.value : value;
    value = next;
    onchange?.(next);
    oninput?.(next);
  }
</script>

{#if items?.length}
  <BitsSelect.Root
    type="single"
    items={items}
    bind:value={value as never}
    bind:open
    {name}
    {required}
    {disabled}
  >
    <BitsSelect.Trigger
      bind:ref={triggerRef}
      type={triggerType}
      class={`underlay-select-trigger ${className ?? ""}`}
      aria-label={triggerAriaLabel ?? placeholder}
    >
      <span class:placeholder={!hasSelection}>
        {hasSelection ? selectedLabel : placeholder}
      </span>
      <span class="underlay-select-trigger__chevron" aria-hidden="true">▾</span>
    </BitsSelect.Trigger>

    <BitsSelect.Portal>
      <BitsSelect.Content
        class={`underlay-select-content ${contentClassName}`}
        style={`--underlay-select-align-shift: ${alignShift}; --underlay-select-align-shift-rtl: ${alignShiftRtl};`}
        {side}
        {sideOffset}
        {align}
        {alignOffset}
        {avoidCollisions}
        {collisionPadding}
      >
        <BitsSelect.Viewport class="underlay-select-viewport">
          {#each items as item (item.value)}
            {@const isSelected = item.value === value}
            <BitsSelect.Item
              value={item.value}
              label={item.label}
              disabled={item.disabled}
              class="underlay-select-item"
            >
              <span class="underlay-select-item__label">{item.label}</span>
              {#if isSelected}
                <span class="underlay-select-item__check" aria-hidden="true">✓</span>
              {/if}
            </BitsSelect.Item>
          {/each}
        </BitsSelect.Viewport>
      </BitsSelect.Content>
    </BitsSelect.Portal>
  </BitsSelect.Root>
{:else}
  <select
    class={`underlay-select ${className ?? ""}`}
    bind:value
    {name}
    {required}
    {disabled}
    onchange={handleNativeChange}
  >
    {#if children}
      {@render children()}
    {/if}
  </select>
{/if}

<style>
  .underlay-select {
    width: 100%;
    min-width: min(var(--underlay-select-min-width, 12rem), 100%);
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: 0.35rem;
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    font-size: 0.85rem;
  }

  .underlay-select:focus,
  .underlay-select:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-select-trigger) {
    width: 100%;
    min-width: min(var(--underlay-select-min-width, 12rem), 100%);
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: 0.35rem;
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    font-size: 0.85rem;
    cursor: pointer;
  }

  :global(.underlay-select-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .placeholder {
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
  }

  :global(.underlay-select-trigger__chevron) {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  :global(.underlay-select-content) {
    z-index: 50;
    border-radius: 0.35rem;
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
      0 8px 16px rgba(0, 0, 0, 0.5)
    );

    padding: 0.25rem;

    min-width: min(var(--underlay-select-menu-min-width, 12rem), calc(100vw - 1.5rem));
    width: auto;
    max-width: min(26rem, calc(100vw - 1.5rem));
  }

  :global(.underlay-select-content[data-align="center"]) {
    transform: translateX(var(--underlay-select-align-shift, 0px));
  }

  :global([dir="rtl"] .underlay-select-content[data-align="center"]) {
    transform: translateX(var(--underlay-select-align-shift-rtl, 0px));
  }

  :global(.underlay-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    max-height: min(18rem, var(--bits-select-content-available-height));
    overflow: auto;
  }

  :global(.underlay-select-item) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.35rem 0.55rem;
    border-radius: 0.25rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.85rem;
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
  }

  :global(.underlay-select-item[data-highlighted]) {
    background: var(
      --underlay-color-hover-bg,
      var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2))
    );
  }

  :global(.underlay-select-item[data-disabled]) {
    opacity: 0.6;
    cursor: default;
  }

  :global(.underlay-select-item__check) {
    opacity: 0.9;
  }
</style>
