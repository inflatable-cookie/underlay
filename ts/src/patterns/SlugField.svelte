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
    /** Whether field is required */
    required?: boolean;
    /** Hint text shown below field */
    hint?: string;
    /** Input element ID */
    id?: string;
    /** Error message from form validation */
    error?: string;
    /** Maximum length for the slug */
    maxlength?: number;
    /** Callback when validation state changes */
    onvalidationchange?: (status: string, isValid: boolean) => void;
    /** Static prefix to display before the slug (e.g., "sa3f2e-") */
    prefix?: string;
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
    required = false,
    hint,
    id,
    error,
    maxlength,
    onvalidationchange,
    prefix,
  }: Props = $props();

  // Track the last auto-generated slug to detect user modifications.
  // This approach handles pre-populated values (edit mode) correctly:
  // - If value is empty or matches lastAutoSlug, we auto-generate
  // - If value differs (e.g., server-provided custom slug), we preserve it
  let lastAutoSlug = $state("");

  // Auto-sync slug from source (unless user has customized it)
  $effect(() => {
    const nextAutoSlug = source ? slugify(source) : "";
    // Only update if value is empty OR matches what we last auto-generated
    if (!value || value === lastAutoSlug) {
      if (nextAutoSlug !== value) {
        value = nextAutoSlug;
      }
    }
    lastAutoSlug = nextAutoSlug;
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
    // When user types, the value will differ from lastAutoSlug,
    // which prevents future auto-generation from overwriting their edit
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

<Field {label} forId={id} error={error} {hint} {required}>
  {#if prefix}
    <div class="slug-field-wrapper">
      <span class="slug-field-prefix"><span class="slug-field-prefix__text">{prefix}</span></span>
      <TextInput
        {id}
        {name}
        {disabled}
        {required}
        {maxlength}
        bind:value
        oninput={handleInput}
        onblur={handleBlur}
        validate={validate ? validateForTextInput : undefined}
        validationContext={validationKey}
        validationDebounce={debounceMs}
        onvalidationchange={onvalidationchange}
        placeholder="url-friendly-slug"
        autocomplete="off"
        spellcheck={false}
        class="slug-field__input"
      />
    </div>
  {:else}
    <TextInput
      {id}
      {name}
      {disabled}
      {required}
      {maxlength}
      bind:value
      oninput={handleInput}
      onblur={handleBlur}
      validate={validate ? validateForTextInput : undefined}
      validationContext={validationKey}
      validationDebounce={debounceMs}
      onvalidationchange={onvalidationchange}
      placeholder="url-friendly-slug"
      autocomplete="off"
      spellcheck={false}
      class="slug-field__input"
    />
  {/if}
</Field>

<style>
  .slug-field-wrapper {
    display: flex;
    align-items: stretch;
    width: 100%;
    border: var(--underlay-field-border-width, 1px) solid var(--underlay-color-border, rgba(148, 163, 184, 0.35));
    border-radius: var(--underlay-radius-sm, 0.35rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
    overflow: hidden;
  }

  .slug-field-wrapper:focus-within {
    border-color: var(--underlay-color-primary, #2563eb);
    outline: var(--underlay-focus-ring-width, 2px) solid var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 1px);
  }

  .slug-field-prefix {
    display: flex;
    align-items: center;
    padding-left: var(--underlay-field-padding-inline, 0.7em);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.08));
    user-select: none;
  }

  .slug-field-prefix__text {
    margin-right: -0.5ch;
    font-family: var(--underlay-font-mono, monospace);
    font-size: var(--underlay-font-size-md, 0.85rem);
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
    color: var(--underlay-color-text-muted, #9ca3af);
    white-space: nowrap;
  }

  /* Remove border/outline from the input when inside wrapper */
  .slug-field-wrapper :global(.underlay-input) {
    border: none !important;
    outline: none !important;
    background: transparent !important;
    flex: 1;
    min-width: 0;
  }

  /* Remove padding-left so text joins up with prefix */
  .slug-field-wrapper :global(.underlay-input input) {
    padding-left: 0 !important;
  }

  :global(.slug-field__input) {
    font-family: var(--underlay-font-mono, monospace);
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
  }

  :global(.underlay-input.slug-field__input) {
    font-size-adjust: var(--underlay-font-mono-size-adjust, 0.52);
  }
</style>
