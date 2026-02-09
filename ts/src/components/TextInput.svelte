<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";
  import { getContext, onMount, untrack } from "svelte";
  import TextInputField from "./text-input/TextInputField.svelte";
  import { createStableId } from "../patterns/dom";
  import {
    registerFormValidationField,
    updateFormValidationField,
    type FormValidationContext
  } from "./text-input/form-validation";
  import {
    isValidationStatusValid,
    type InputValidationStatus
  } from "./text-input/validation-state";

  export interface ValidationResult {
    valid: boolean;
    message?: string;
    suggestion?: string;
  }

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
    onvalidationchange?: (status: InputValidationStatus, isValid: boolean) => void;
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
  const formValidation = getContext<FormValidationContext | undefined>("formValidation");

  // Generate stable ID for form validation tracking
  // Note: fieldId intentionally captures id once - we want a stable ID for the lifetime of the component
  const fieldId = untrack(() => id) ?? createStableId("underlay-text-input");
  const isRequired = $derived(required ?? false);

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let validationTimer: ReturnType<typeof setTimeout> | null = null;

  // Validation state
  let validationStatus = $state<InputValidationStatus>("idle");
  let validationMessage = $state<string>("");
  let lastValidatedValue = $state<string>("");
  let hasUserInteracted = $state(false);

  // Serialize context for value-based comparison (avoids proxy reference issues)
  const contextKey = $derived(JSON.stringify(validationContext ?? null));
  // Initialize lastContextKey with the initial value for comparison tracking
  let lastContextKey = $state(untrack(() => JSON.stringify(validationContext ?? null)));

  const showClearButton = $derived(search && value.length > 0);
  const showValidationIcon = $derived(showValidationStatus && validationStatus !== "idle");
  const needsWrapper = $derived(Boolean(search || showValidationStatus || prefix || suffix));

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
  let prevValidationStatus = $state<InputValidationStatus>("idle");

  // Register with FormValidationProvider on mount
  onMount(() => {
    if (formValidation) {
      const status = untrack(() => validationStatus);
      const inputValue = untrack(() => value);
      registerFormValidationField(formValidation, fieldId, isRequired, inputValue, status);

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
      updateFormValidationField(formValidation, fieldId, value, validationStatus);

      prevValue = value;
      prevValidationStatus = validationStatus;
    }
  });

  // Notify validation state changes
  $effect(() => {
    if (onvalidationchange && validate) {
      const isValid = isValidationStatusValid(validationStatus);
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

<TextInputField
  bind:value
  bind:inputRef
  {needsWrapper}
  {fieldId}
  {type}
  {autocomplete}
  required={required ?? false}
  {className}
  {search}
  {showValidationIcon}
  {showClearButton}
  {showValidationStatus}
  {validationStatus}
  {validationMessage}
  {prefix}
  {suffix}
  onInput={handleInput}
  onChange={handleChange}
  onClear={handleClear}
  {restProps}
/>
