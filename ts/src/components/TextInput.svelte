<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { getContext, onMount, untrack } from "svelte";
  import X from "lucide-svelte/icons/x";
  import Check from "lucide-svelte/icons/check";
  import AlertCircle from "lucide-svelte/icons/alert-circle";
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
  const fieldId = id ?? createStableId("underlay-text-input");

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let validationTimer: ReturnType<typeof setTimeout> | null = null;

  // Validation state
  let validationStatus = $state<ValidationStatus>("idle");
  let validationMessage = $state<string>("");
  let lastValidatedValue = $state<string>("");
  let lastValidationContext = $state<unknown>(validationContext);
  let hasUserInteracted = $state(false);

  const showClearButton = $derived(search && value.length > 0);
  const showValidationIcon = $derived(showValidationStatus && validationStatus !== "idle" && hasUserInteracted);
  const needsWrapper = $derived(search || showValidationStatus);

  // Trigger validation when value changes
  $effect(() => {
    if (validate && hasUserInteracted && value !== lastValidatedValue) {
      triggerValidation(value);
    }
  });

  // Revalidate when context changes
  $effect(() => {
    if (validationContext !== lastValidationContext) {
      lastValidationContext = validationContext;
      if (validate && hasUserInteracted && value) {
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

  // Register with FormValidationProvider on mount
  onMount(() => {
    if (formValidation) {
      const hasValue = untrack(() => value.trim() !== "");
      const isValid = untrack(() => validationStatus === "valid" || validationStatus === "idle");
      formValidation.registerField(fieldId, required, hasValue, untrack(() => validationStatus), isValid);

      return () => {
        formValidation.unregisterField(fieldId);
      };
    }
  });

  // Update FormValidationProvider when value or validation changes
  $effect(() => {
    if (formValidation) {
      const hasValue = value.trim() !== "";
      const isValid = validationStatus === "valid" || validationStatus === "idle";
      formValidation.updateField(fieldId, hasValue, validationStatus, isValid);
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
  <div class="underlay-input-wrapper">
    <input
      {...restProps}
      id={fieldId}
      class={`underlay-input ${search ? "underlay-input--search" : ""} ${showValidationIcon ? "underlay-input--validated" : ""} ${className ?? ""}`}
      {type}
      {autocomplete}
      {required}
      bind:this={inputRef}
      bind:value
      oninput={handleInput}
      onchange={handleChange}
    />
    {#if showClearButton}
      <button
        type="button"
        class="underlay-input-clear"
        aria-label="Clear"
        onclick={handleClear}
      >
        <X size="1em" strokeWidth={2.5} />
      </button>
    {:else if showValidationIcon}
      <div class="underlay-input-validation" aria-live="polite">
        {#if validationStatus === "validating"}
          <span class="underlay-input-validation__spinner" aria-label="Validating"></span>
        {:else if validationStatus === "valid"}
          <Check
            size="1em"
            class="underlay-input-validation__icon underlay-input-validation__icon--success"
            aria-label="Valid"
          />
        {:else if validationStatus === "invalid"}
          <AlertCircle
            size="1em"
            class="underlay-input-validation__icon underlay-input-validation__icon--error"
            aria-label="Invalid"
          />
        {/if}
      </div>
    {/if}
  </div>
  {#if validationMessage && showValidationStatus}
    <p class="underlay-input-validation__message underlay-input-validation__message--{validationStatus}">
      {validationMessage}
    </p>
  {/if}
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
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    font-size: var(--underlay-font-size-md, var(--underlay-font-size-md, 0.85rem));
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

  .underlay-input-clear {
    position: absolute;
    right: 0.5em;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.15em;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 0.2rem;
    opacity: 0.7;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  .underlay-input-clear:hover {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-input-clear:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  .underlay-input-validation {
    position: absolute;
    right: var(--underlay-space-3, 0.75rem);
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    pointer-events: none;
  }

  .underlay-input-validation__spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--underlay-color-text-muted, #9ca3af);
    border-top-color: transparent;
    border-radius: 50%;
    animation: underlay-input-spin 0.8s linear infinite;
  }

  @keyframes underlay-input-spin {
    to {
      transform: rotate(360deg);
    }
  }

  .underlay-input-validation__icon {
    font-size: 1rem;
  }

  :global(.underlay-input-validation__icon--success) {
    color: var(--underlay-color-success, #22c55e) !important;
  }

  :global(.underlay-input-validation__icon--error) {
    color: var(--underlay-color-danger, #ef4444) !important;
  }

  .underlay-input-validation__message {
    margin: var(--underlay-space-2, 0.5rem) 0 0;
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .underlay-input-validation__message--validating {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-input-validation__message--valid {
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-input-validation__message--invalid {
    color: var(--underlay-color-error, #ef4444);
  }
</style>
