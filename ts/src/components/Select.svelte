<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { Select as BitsSelect } from "bits-ui";

  type SelectItem = {
    value: string;
    label: string;
    disabled?: boolean;
  };

  export let value: string = "";
  export let items: SelectItem[] | null | undefined = undefined;
  export let placeholder: string = "Select…";

  // Used by BitsSelect mode for hidden input support.
  export let name: string | undefined = undefined;
  export let required: boolean = false;
  export let disabled: boolean = false;

  export let side: "top" | "right" | "bottom" | "left" = "bottom";
  export let sideOffset = 6;
  export let align: "start" | "center" | "end" = "end";
  export let alignOffset = 0;
  export let avoidCollisions = true;
  export let collisionPadding = 8;

  const dispatch = createEventDispatcher<{ change: string; input: string }>();

  function handleNativeChange(event: Event) {
    const target = event.currentTarget as HTMLSelectElement | null;
    const next = target ? target.value : value;
    value = next;
    dispatch("change", next);
    dispatch("input", next);
  }

  // BitsSelect doesn't emit native "change" events in a way we can forward
  // without opting into internal types, so we dispatch when bound value changes.
  let lastDispatchedValue = value;
  $: if (items?.length && value !== lastDispatchedValue) {
    lastDispatchedValue = value;
    dispatch("change", value);
    dispatch("input", value);
  }

  $: selectedLabel = (items ?? []).find((item) => item.value === value)?.label;
  $: hasSelection = typeof selectedLabel === "string" && selectedLabel.length > 0;

  $: alignShift =
    align === "end"
      ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
      : align === "start"
        ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
        : "0px";

  $: alignShiftRtl =
    align === "end"
      ? "calc((100% - var(--bits-floating-anchor-width)) / 2)"
      : align === "start"
        ? "calc((var(--bits-floating-anchor-width) - 100%) / 2)"
        : "0px";

</script>

{#if items?.length}
  <BitsSelect.Root
    type="single"
    items={items}
    bind:value={value as never}
    {name}
    {required}
    {disabled}
  >
    <BitsSelect.Trigger
      {...$$restProps}
      class={`underlay-select-trigger ${$$restProps.class ?? ""}`}
      aria-label={placeholder}
    >
      <span class:placeholder={!hasSelection}>
        {hasSelection ? selectedLabel : placeholder}
      </span>
      <span class="underlay-select-trigger__chevron" aria-hidden="true">▾</span>
    </BitsSelect.Trigger>

    <BitsSelect.Portal>
      <BitsSelect.Content
        class="underlay-select-content"
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
    {...$$restProps}
    class={`underlay-select ${$$restProps.class ?? ""}`}
    bind:value
    {name}
    {required}
    {disabled}
    onchange={handleNativeChange}
  >
    <slot />
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
    outline: 2px solid var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: 2px;
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
    outline: 2px solid var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: 2px;
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
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.5);
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
    background: rgba(148, 163, 184, 0.2);
  }

  :global(.underlay-select-item[data-disabled]) {
    opacity: 0.6;
    cursor: default;
  }

  :global(.underlay-select-item__check) {
    opacity: 0.9;
  }
</style>
