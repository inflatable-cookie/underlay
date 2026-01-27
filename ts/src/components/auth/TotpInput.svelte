<script lang="ts">
  /**
   * TOTP/OTP code input with single input + visual slots architecture.
   *
   * Uses a single invisible input for password manager compatibility,
   * with visual digit slots rendered on top for the split-digit UX.
   *
   * Features:
   * - Password manager autofill support (1Password, Bitwarden, etc.)
   * - Paste support for full codes
   * - Mobile SMS autofill via autocomplete="one-time-code"
   * - Accessible - single input for screen readers
   */

  import Field from "../Field.svelte";

  interface Props {
    /** The complete code value (bindable) */
    value?: string;
    /** Input name for form submission */
    name?: string;
    /** Field label */
    label?: string;
    /** Hint text below the input */
    hint?: string;
    /** Error message */
    error?: string;
    /** Number of digits (default: 6) */
    length?: number;
    /** Whether the input is disabled */
    disabled?: boolean;
    /** Callback when value changes */
    oninput?: (value: string) => void;
    /** Callback when value changes (on blur or complete) */
    onchange?: (value: string) => void;
    /** Callback when all digits are filled */
    oncomplete?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    name = "code",
    label = "Authenticator code",
    hint,
    error,
    length = 6,
    disabled = false,
    oninput,
    onchange,
    oncomplete
  }: Props = $props();

  let inputRef = $state<HTMLInputElement | null>(null);

  // Derive digits array from value for display
  let digits = $derived(() => {
    const chars = value.replace(/\D/g, "").slice(0, length).split("");
    return Array(length).fill("").map((_, i) => chars[i] || "");
  });

  // Track which slot appears "focused" based on caret position
  let caretIndex = $state(0);
  let hasFocus = $state(false);

  function handleInput(event: Event) {
    const input = event.target as HTMLInputElement;
    // Strip non-digits and limit to length
    const newValue = input.value.replace(/\D/g, "").slice(0, length);

    if (newValue !== value) {
      value = newValue;
      oninput?.(newValue);

      if (newValue.length === length) {
        onchange?.(newValue);
        oncomplete?.(newValue);
      }
    }

    // Update caret position
    caretIndex = Math.min(newValue.length, length - 1);
  }

  function handleKeydown(event: KeyboardEvent) {
    // Update caret index based on input selection
    requestAnimationFrame(() => {
      if (inputRef) {
        const pos = inputRef.selectionStart ?? value.length;
        caretIndex = Math.min(pos, length - 1);
      }
    });
  }

  function handleFocus() {
    hasFocus = true;
    caretIndex = Math.min(value.length, length - 1);
  }

  function handleBlur() {
    hasFocus = false;
    // Trigger onchange on blur if we have a partial value
    if (value && value.length < length) {
      onchange?.(value);
    }
  }

  function handleSlotClick(index: number) {
    inputRef?.focus();
    // Try to position caret at clicked slot
    if (inputRef) {
      const pos = Math.min(index, value.length);
      inputRef.setSelectionRange(pos, pos);
      caretIndex = Math.min(pos, length - 1);
    }
  }
</script>

<Field {label} {hint} {error}>
  <!-- Hidden input for form submission -->
  <input type="hidden" {name} {value} />

  <div
    class="underlay-totp-input"
    class:underlay-totp-input--disabled={disabled}
    class:underlay-totp-input--error={!!error}
    class:underlay-totp-input--focused={hasFocus}
    role="group"
    aria-label={label}
  >
    <!-- Single real input - invisible but functional -->
    <input
      bind:this={inputRef}
      type="text"
      inputmode="numeric"
      pattern="[0-9]*"
      autocomplete="one-time-code"
      maxlength={length}
      {disabled}
      {value}
      oninput={handleInput}
      onkeydown={handleKeydown}
      onkeyup={handleKeydown}
      onfocus={handleFocus}
      onblur={handleBlur}
      class="underlay-totp-input__input"
      aria-label={label}
      aria-invalid={error ? "true" : undefined}
    />

    <!-- Visual digit slots -->
    {#each digits() as digit, index}
      <button
        type="button"
        class="underlay-totp-input__slot"
        class:underlay-totp-input__slot--active={hasFocus && index === caretIndex}
        class:underlay-totp-input__slot--filled={!!digit}
        tabindex={-1}
        onclick={() => handleSlotClick(index)}
        aria-hidden="true"
      >
        {digit}
      </button>
    {/each}
  </div>
</Field>

<style>
  .underlay-totp-input {
    position: relative;
    display: inline-flex;
    gap: 0.5rem;
    width: max-content;
  }

  /* Single invisible input that receives all interaction */
  .underlay-totp-input__input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: text;
    /* Ensure it's above slots for interaction but invisible */
    z-index: 1;
  }

  /* Visual digit slots */
  .underlay-totp-input__slot {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.75rem;
    height: 3.25rem;
    padding: 0;
    border: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.4));
    border-radius: 0.5rem;
    background: var(--underlay-color-input-bg, rgba(15, 23, 42, 0.6));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 1.5rem;
    font-weight: 600;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    text-align: center;
    cursor: text;
    transition: border-color 0.15s, box-shadow 0.15s;
    /* Below the invisible input */
    z-index: 0;
  }

  .underlay-totp-input__slot--active {
    border-color: var(--underlay-color-primary, #14b8a6);
    box-shadow: 0 0 0 2px rgba(20, 184, 166, 0.25);
  }

  .underlay-totp-input--disabled .underlay-totp-input__slot {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .underlay-totp-input--error .underlay-totp-input__slot {
    border-color: var(--underlay-color-error, #ef4444);
  }

  .underlay-totp-input--error .underlay-totp-input__slot--active {
    border-color: var(--underlay-color-error, #ef4444);
    box-shadow: 0 0 0 2px rgba(239, 68, 68, 0.25);
  }

  /* Add visual separator after 3rd digit for 6-digit codes */
  .underlay-totp-input__slot:nth-child(4) {
    margin-right: 0.75rem;
  }

  @media (max-width: 400px) {
    .underlay-totp-input {
      gap: 0.35rem;
    }

    .underlay-totp-input__slot {
      width: 2.25rem;
      height: 2.75rem;
      font-size: 1.25rem;
    }

    .underlay-totp-input__slot:nth-child(4) {
      margin-right: 0.5rem;
    }
  }
</style>
