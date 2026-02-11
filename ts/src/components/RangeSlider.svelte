<script lang="ts">
  import { getContext, onMount, untrack } from "svelte";
  import { createStableId } from "../patterns/dom";
  import type { FormValidationContext } from "./text-input/form-validation";

  export interface RangeSliderOption {
    value: string;
    label?: string;
    tone?: "default" | "primary" | "success" | "warning" | "danger";
    color?: string;
  }

  interface Props {
    options?: Array<string | RangeSliderOption>;
    value?: string;
    id?: string;
    name?: string;
    required?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    showValue?: boolean;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
    class?: string;
  }

  let {
    options = [],
    value = $bindable(""),
    id = undefined,
    name = undefined,
    required = false,
    disabled = false,
    ariaLabel = undefined,
    showValue = true,
    oninput,
    onchange,
    class: className
  }: Props = $props();

  const formValidation = getContext<FormValidationContext | undefined>("formValidation");
  const fieldId = untrack(() => id) ?? createStableId("underlay-range-slider");
  const isRequired = $derived(required ?? false);

  const normalizedOptions = $derived.by<RangeSliderOption[]>(() => {
    const items: RangeSliderOption[] = [];

    for (const option of options) {
      if (typeof option === "string") {
        const trimmed = option.trim();
        if (!trimmed) continue;
        items.push({ value: trimmed, label: trimmed });
        continue;
      }

      const trimmedValue = option.value.trim();
      if (!trimmedValue) continue;

      const trimmedLabel = option.label?.trim();
      const trimmedColor = option.color?.trim();
      items.push({
        value: trimmedValue,
        label: trimmedLabel && trimmedLabel.length > 0 ? trimmedLabel : trimmedValue,
        tone: option.tone,
        color: trimmedColor && trimmedColor.length > 0 ? trimmedColor : undefined
      });
    }

    return items;
  });

  const selectedIndex = $derived.by(() => normalizedOptions.findIndex((option) => option.value === value));
  const selectedOption = $derived.by(() => {
    if (selectedIndex < 0) return null;
    return normalizedOptions[selectedIndex] ?? null;
  });

  const selectedAccentColor = $derived.by(() => {
    const option = selectedOption;
    if (!option) {
      return "var(--underlay-color-primary, #2563eb)";
    }

    if (typeof option.color === "string" && option.color.trim().length > 0) {
      return option.color.trim();
    }

    switch (option.tone) {
      case "success":
        return "var(--underlay-color-success, #22c55e)";
      case "warning":
        return "var(--underlay-color-warning, #de8d04)";
      case "danger":
        return "var(--underlay-color-danger, #ef4444)";
      case "default":
        return "var(--underlay-color-text-muted, #9ca3af)";
      case "primary":
      default:
        return "var(--underlay-color-primary, #2563eb)";
    }
  });

  const selectedAccentHalo = $derived.by(() => {
    const option = selectedOption;
    switch (option?.tone) {
      case "success":
        return "rgba(34, 197, 94, 0.2)";
      case "warning":
        return "rgba(222, 141, 4, 0.2)";
      case "danger":
        return "rgba(239, 68, 68, 0.2)";
      case "default":
        return "rgba(148, 163, 184, 0.2)";
      default:
        return "rgba(37, 99, 235, 0.2)";
    }
  });

  const maxIndex = $derived(Math.max(normalizedOptions.length - 1, 0));
  const progressPercent = $derived.by(() => {
    if (maxIndex <= 0) return 0;
    const index = Math.max(selectedIndex, 0);
    return (index / maxIndex) * 100;
  });

  let prevValue = $state("");

  function registerField() {
    if (!formValidation) return;

    formValidation.registerField(
      fieldId,
      isRequired,
      selectedOption !== null,
      "valid",
      true
    );
  }

  function updateField() {
    if (!formValidation) return;

    formValidation.updateField(fieldId, selectedOption !== null, "valid", true);
  }

  onMount(() => {
    if (!formValidation) return;

    registerField();
    prevValue = untrack(() => value);

    return () => {
      formValidation.unregisterField(fieldId);
    };
  });

  $effect(() => {
    if (!formValidation || value === prevValue) return;
    updateField();
    prevValue = value;
  });

  $effect(() => {
    if (normalizedOptions.length === 0) {
      if (value !== "") {
        value = "";
      }
      return;
    }

    if (selectedOption === null) {
      value = normalizedOptions[0].value;
    }
  });

  function setFromIndex(index: number, source: "input" | "change") {
    if (normalizedOptions.length === 0) return;

    const clampedIndex = Math.max(0, Math.min(index, maxIndex));
    const nextOption = normalizedOptions[clampedIndex];
    if (!nextOption) return;

    const nextValue = nextOption.value;
    if (nextValue === value) return;

    value = nextValue;

    if (source === "input") {
      oninput?.(nextValue);
      return;
    }

    onchange?.(nextValue);
  }

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement | null;
    const nextIndex = target ? Number(target.value) : 0;
    setFromIndex(Number.isFinite(nextIndex) ? nextIndex : 0, "input");
  }

  function handleChange(event: Event) {
    const target = event.currentTarget as HTMLInputElement | null;
    const nextIndex = target ? Number(target.value) : 0;
    setFromIndex(Number.isFinite(nextIndex) ? nextIndex : 0, "change");
  }
</script>

<div
  class={`underlay-range-slider ${className ?? ""}`}
  style={`--underlay-range-slider-progress: ${progressPercent}%; --underlay-range-slider-accent: ${selectedAccentColor}; --underlay-range-slider-halo: ${selectedAccentHalo};`}
>
  <div class="underlay-range-slider__row">
    <input
      id={id}
      type="range"
      class="underlay-range-slider__input"
      min="0"
      max={maxIndex}
      step="1"
      value={Math.max(selectedIndex, 0)}
      disabled={disabled || normalizedOptions.length <= 1}
      aria-label={ariaLabel ?? name ?? "Range slider"}
      aria-valuemin={0}
      aria-valuemax={maxIndex}
      aria-valuenow={Math.max(selectedIndex, 0)}
      aria-valuetext={selectedOption?.label ?? ""}
      oninput={handleInput}
      onchange={handleChange}
    />

    {#if showValue}
      <div class="underlay-range-slider__value" aria-live="polite">
        {selectedOption?.label ?? ""}
      </div>
    {/if}
  </div>

  {#if name}
    <input type="hidden" {name} value={selectedOption?.value ?? ""} />
  {/if}
</div>

<style>
  @property --underlay-range-slider-progress {
    syntax: "<percentage>";
    inherits: false;
    initial-value: 0%;
  }

  .underlay-range-slider {
    display: block;
    width: 100%;
    transition: --underlay-range-slider-progress 180ms ease;
  }

  .underlay-range-slider__row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-range-slider__value {
    width: fit-content;
    min-width: 4.25em;
    padding: 0.15rem 0.5rem;
    border-radius: var(--underlay-radius-pill, 999px);
    border: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    background: var(--underlay-color-surface-elevated, rgba(30, 41, 59, 0.35));
    color: var(--underlay-range-slider-accent, var(--underlay-color-text, #e5e7eb));
    font-size: var(--underlay-font-size-xs, 0.72rem);
    font-weight: 500;
    letter-spacing: 0.01em;
    text-align: center;
    white-space: nowrap;
    transition: color 160ms ease, border-color 160ms ease, background-color 160ms ease;
  }

  .underlay-range-slider__input {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 0.5rem;
    border-radius: var(--underlay-radius-pill, 999px);
    outline: none;
    background:
      linear-gradient(
        to right,
        var(--underlay-range-slider-accent, var(--underlay-color-primary, #2563eb)) 0%,
        var(--underlay-range-slider-accent, var(--underlay-color-primary, #2563eb)) var(--underlay-range-slider-progress, 50%),
        var(--underlay-color-field-bg, rgba(148, 163, 184, 0.24)) var(--underlay-range-slider-progress, 50%),
        var(--underlay-color-field-bg, rgba(148, 163, 184, 0.24)) 100%
      );
    cursor: pointer;
    transition: background 180ms ease, opacity 120ms ease;
  }

  .underlay-range-slider__input:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-range-slider-accent, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-range-slider__input::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--underlay-range-slider-accent, var(--underlay-color-primary, #2563eb));
    border-radius: var(--underlay-radius-pill, 999px);
    background: var(--underlay-color-surface, #0f172a);
    box-shadow: 0 0 0 2px var(--underlay-range-slider-halo, rgba(37, 99, 235, 0.2));
    transition:
      transform 0.15s ease,
      border-color 0.18s ease,
      box-shadow 0.18s ease,
      background-color 0.18s ease;
  }

  .underlay-range-slider__input:hover::-webkit-slider-thumb {
    transform: scale(1.05);
  }

  .underlay-range-slider__input::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--underlay-range-slider-accent, var(--underlay-color-primary, #2563eb));
    border-radius: var(--underlay-radius-pill, 999px);
    background: var(--underlay-color-surface, #0f172a);
    box-shadow: 0 0 0 2px var(--underlay-range-slider-halo, rgba(37, 99, 235, 0.2));
    transition:
      transform 0.15s ease,
      border-color 0.18s ease,
      box-shadow 0.18s ease,
      background-color 0.18s ease;
  }

  .underlay-range-slider__input:hover::-moz-range-thumb {
    transform: scale(1.05);
  }

  .underlay-range-slider__input::-moz-range-track {
    height: 0.5rem;
    border-radius: var(--underlay-radius-pill, 999px);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.24));
  }

  .underlay-range-slider__input:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .underlay-range-slider__input:disabled::-webkit-slider-thumb,
  .underlay-range-slider__input:disabled::-moz-range-thumb {
    cursor: not-allowed;
  }
</style>
