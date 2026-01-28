<script lang="ts">
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
    if (!rfc3339) {
      if (defaultToNow) {
        return getCurrentDateTimeLocal();
      }
      return "";
    }
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
    // datetime-local gives us YYYY-MM-DDTHH:MM, we need to add seconds and timezone
    // Parse as local time and convert to UTC
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

  // Internal value in datetime-local format
  let localValue = $state(toDateTimeLocal(value));

  // Sync external value changes to local value
  $effect(() => {
    const converted = toDateTimeLocal(value);
    if (converted !== localValue) {
      localValue = converted;
    }
  });

  function handleChange(newLocalValue: string) {
    localValue = newLocalValue;
    const rfc3339Value = toRfc3339(newLocalValue);
    value = rfc3339Value;
    onchange?.(rfc3339Value);
  }
</script>

<TextInput
  type="datetime-local"
  bind:value={localValue}
  onchange={handleChange}
  {name}
  {required}
  {placeholder}
  {disabled}
  {id}
  class={className}
/>
