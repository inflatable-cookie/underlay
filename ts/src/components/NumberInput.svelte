<script lang="ts">
  import type { ValidationResult } from "./TextInput.svelte";
  import TextInput from "./TextInput.svelte";
  import ChevronUp from "lucide-svelte/icons/chevron-up";
  import ChevronDown from "lucide-svelte/icons/chevron-down";

  interface Props {
    /** Current value (bindable) */
    value?: string;
    /** Input name for form submission */
    name?: string;
    /** Input ID */
    id?: string;
    /** Minimum allowed value */
    min?: number;
    /** Maximum allowed value */
    max?: number;
    /** Step increment (default: 1) */
    step?: number;
    /** Whether field is required */
    required?: boolean;
    /** Whether field is disabled */
    disabled?: boolean;
    /** Static prefix to display before the input (e.g., "A") */
    prefix?: string;
    /** Placeholder text */
    placeholder?: string;
    /** Async validation function */
    validate?: (value: string, context?: unknown) => Promise<ValidationResult>;
    /** Context to pass to validation function */
    validationContext?: unknown;
    /** Debounce delay for validation in ms (default: 300) */
    validationDebounce?: number;
    /** Callback when validation state changes */
    onvalidationchange?: (status: string, isValid: boolean) => void;
    /** Additional CSS class */
    class?: string;
  }

  let {
    value = $bindable(""),
    name,
    id,
    min,
    max,
    step = 1,
    required = false,
    disabled = false,
    prefix,
    placeholder,
    validate,
    validationContext,
    validationDebounce = 300,
    onvalidationchange,
    class: className,
  }: Props = $props();

  let inputRef = $state<HTMLInputElement | null>(null);

  function parseValue(): number {
    const num = parseInt(value, 10);
    return isNaN(num) ? (min ?? 0) : num;
  }

  function clampValue(num: number): number {
    if (min !== undefined && num < min) return min;
    if (max !== undefined && num > max) return max;
    return num;
  }

  function increment() {
    if (disabled) return;
    const current = parseValue();
    const next = clampValue(current + step);
    value = String(next);
    inputRef?.focus();
  }

  function decrement() {
    if (disabled) return;
    const current = parseValue();
    const next = clampValue(current - step);
    value = String(next);
    inputRef?.focus();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "ArrowUp") {
      event.preventDefault();
      increment();
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      decrement();
    }
  }

  function handleInput(newValue: string) {
    // Only allow digits and optional leading minus
    const sanitized = newValue.replace(/[^0-9-]/g, "").replace(/(?!^)-/g, "");
    if (sanitized !== newValue) {
      value = sanitized;
    } else {
      value = newValue;
    }
  }

  function handleBlur() {
    // Clamp value on blur if it's a valid number
    if (value) {
      const num = parseInt(value, 10);
      if (!isNaN(num)) {
        const clamped = clampValue(num);
        if (clamped !== num) {
          value = String(clamped);
        }
      }
    }
  }

  // Determine if decrement/increment are at limits
  const atMin = $derived(min !== undefined && parseValue() <= min);
  const atMax = $derived(max !== undefined && parseValue() >= max);
</script>

<div class="underlay-number-input {className ?? ''}" class:underlay-number-input--disabled={disabled}>
  <TextInput
    {id}
    {name}
    {required}
    {disabled}
    {prefix}
    {placeholder}
    {validate}
    {validationContext}
    {validationDebounce}
    {onvalidationchange}
    bind:value
    bind:inputRef
    oninput={handleInput}
    onblur={handleBlur}
    onkeydown={handleKeydown}
    inputmode="numeric"
    pattern="[0-9]*"
    autocomplete="off"
    class="underlay-number-input__input"
  >
    {#snippet suffix()}
      <div class="underlay-number-input__controls">
        <button
          type="button"
          class="underlay-number-input__button underlay-number-input__button--up"
          onclick={increment}
          disabled={disabled || atMax}
          tabindex={-1}
          aria-label="Increment"
        >
          <ChevronUp size="1em" strokeWidth={2.5} />
        </button>
        <button
          type="button"
          class="underlay-number-input__button underlay-number-input__button--down"
          onclick={decrement}
          disabled={disabled || atMin}
          tabindex={-1}
          aria-label="Decrement"
        >
          <ChevronDown size="1em" strokeWidth={2.5} />
        </button>
      </div>
    {/snippet}
  </TextInput>
</div>

<style>
  .underlay-number-input {
    width: 100%;
    /* Tell TextInput how wide the suffix is so validation icon can be positioned */
    --underlay-input-suffix-width: 1.2em;
  }

  /* Adjust input padding to make room for controls + validation icon */
  .underlay-number-input :global(.underlay-input) {
    padding-right: calc(var(--underlay-input-suffix-width) + 2em) !important;
  }

  .underlay-number-input__controls {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    width: 1.2em;
    border-radius: 0 var(--underlay-radius-sm, 0.35rem) var(--underlay-radius-sm, 0.35rem) 0;
    overflow: hidden;
  }

  .underlay-number-input__button {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    transition: background-color 0.15s ease, color 0.15s ease;
    font-size: 0.85em;
  }

  .underlay-number-input__button:hover:not(:disabled) {
    background: var(--underlay-color-primary-subtle, rgba(37, 99, 235, 0.15));
    color: var(--underlay-color-primary, #2563eb);
  }

  .underlay-number-input__button:active:not(:disabled) {
    background: var(--underlay-color-primary-subtle, rgba(37, 99, 235, 0.25));
  }

  .underlay-number-input__button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .underlay-number-input__button--up {
    border-bottom: none;
  }

  .underlay-number-input--disabled .underlay-number-input__controls {
    opacity: 0.5;
    pointer-events: none;
  }
</style>
