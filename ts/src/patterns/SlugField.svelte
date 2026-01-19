<script lang="ts">
  import { Field, TextInput } from "@decodelabs/underlay/components";
  import { slugify, isValidSlugFormat, isReservedSlug, type SlugValidationResult } from "./slugify";

  type SlugStatus = "idle" | "checking" | "available" | "taken" | "invalid";

  interface Props {
    /** Current slug value (bindable) */
    value: string;
    /** Source value to generate slug from (e.g., title) */
    source?: string;
    /** Async function to validate slug availability */
    validate?: (slug: string, validationKey?: unknown) => Promise<SlugValidationResult>;
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

  // Internal state
  let status = $state<SlugStatus>("idle");
  let statusMessage: string = $state("");
  let hasManuallyEdited: boolean = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let lastValidatedSlug: string = "";
  let lastValidationKey: unknown = $state(validationKey);

  // Track the previous source to detect changes
  let prevSource: string = $state(source);

  // Auto-sync slug from source when source changes (unless manually edited)
  $effect(() => {
    if (source !== prevSource) {
      prevSource = source;
      if (!hasManuallyEdited && source) {
        const newSlug = slugify(source);
        if (newSlug !== value) {
          value = newSlug;
          triggerValidation(newSlug);
        }
      }
    }
  });

  // Validate when value changes (debounced)
  $effect(() => {
    if (value && value !== lastValidatedSlug) {
      triggerValidation(value);
    }
  });

  // Revalidate when external context changes
  $effect(() => {
    if (validationKey !== lastValidationKey) {
      lastValidationKey = validationKey;
      if (value) {
        triggerValidation(value);
      }
    }
  });

  function triggerValidation(slug: string) {
    // Clear any pending debounce
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    // Don't validate empty slugs
    if (!slug) {
      status = "idle";
      statusMessage = "";
      return;
    }

    // Check format first (synchronously)
    const maxLen = maxlength ?? 100;
    if (!isValidSlugFormat(slug, maxLen)) {
      status = "invalid";
      if (slug.length < 2) {
        statusMessage = "Too short (min 2 characters)";
      } else if (slug.length > maxLen) {
        statusMessage = `Too long (max ${maxLen} characters)`;
      } else {
        statusMessage = "Invalid format (use lowercase letters, numbers, hyphens)";
      }
      lastValidatedSlug = slug;
      return;
    }

    // Check reserved (synchronously)
    if (isReservedSlug(slug)) {
      status = "invalid";
      statusMessage = "This slug is reserved";
      lastValidatedSlug = slug;
      return;
    }

    // If no async validator, mark as valid format
     if (!validate) {
       status = "available";
       statusMessage = "Valid format";
       lastValidatedSlug = slug;
       return;
     }

     // Debounce the async validation
     status = "checking";
     statusMessage = "Checking availability...";

     const key = validationKey;
     debounceTimer = setTimeout(async () => {
       try {
         const result = await validate(slug, key);
         // Only update if this is still the current value
         if (slug === value) {
           lastValidatedSlug = slug;
if (result.available) {
              status = "available";
              statusMessage = "Slug is available";
            } else {
              status = "taken";
              statusMessage = result.message
                ? result.message
                : result.reason === "already_exists"
                  ? "This slug is already in use"
                  : result.reason === "reserved"
                    ? "This slug is reserved"
                    : "Invalid slug";
            }
         }
       } catch {
         // Only update if this is still the current value
         if (slug === value) {
           status = "idle";
           statusMessage = "Could not verify availability";
         }
       }
     }, debounceMs);
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

  // Cleanup on unmount
  $effect(() => {
    return () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
    };
  });

  // Derive the status indicator icon/class
  const statusClass = $derived(
    status === "checking" ? "slug-field--checking" :
    status === "available" ? "slug-field--available" :
    status === "taken" ? "slug-field--taken" :
    status === "invalid" ? "slug-field--invalid" :
    ""
  );
</script>

<Field {label} forId={id} error={error} {hint}>
  <div class="slug-field {statusClass}">
    <TextInput
      {id}
      {name}
      {disabled}
      {maxlength}
      value={value}
      oninput={handleInput}
      onblur={handleBlur}
      placeholder="url-friendly-slug"
      autocomplete="off"
      spellcheck={false}
    />
    <div class="slug-field__status" aria-live="polite">
      {#if status === "checking"}
        <span class="slug-field__spinner" aria-label="Checking"></span>
      {:else if status === "available"}
        <span class="slug-field__icon slug-field__icon--success" aria-label="Available">&#10003;</span>
      {:else if status === "taken" || status === "invalid"}
        <span class="slug-field__icon slug-field__icon--error" aria-label="Unavailable">&#10005;</span>
      {/if}
    </div>
  </div>
  {#if statusMessage && !error}
    <p class="slug-field__message slug-field__message--{status}">{statusMessage}</p>
  {/if}
</Field>

<style>
  .slug-field {
    position: relative;
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .slug-field :global(.underlay-input) {
    flex: 1;
    font-family: var(--underlay-font-mono, monospace);
  }

  .slug-field__status {
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

  .slug-field__spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid var(--underlay-color-text-muted, #9ca3af);
    border-top-color: transparent;
    border-radius: 50%;
    animation: slug-spin 0.8s linear infinite;
  }

  @keyframes slug-spin {
    to { transform: rotate(360deg); }
  }

  .slug-field__icon {
    font-size: 1rem;
    font-weight: bold;
  }

  .slug-field__icon--success {
    color: var(--underlay-color-success, #22c55e);
  }

  .slug-field__icon--error {
    color: var(--underlay-color-error, #ef4444);
  }

  .slug-field__message {
    margin: var(--underlay-space-2, 0.5rem) 0 0;
    font-size: var(--underlay-font-size-sm, 0.8rem);
  }

  .slug-field__message--checking {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .slug-field__message--available {
    color: var(--underlay-color-success, #22c55e);
  }

  .slug-field__message--taken,
  .slug-field__message--invalid {
    color: var(--underlay-color-error, #ef4444);
  }

  .slug-field__message--idle {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  /* Add right padding to input to make room for status icon */
  .slug-field--checking :global(.underlay-input),
  .slug-field--available :global(.underlay-input),
  .slug-field--taken :global(.underlay-input),
  .slug-field--invalid :global(.underlay-input) {
    padding-right: 2.5rem;
  }
</style>
