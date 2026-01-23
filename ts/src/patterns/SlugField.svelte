<script lang="ts">
  import { Field, TextInput, type ValidationResult } from "@decodelabs/underlay/components";
  import { slugify, isValidSlugFormat, isReservedSlug } from "./slugify";

  interface Props {
    /** Current slug value (bindable) */
    value: string;
    /** Source value to generate slug from (e.g., title) */
    source?: string;
    /** Async function to validate slug (uses ValidationResult format) */
    validate?: (slug: string) => Promise<ValidationResult>;
    /** Revalidate when this value changes */
    validationKey?: unknown;
    /** Field label */
    label?: string;
    /** Field name for form submission */
    name?: string;
    /** ID of existing entity (for edit mode - excluded from uniqueness check) */
    excludeId?: string | null;
    /** Debounce delay in ms (default: 300) */
    debounceMs?: number;
    /** Whether field is disabled */
    disabled?: boolean;
    /** Hint text shown below field */
    hint?: string;
    /** Input element ID */
    id?: string;
    /** Error message from form validation */
    error?: string;
    /** Maximum length for the slug */
    maxlength?: number;
  }

  let {
    value = $bindable(""),
    source = "",
    validate,
    validationKey = null,
    label = "Slug",
    name = "slug",
    excludeId = null,
    debounceMs = 300,
    disabled = false,
    hint,
    id,
    error,
    maxlength,
  }: Props = $props();

  // Internal state for slug-specific features
  let hasManuallyEdited = $state(false);
  let prevSource = $state(source);

  // Auto-sync slug from source when source changes (unless manually edited)
  $effect(() => {
    if (source !== prevSource) {
      prevSource = source;
      if (!hasManuallyEdited && source) {
        const newSlug = slugify(source);
        if (newSlug !== value) {
          value = newSlug;
        }
      }
    }
  });

  // Adapter: Adds slug-specific format/reserved checks before async validation
  async function validateForTextInput(slug: string): Promise<ValidationResult> {
    // Check format first (synchronously)
    const maxLen = maxlength ?? 100;
    if (!isValidSlugFormat(slug, maxLen)) {
      if (slug.length < 2) {
        return { valid: false, message: "Too short (min 2 characters)" };
      } else if (slug.length > maxLen) {
        return { valid: false, message: `Too long (max ${maxLen} characters)` };
      } else {
        return { valid: false, message: "Invalid format (use lowercase letters, numbers, hyphens)" };
      }
    }

    // Check reserved (synchronously)
    if (isReservedSlug(slug)) {
      return { valid: false, message: "This slug is reserved" };
    }

    // If no async validator, mark as valid format
    if (!validate) {
      return { valid: true, message: "Valid format" };
    }

    // Call the async validator (already returns ValidationResult)
    try {
      return await validate(slug);
    } catch {
      return { valid: false, message: "Could not verify availability" };
    }
  }

  function handleInput(newValue: string) {
    hasManuallyEdited = true;
    value = newValue;
  }

  function handleBlur() {
    // Normalize the slug on blur
    if (value) {
      const normalized = slugify(value);
      if (normalized !== value) {
        value = normalized;
      }
    }
  }
</script>

<Field {label} forId={id} error={error} {hint}>
  <TextInput
    {id}
    {name}
    {disabled}
    {maxlength}
    bind:value
    oninput={handleInput}
    onblur={handleBlur}
    validate={validate ? validateForTextInput : undefined}
    validationContext={validationKey}
    validationDebounce={debounceMs}
    placeholder="url-friendly-slug"
    autocomplete="off"
    spellcheck={false}
    class="slug-field__input"
  />
</Field>

<style>
  :global(.slug-field__input) {
    font-family: var(--underlay-font-mono, monospace);
  }
</style>
