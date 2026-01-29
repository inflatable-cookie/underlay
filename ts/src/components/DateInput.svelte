<script lang="ts">
  import { untrack } from "svelte";
  import TextInput, { type ValidationResult } from "./TextInput.svelte";

  type ValidationStatus = "idle" | "validating" | "valid" | "invalid";

  interface Props {
    /** ISO date string (e.g., "2024-01-28") */
    value?: string;
    /** Whether to default to current date when value is empty (useful for create forms) */
    defaultToNow?: boolean;
    /** Callback when value changes (receives ISO date string) */
    onchange?: (value: string) => void;
    /** Input name for form submission */
    name?: string;
    /** Whether the field is required */
    required?: boolean;
    /** Placeholder text */
    placeholder?: string;
    /** Whether the input is disabled */
    disabled?: boolean;
    /** Input ID */
    id?: string;
    /** Additional CSS class */
    class?: string;
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
    value = $bindable(""),
    defaultToNow = false,
    onchange,
    name,
    required,
    placeholder,
    disabled,
    id,
    class: className,
    validate,
    validationContext,
    validationDebounce,
    showValidationStatus,
    validateOnBlur,
    onvalidationchange
  }: Props = $props();

  /**
   * Extract date portion from various date formats.
   * Returns YYYY-MM-DD format.
   */
  function toDateString(input: string | undefined | null): string {
    if (!input) return "";
    try {
      // If it's already in YYYY-MM-DD format, return as-is
      if (/^\d{4}-\d{2}-\d{2}$/.test(input)) {
        return input;
      }
      // Otherwise parse and extract date
      const date = new Date(input);
      if (isNaN(date.getTime())) return "";
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1).padStart(2, "0");
      const day = String(date.getDate()).padStart(2, "0");
      return `${year}-${month}-${day}`;
    } catch {
      return "";
    }
  }

  /**
   * Get current date in YYYY-MM-DD format.
   */
  function getCurrentDate(): string {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, "0");
    const day = String(now.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
  }

  // Compute initial local value and sync to parent if defaulting to now
  function getInitialLocalValue(): string {
    const converted = toDateString(value);
    if (converted) return converted;
    if (defaultToNow) return getCurrentDate();
    return "";
  }

  // Internal value in date format - only set once on init
  let localValue = $state(untrack(() => getInitialLocalValue()));

  // Track the last value we synced FROM to detect external changes
  let lastExternalValue = $state(untrack(() => value));

  // Flag to track if we've synced the default value
  let hasInitialized = $state(false);

  // If we defaulted to now, sync that value back to the parent immediately (runs once)
  $effect(() => {
    if (!hasInitialized && defaultToNow && !lastExternalValue && localValue) {
      hasInitialized = true;
      lastExternalValue = localValue;
      value = localValue;
      onchange?.(localValue);
    }
  });

  // Sync external value changes to local value (only when value changes externally)
  $effect(() => {
    // Only sync if the external value actually changed (not from our own update)
    if (value !== lastExternalValue) {
      const converted = toDateString(value);
      if (converted) {
        localValue = converted;
      }
      lastExternalValue = value;
    }
  });

  // Update external value when local value changes (user input)
  function handleInput(newLocalValue: string) {
    localValue = newLocalValue;
    // Update tracking to prevent the effect from overwriting
    lastExternalValue = newLocalValue;
    value = newLocalValue;
    onchange?.(newLocalValue);
  }

  // Compute the value for form submission
  const formValue = $derived(value || localValue);
</script>

<!-- Hidden input for form submission -->
{#if name}
  <input type="hidden" {name} value={formValue} />
{/if}

<!-- Visible date picker (no name to avoid duplicate submission) -->
<TextInput
  type="date"
  value={localValue}
  oninput={handleInput}
  {required}
  {placeholder}
  {disabled}
  {id}
  class={className}
  {validate}
  {validationContext}
  {validationDebounce}
  {showValidationStatus}
  {validateOnBlur}
  {onvalidationchange}
/>
