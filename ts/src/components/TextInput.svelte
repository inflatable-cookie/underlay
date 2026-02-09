<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";
  import { getContext, onMount, untrack } from "svelte";
  import TextInputAffordances from "./text-input/TextInputAffordances.svelte";
  import { createStableId } from "../patterns/dom";

  export interface ValidationResult {
    valid: boolean;
    message?: string;
    suggestion?: string;
  }

  type ValidationStatus = "idle" | "validating" | "valid" | "invalid";

  interface Props extends Omit<HTMLInputAttributes, "value" | "oninput" | "onchange" | "type"> {
    type?: string;
    value?: string;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    inputRef?: HTMLInputElement | null;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
    /** Debounce delay in ms. When set, onchange fires after delay instead of on blur. */
    debounce?: number;
    /** Show a clear button when input has content. */
    search?: boolean;
    /** Async validation function */
    validate?: (value: string, context?: unknown) => Promise<ValidationResult>;
    /** Context to pass to validation function */
    validationContext?: unknown;
    /** Debounce delay for validation in ms (default: 300) */
    validationDebounce?: number;
    /** Show validation status indicator (default: true if validate provided) */
    showValidationStatus?: boolean;
    /** Validate on blur in addition to on change (default: true) */
    validateOnBlur?: boolean;
    /** Callback when validation state changes */
    onvalidationchange?: (status: ValidationStatus, isValid: boolean) => void;
    /** Static prefix to display before the input (e.g., "A", "$") */
    prefix?: string;
    /** Suffix content to render inside the wrapper (e.g., stepper buttons) */
    suffix?: Snippet;
  }

  let {
    type = "text",
    value = $bindable(""),
    autocomplete = "off",
    inputRef = $bindable(null),
    oninput,
    onchange,
    debounce,
    search = false,
    validate,
    validationContext,
    validationDebounce = 300,
    showValidationStatus = validate !== undefined,
    validateOnBlur = true,
    onvalidationchange,
    prefix,
    suffix,
    class: className,
    id,
    required = false,
    ...restProps
  }: Props = $props();

  // Get FormValidationProvider context if present
  const formValidation = getContext<{
    registerField: (id: string, required: boolean, hasValue: boolean, validationStatus: string, isValidationValid: boolean) => void;
    unregisterField: (id: string) => void;
    updateField: (id: string, hasValue: boolean, validationStatus?: string, isValidationValid?: boolean) => void;
  } | undefined>("formValidation");

  // Generate stable ID for form validation tracking
  // Note: fieldId intentionally captures id once - we want a stable ID for the lifetime of the component
  const fieldId = untrack(() => id) ?? createStableId("underlay-text-input");
  const isRequired = $derived(required ?? false);

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let validationTimer: ReturnType<typeof setTimeout> | null = null;

  // Validation state
  let validationStatus = $state<ValidationStatus>("idle");
  let validationMessage = $state<string>("");
  let lastValidatedValue = $state<string>("");
  let hasUserInteracted = $state(false);

  // Serialize context for value-based comparison (avoids proxy reference issues)
  const contextKey = $derived(JSON.stringify(validationContext ?? null));
  // Initialize lastContextKey with the initial value for comparison tracking
  let lastContextKey = $state(untrack(() => JSON.stringify(validationContext ?? null)));

  const showClearButton = $derived(search && value.length > 0);
  const showValidationIcon = $derived(showValidationStatus && validationStatus !== "idle");
  const needsWrapper = $derived(search || showValidationStatus || prefix || suffix);

  // Trigger validation when value changes (even if auto-generated)
  $effect(() => {
    if (validate && value !== lastValidatedValue) {
      triggerValidation(value);
    }
  });

  // Revalidate when context changes (use serialized key for value comparison)
  $effect(() => {
    if (contextKey !== lastContextKey) {
      lastContextKey = contextKey;
      if (validate && value) {
        triggerValidation(value);
      }
    }
  });

  // Cleanup on unmount
  $effect(() => {
    return () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      if (validationTimer) {
        clearTimeout(validationTimer);
      }
    };
  });

  // Track previous values to prevent unnecessary updates
  let prevValue = $state("");
  let prevValidationStatus = $state<ValidationStatus>("idle");

  // Helper to check if field has value (handles both string and number inputs)
  function checkHasValue(val: string | number): boolean {
    if (typeof val === 'number') {
      return !isNaN(val);
    }
    return String(val).trim() !== "";
  }

  // Register with FormValidationProvider on mount
  onMount(() => {
    if (formValidation) {
      const hasValue = untrack(() => checkHasValue(value));
      const isValid = untrack(() => validationStatus === "valid" || validationStatus === "idle");
      const status = untrack(() => validationStatus);
      formValidation.registerField(fieldId, isRequired, hasValue, status, isValid);

      // Initialize previous values
      prevValue = untrack(() => value);
      prevValidationStatus = untrack(() => validationStatus);

      return () => {
        formValidation.unregisterField(fieldId);
      };
    }
  });

  // Update FormValidationProvider when value or validation actually changes
  $effect(() => {
    if (formValidation && (value !== prevValue || validationStatus !== prevValidationStatus)) {
      const hasValue = checkHasValue(value);
      const isValid = validationStatus === "valid" || validationStatus === "idle";
      formValidation.updateField(fieldId, hasValue, validationStatus, isValid);

      prevValue = value;
      prevValidationStatus = validationStatus;
    }
  });

  // Notify validation state changes
  $effect(() => {
    if (onvalidationchange && validate) {
      const isValid = validationStatus === "valid" || validationStatus === "idle";
      onvalidationchange(validationStatus, isValid);
    }
  });

  function triggerValidation(inputValue: string) {
    // Clear any pending validation
    if (validationTimer) {
      clearTimeout(validationTimer);
    }

    // Don't validate empty values
    if (!inputValue) {
      validationStatus = "idle";
      validationMessage = "";
      return;
    }

    // Set validating status immediately
    validationStatus = "validating";
    validationMessage = "";

    // Debounce the async validation
    const ctx = validationContext;
    validationTimer = setTimeout(async () => {
      try {
        const result = await validate!(inputValue, ctx);
        // Only update if this is still the current value
        if (inputValue === value) {
          lastValidatedValue = inputValue;
          validationStatus = result.valid ? "valid" : "invalid";
          validationMessage = result.message || "";
        }
      } catch (error) {
        // Only update if this is still the current value
        if (inputValue === value) {
          validationStatus = "idle";
          validationMessage = "Could not validate";
        }
      }
    }, validationDebounce);
  }

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement | null;
    const next = target ? target.value : value;
    hasUserInteracted = true;
    value = next;
    oninput?.(next);

    // If debounce is enabled, fire onchange after delay
    if (debounce && debounce > 0 && onchange) {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      debounceTimer = setTimeout(() => {
        onchange(next);
      }, debounce);
    }
  }

  function handleChange() {
    // Validate on blur if enabled
    if (validate && validateOnBlur && hasUserInteracted && value) {
      // Clear validation timer and validate immediately
      if (validationTimer) {
        clearTimeout(validationTimer);
      }
      triggerValidation(value);
    }

    // Only fire onchange on blur if debounce is not enabled
    if (!debounce || debounce <= 0) {
      onchange?.(value);
    }
  }

  function handleClear() {
    // Clear debounce and validation timers
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    if (validationTimer) {
      clearTimeout(validationTimer);
    }

    value = "";
    validationStatus = "idle";
    validationMessage = "";
    lastValidatedValue = "";

    onchange?.("");
    inputRef?.focus();
  }
</script>

{#if needsWrapper}
  <div class="underlay-input-wrapper {prefix ? 'underlay-input-wrapper--prefixed' : ''} {suffix ? 'underlay-input-wrapper--suffixed' : ''}">
    {#if prefix}
      <span class="underlay-input-prefix">{prefix}</span>
    {/if}
    <input
      {...restProps}
      id={fieldId}
      class={`underlay-input ${prefix ? "underlay-input--prefixed" : ""} ${search ? "underlay-input--search" : ""} ${showValidationIcon ? "underlay-input--validated" : ""} ${className ?? ""}`}
      {type}
      {autocomplete}
      {required}
      bind:this={inputRef}
      bind:value
      oninput={handleInput}
      onchange={handleChange}
    />
    <TextInputAffordances
      {showClearButton}
      {showValidationIcon}
      {showValidationStatus}
      {validationStatus}
      {validationMessage}
      onClear={handleClear}
    />
    {#if suffix}
      {@render suffix()}
    {/if}
  </div>
{:else}
  <input
    {...restProps}
    id={fieldId}
    class={`underlay-input ${className ?? ""}`}
    {type}
    {autocomplete}
    {required}
    bind:this={inputRef}
    bind:value
    oninput={handleInput}
    onchange={handleChange}
  />
{/if}

<style>
  .underlay-input-wrapper {
    position: relative;
    width: 100%;
  }

  .underlay-input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.55em 0.7em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
  }

  /* Native date/time inputs need explicit styling to match Select */
  .underlay-input[type="date"],
  .underlay-input[type="datetime-local"],
  .underlay-input[type="time"] {
    color-scheme: dark;
    font-family: inherit;
    font-size: 0.85rem;
    line-height: 1;
    /* Force exact height to match Select trigger */
    height: 2.2em;
    padding: 0 0.7em;
    /* Force background to override browser defaults */
    background-color: var(
      --underlay-color-field-bg,
      rgba(148, 163, 184, 0.18)
    ) !important;
  }

  /* Style the calendar picker icon for dark mode */
  .underlay-input[type="date"]::-webkit-calendar-picker-indicator,
  .underlay-input[type="datetime-local"]::-webkit-calendar-picker-indicator,
  .underlay-input[type="time"]::-webkit-calendar-picker-indicator {
    filter: invert(0.7);
    cursor: pointer;
    opacity: 0.7;
  }

  .underlay-input[type="date"]::-webkit-calendar-picker-indicator:hover,
  .underlay-input[type="datetime-local"]::-webkit-calendar-picker-indicator:hover,
  .underlay-input[type="time"]::-webkit-calendar-picker-indicator:hover {
    opacity: 1;
  }

  .underlay-input--search {
    padding-right: 2.2em;
  }

  .underlay-input--validated {
    padding-right: 2.5rem;
  }

  .underlay-input:focus,
  .underlay-input:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  /* Prefixed input styles */
  .underlay-input-wrapper--prefixed {
    display: flex;
    align-items: stretch;
    border: var(--underlay-field-border-width, 1px) solid var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-sm, 0.35rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
  }

  .underlay-input-wrapper--prefixed:focus-within {
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-input-prefix {
    display: flex;
    align-items: center;
    padding: 0 0.6em;
    font-family: var(--underlay-font-mono, monospace);
    font-size: 0.9em;
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    font-weight: 600;
    color: var(--underlay-color-text-muted, #9ca3af);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
    border-right: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-sm, 0.35rem) 0 0 var(--underlay-radius-sm, 0.35rem);
    user-select: none;
    white-space: nowrap;
  }

  .underlay-input--prefixed {
    flex: 1;
    min-width: 0;
    border: none !important;
    border-radius: 0 var(--underlay-radius-sm, 0.35rem) var(--underlay-radius-sm, 0.35rem) 0 !important;
    background: transparent !important;
  }

  .underlay-input--prefixed:focus,
  .underlay-input--prefixed:focus-visible {
    outline: none !important;
  }

  /* Readonly state - slightly dimmed */
  .underlay-input:read-only {
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: default;
  }

  /* Disabled state - more dimmed */
  .underlay-input:disabled {
    color: var(--underlay-color-text-disabled, rgba(156, 163, 175, 0.5));
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
