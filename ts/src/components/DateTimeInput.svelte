<script lang="ts">
  import { untrack } from "svelte";
  import TextInput from "./TextInput.svelte";

  interface Props {
    /** RFC3339 datetime string (e.g., "2024-01-28T14:30:00Z") */
    value?: string;
    /** Whether to default to current datetime when value is empty (useful for create forms) */
    defaultToNow?: boolean;
    /** Callback when value changes (receives RFC3339 string) */
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
    class: className
  }: Props = $props();

  /**
   * Convert RFC3339 datetime to datetime-local format (YYYY-MM-DDTHH:MM).
   * Uses local timezone for display.
   */
  function toDateTimeLocal(rfc3339: string | undefined | null): string {
    if (!rfc3339) return "";
    try {
      const date = new Date(rfc3339);
      if (isNaN(date.getTime())) return "";
      const year = date.getFullYear();
      const month = String(date.getMonth() + 1).padStart(2, "0");
      const day = String(date.getDate()).padStart(2, "0");
      const hours = String(date.getHours()).padStart(2, "0");
      const minutes = String(date.getMinutes()).padStart(2, "0");
      return `${year}-${month}-${day}T${hours}:${minutes}`;
    } catch {
      return "";
    }
  }

  /**
   * Convert datetime-local format to RFC3339 (with UTC timezone).
   */
  function toRfc3339(dateTimeLocal: string): string {
    if (!dateTimeLocal) return "";
    try {
      const date = new Date(dateTimeLocal);
      if (isNaN(date.getTime())) return "";
      return date.toISOString();
    } catch {
      return "";
    }
  }

  /**
   * Get current datetime in datetime-local format.
   */
  function getCurrentDateTimeLocal(): string {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, "0");
    const day = String(now.getDate()).padStart(2, "0");
    const hours = String(now.getHours()).padStart(2, "0");
    const minutes = String(now.getMinutes()).padStart(2, "0");
    return `${year}-${month}-${day}T${hours}:${minutes}`;
  }

  // Compute initial local value
  function getInitialLocalValue(): string {
    const converted = toDateTimeLocal(value);
    if (converted) return converted;
    if (defaultToNow) return getCurrentDateTimeLocal();
    return "";
  }

  // Internal value in datetime-local format - only set once on init
  let localValue = $state(untrack(() => getInitialLocalValue()));

  // Track the last value we synced FROM to detect external changes
  let lastExternalValue = $state(untrack(() => value));

  // Sync external value changes to local value (only when value changes externally)
  $effect(() => {
    // Only sync if the external value actually changed (not from our own update)
    if (value !== lastExternalValue) {
      const converted = toDateTimeLocal(value);
      if (converted) {
        localValue = converted;
      }
      lastExternalValue = value;
    }
  });

  // Update external value when local value changes (user input)
  function handleInput(newLocalValue: string) {
    localValue = newLocalValue;
    const rfc3339Value = toRfc3339(newLocalValue);
    // Update tracking to prevent the effect from overwriting
    lastExternalValue = rfc3339Value;
    value = rfc3339Value;
    onchange?.(rfc3339Value);
  }
</script>

<TextInput
  type="datetime-local"
  value={localValue}
  oninput={handleInput}
  {name}
  {required}
  {placeholder}
  {disabled}
  {id}
  class={className}
/>
