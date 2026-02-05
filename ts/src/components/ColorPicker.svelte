<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLInputAttributes, "value" | "oninput" | "onchange" | "type" | "size"> {
    /** Current color value (hex format, e.g., "#ff0000") */
    value?: string;
    /** Input ref for programmatic access */
    inputRef?: HTMLInputElement | null;
    /** Called on every input change */
    oninput?: (value: string) => void;
    /** Called when input loses focus or color is selected */
    onchange?: (value: string) => void;
    /** Whether to show the color swatch preview (default: true) */
    showSwatch?: boolean;
    /** Preset colors to display for quick selection */
    presets?: string[];
    /** Size variant */
    size?: "sm" | "md" | "lg";
  }

  let {
    value = $bindable("#000000"),
    inputRef = $bindable(null),
    oninput,
    onchange,
    showSwatch = true,
    presets,
    size = "md",
    class: className,
    id,
    disabled = false,
    ...restProps
  }: Props = $props();

  // Normalize hex color to 6-digit format
  function normalizeHex(hex: string): string {
    let normalized = hex.replace(/^#/, "");
    if (normalized.length === 3) {
      normalized = normalized
        .split("")
        .map((c) => c + c)
        .join("");
    }
    return `#${normalized.toLowerCase()}`;
  }

  // Validate hex color
  function isValidHex(hex: string): boolean {
    return /^#[0-9a-f]{6}$/i.test(hex);
  }

  // Handle text input changes
  function handleTextInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    let inputValue = target.value;

    // Auto-add # if not present
    if (inputValue && !inputValue.startsWith("#")) {
      inputValue = `#${inputValue}`;
    }

    value = inputValue;
    oninput?.(inputValue);
  }

  // Handle text input blur - normalize the color
  function handleTextBlur() {
    if (value && isValidHex(normalizeHex(value))) {
      const normalized = normalizeHex(value);
      value = normalized;
      onchange?.(normalized);
    } else if (value) {
      // Reset to previous valid value or default
      value = "#000000";
      onchange?.("#000000");
    }
  }

  // Handle native color picker change
  function handleColorPickerChange(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    value = target.value;
    oninput?.(target.value);
    onchange?.(target.value);
  }

  // Handle preset click
  function handlePresetClick(color: string) {
    const normalized = normalizeHex(color);
    value = normalized;
    oninput?.(normalized);
    onchange?.(normalized);
  }

  const sizeClass = $derived(`underlay-color-picker--${size}`);
</script>

<div class="underlay-color-picker {sizeClass} {className ?? ''}">
  <div class="underlay-color-picker__input-wrapper">
    {#if showSwatch}
      <label class="underlay-color-picker__swatch-wrapper">
        <input
          type="color"
          class="underlay-color-picker__native"
          value={isValidHex(value) ? value : "#000000"}
          onchange={handleColorPickerChange}
          {disabled}
          aria-label="Color picker"
        />
        <span
          class="underlay-color-picker__swatch"
          style="background-color: {isValidHex(value) ? value : '#000000'}"
        ></span>
      </label>
    {/if}
    <input
      {...restProps}
      {id}
      type="text"
      class="underlay-color-picker__text-input"
      bind:this={inputRef}
      bind:value
      oninput={handleTextInput}
      onblur={handleTextBlur}
      placeholder="#000000"
      maxlength={7}
      {disabled}
    />
  </div>

  {#if presets && presets.length > 0}
    <div class="underlay-color-picker__presets" role="listbox" aria-label="Preset colors">
      {#each presets as preset}
        <button
          type="button"
          class="underlay-color-picker__preset"
          class:underlay-color-picker__preset--selected={normalizeHex(value) === normalizeHex(preset)}
          style="background-color: {preset}"
          onclick={() => handlePresetClick(preset)}
          {disabled}
          aria-label={`Select color ${preset}`}
          aria-selected={normalizeHex(value) === normalizeHex(preset)}
          role="option"
        ></button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .underlay-color-picker {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    width: 100%;
  }

  .underlay-color-picker__input-wrapper {
    display: flex;
    align-items: stretch;
    gap: 0;
    border: var(--underlay-field-border-width, 1px) solid
      var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-sm, 0.35rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
    overflow: hidden;
  }

  .underlay-color-picker__input-wrapper:focus-within {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-color-picker__swatch-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }

  .underlay-color-picker__native {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: pointer;
  }

  .underlay-color-picker__native:disabled {
    cursor: not-allowed;
  }

  .underlay-color-picker__swatch {
    display: block;
    border-right: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    transition: background-color 0.15s ease;
  }

  /* Size variants */
  .underlay-color-picker--sm .underlay-color-picker__swatch {
    width: 2rem;
    height: 2rem;
  }

  .underlay-color-picker--md .underlay-color-picker__swatch {
    width: 2.5rem;
    height: 2.5rem;
  }

  .underlay-color-picker--lg .underlay-color-picker__swatch {
    width: 3rem;
    height: 3rem;
  }

  .underlay-color-picker__text-input {
    flex: 1;
    min-width: 0;
    border: none;
    background: transparent;
    color: var(--underlay-color-text, inherit);
    font-family: var(--underlay-font-mono, monospace);
    font-size: var(--underlay-font-size-md, 0.85rem);
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    text-transform: uppercase;
  }

  .underlay-color-picker--sm .underlay-color-picker__text-input {
    padding: var(--underlay-space-1, 0.25rem) var(--underlay-space-2, 0.5rem);
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .underlay-color-picker--md .underlay-color-picker__text-input {
    padding: var(--underlay-space-2, 0.5rem) var(--underlay-space-3, 0.75rem);
  }

  .underlay-color-picker--lg .underlay-color-picker__text-input {
    padding: var(--underlay-space-3, 0.75rem) var(--underlay-space-4, 1rem);
    font-size: var(--underlay-font-size-lg, 1rem);
  }

  .underlay-color-picker__text-input:focus {
    outline: none;
  }

  .underlay-color-picker__text-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .underlay-color-picker__text-input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
    opacity: 0.7;
  }

  .underlay-color-picker__presets {
    display: flex;
    flex-wrap: wrap;
    gap: var(--underlay-space-1, 0.25rem);
  }

  .underlay-color-picker__preset {
    width: 1.5rem;
    height: 1.5rem;
    border: 2px solid transparent;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    cursor: pointer;
    transition: transform 0.1s ease, border-color 0.15s ease;
  }

  .underlay-color-picker__preset:hover:not(:disabled) {
    transform: scale(1.1);
  }

  .underlay-color-picker__preset--selected {
    border-color: var(--underlay-color-primary, #2563eb);
    box-shadow: 0 0 0 2px var(--underlay-color-surface, #fff);
  }

  .underlay-color-picker__preset:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-color-picker__preset:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
