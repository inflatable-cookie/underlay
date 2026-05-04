<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    Field,
    FieldSet,
    TextInput,
    Select,
    Button,
    Callout,
    PageLoading
  } from "@poodle/svelte";
  import type { FieldConfig, FieldType, SelectOption } from "./types";

  export type { FieldConfig } from "./types";

  interface Props {
    /** Declarative field configuration */
    fields: FieldConfig[];
    /** Initial form values */
    initialValues?: Record<string, unknown>;
    /** External form values (controlled mode) */
    values?: Record<string, unknown>;
    /** Field-level errors from validation or API */
    fieldErrors?: Record<string, string>;
    /** Form-level error message */
    error?: string | null;
    /** Whether the form is submitting */
    submitting?: boolean;
    /** Whether the form is loading initial data */
    loading?: boolean;
    /** Submit button label */
    submitLabel?: string;
    /** Cancel button label */
    cancelLabel?: string;
    /** Whether to show cancel button */
    showCancel?: boolean;
    /** Called when form is submitted (after validation) */
    onSubmit: (values: Record<string, unknown>) => Promise<void> | void;
    /** Called when cancel is clicked */
    onCancel?: () => void;
    /** Custom validation function. Return field errors or null if valid. */
    validate?: (values: Record<string, unknown>) => Record<string, string> | null;
    /** Additional content below fields (e.g., relation selectors) */
    children?: Snippet;
  }

  // --- Props ---

  let {
    fields,
    initialValues = {},
    values: externalValues,
    fieldErrors: externalFieldErrors,
    error,
    submitting = false,
    loading = false,
    submitLabel = "Save",
    cancelLabel = "Cancel",
    showCancel = true,
    onSubmit,
    onCancel,
    validate,
    children
  }: Props = $props();

  // --- State ---

  // Internal form values (uncontrolled mode)
  let internalValues = $state<Record<string, unknown>>(
    Object.fromEntries(
      fields.map((f) => [f.id, initialValues[f.id] ?? defaultValueForType(f.type)])
    )
  );

  // Use external values if provided (controlled mode), otherwise internal
  let formValues = $derived(externalValues ?? internalValues);

  // Validation errors
  let validationErrors = $state<Record<string, string>>({});
  let touched = $state<Record<string, boolean>>({});

  // Combine external and validation errors
  const allFieldErrors = $derived({
    ...externalFieldErrors,
    ...validationErrors
  });

  // --- Helpers ---

  function defaultValueForType(type: FieldType): unknown {
    switch (type) {
      case "text":
      case "textarea":
        return "";
      case "select":
        return "";
      case "number":
        return null;
      case "checkbox":
        return false;
      case "custom":
        return undefined;
      default:
        return "";
    }
  }

  function handleFieldChange(id: string, value: unknown) {
    if (externalValues) {
      // Controlled mode — parent manages values
      // Emit change event through a different mechanism or assume parent handles it
      // For now, controlled mode requires parent to update values prop
    } else {
      internalValues = { ...internalValues, [id]: value };
    }
    touched = { ...touched, [id]: true };
    // Clear validation error when field is edited
    if (validationErrors[id]) {
      const { [id]: _, ...rest } = validationErrors;
      validationErrors = rest;
    }
  }

  function validateForm(): boolean {
    const errors: Record<string, string> = {};

    // Required field validation
    for (const field of fields) {
      if (field.required) {
        const value = formValues[field.id];
        const isEmpty =
          value === undefined ||
          value === null ||
          value === "" ||
          (typeof value === "string" && value.trim() === "");
        if (isEmpty) {
          errors[field.id] = `${field.label} is required`;
        }
      }
    }

    // Custom validation
    if (validate) {
      const customErrors = validate(formValues);
      if (customErrors) {
        Object.assign(errors, customErrors);
      }
    }

    validationErrors = errors;
    return Object.keys(errors).length === 0;
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!validateForm()) return;

    await onSubmit(formValues);
  }

  function handleCancel() {
    onCancel?.();
  }

  // Load async select options
  let loadedOptions = $state<Record<string, SelectOption[]>>({});
  let optionsLoading = $state<Record<string, boolean>>({});

  $effect(() => {
    for (const field of fields) {
      if (field.type === "select" && field.loadOptions && !loadedOptions[field.id] && !optionsLoading[field.id]) {
        optionsLoading = { ...optionsLoading, [field.id]: true };
        field.loadOptions().then((options) => {
          loadedOptions = { ...loadedOptions, [field.id]: options };
          optionsLoading = { ...optionsLoading, [field.id]: false };
        }).catch(() => {
          optionsLoading = { ...optionsLoading, [field.id]: false };
        });
      }
    }
  });
</script>

{#if loading}
  <PageLoading presentation="inline" message="Loading form..." />
{:else}
  <form class="entity-form" onsubmit={handleSubmit}>
    {#if error}
      <Callout tone="danger" message={error} announceMode="polite" />
    {/if}

    <FieldSet>
      {#each fields as field (field.id)}
        {@const value = formValues[field.id]}
        {@const fieldError = allFieldErrors[field.id]}
        {@const isDisabled = submitting}

        {#if field.type === "custom"}
          {@render field.render({
            value,
            onChange: (v) => handleFieldChange(field.id, v),
            error: fieldError,
            disabled: isDisabled
          })}
        {:else}
          <Field
            id={field.id}
            label={field.label}
            required={field.required}
            error={fieldError}
            helpText={field.helpText}
          >
            {#if field.type === "text"}
              <TextInput
                id={field.id}
                type="text"
                value={String(value ?? "")}
                placeholder={field.placeholder}
                disabled={isDisabled}
                oninput={(e: Event) => handleFieldChange(field.id, (e.currentTarget as HTMLInputElement).value)}
              />
            {:else if field.type === "textarea"}
              <textarea
                id={field.id}
                class="poodle-textarea"
                rows={field.rows ?? 4}
                placeholder={field.placeholder}
                disabled={isDisabled}
                value={String(value ?? "")}
                oninput={(e: Event) => handleFieldChange(field.id, (e.currentTarget as HTMLTextAreaElement).value)}
              ></textarea>
            {:else if field.type === "select"}
              <Select
                id={field.id}
                value={String(value ?? "")}
                items={loadedOptions[field.id] ?? field.options ?? []}
                placeholder={field.placeholder}
                disabled={isDisabled}
                onchange={(e: Event) => handleFieldChange(field.id, (e.currentTarget as HTMLSelectElement).value)}
              />
            {:else if field.type === "number"}
              <input
                id={field.id}
                type="number"
                class="poodle-input"
                value={value ?? ""}
                min={field.min}
                max={field.max}
                step={field.step}
                placeholder={field.placeholder}
                disabled={isDisabled}
                oninput={(e: Event) => {
                  const val = (e.currentTarget as HTMLInputElement).value;
                  handleFieldChange(field.id, val === "" ? null : Number(val));
                }}
              />
            {:else if field.type === "checkbox"}
              <label class="checkbox-label">
                <input
                  id={field.id}
                  type="checkbox"
                  checked={Boolean(value)}
                  disabled={isDisabled}
                  onchange={(e: Event) => handleFieldChange(field.id, (e.currentTarget as HTMLInputElement).checked)}
                />
                {field.checkboxLabel ?? field.label}
              </label>
            {/if}
          </Field>
        {/if}
      {/each}
    </FieldSet>

    {#if children}
      {@render children()}
    {/if}

    <div class="form-actions">
      <Button
        type="submit"
        variant="primary"
        loading={submitting}
        disabled={submitting}
      >
        {submitLabel}
      </Button>
      {#if showCancel && onCancel}
        <Button
          type="button"
          variant="secondary"
          disabled={submitting}
          onclick={handleCancel}
        >
          {cancelLabel}
        </Button>
      {/if}
    </div>
  </form>
{/if}

<style>
  .entity-form {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .form-actions {
    display: flex;
    gap: var(--underlay-space-3, 0.75rem);
    margin-top: var(--underlay-space-4, 1rem);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    cursor: pointer;
  }

  .checkbox-label input[type="checkbox"] {
    cursor: pointer;
  }

  .poodle-textarea,
  .poodle-input {
    width: 100%;
    padding: var(--underlay-space-2, 0.5rem);
    border: 1px solid var(--underlay-color-border-subtle, #e5e7eb);
    border-radius: var(--underlay-radius-md, 0.375rem);
    background: var(--underlay-color-surface, #fff);
    color: var(--underlay-color-text-primary, #111827);
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
  }

  .poodle-textarea:focus,
  .poodle-input:focus {
    outline: none;
    border-color: var(--underlay-color-border-focus, #3b82f6);
    box-shadow: 0 0 0 2px var(--underlay-color-focus-ring, rgba(59, 130, 246, 0.2));
  }

  .poodle-textarea:disabled,
  .poodle-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .poodle-textarea {
    resize: vertical;
    min-height: 6rem;
  }
</style>
