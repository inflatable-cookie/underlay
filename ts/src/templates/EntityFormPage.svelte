<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    PageHeader,
    Button,
    Callout,
    PageLoading
  } from "@poodle/svelte";
  import EntityForm from "./EntityForm.svelte";
  import type { FieldConfig } from "./types";

  // Re-export types
  export type { FieldConfig } from "./types";

  // --- Types ---

  interface Props {
    /** Page title */
    title: string;
    /** Subtitle or description */
    subtitle?: string;
    /** Section label (e.g., "Edit Project") */
    section?: string;
    /** Back link URL */
    backHref?: string;
    /** Back link label */
    backLabel?: string;
    /** Banner message (e.g., for warnings) */
    bannerMessage?: string;
    /** Banner tone */
    bannerTone?: "warning" | "info" | "danger";
    /** Data loading function (for edit mode) */
    dataLoader?: (fetch: typeof window.fetch, token: string | null) => Promise<Record<string, unknown> | null>;
    /** Declarative field configuration */
    fields: FieldConfig[];
    /** Custom validation function */
    validate?: (values: Record<string, unknown>) => Record<string, string> | null;
    /** Submit handler */
    onSubmit: (values: Record<string, unknown>) => Promise<void> | void;
    /** Cancel handler */
    onCancel?: () => void;
    /** Submit button label */
    submitLabel?: string;
    /** Cancel button label */
    cancelLabel?: string;
    /** Whether to show cancel button */
    showCancel?: boolean;
    /** Success message after submit */
    successMessage?: string;
    /** Additional content below fields */
    children?: Snippet;
    /** Additional actions in the header */
    headerActions?: Snippet;
  }

  // --- Props ---

  let {
    title,
    subtitle,
    section,
    backHref,
    backLabel,
    bannerMessage,
    bannerTone = "warning",
    dataLoader,
    fields,
    validate,
    onSubmit,
    onCancel,
    submitLabel = "Save",
    cancelLabel = "Cancel",
    showCancel = true,
    successMessage = "Saved successfully.",
    children,
    headerActions
  }: Props = $props();

  // --- State ---

  let submitting = $state(false);
  let loading = $state(false);
  let loadError = $state<string | null>(null);
  let initialValues = $state<Record<string, unknown>>({});
  let submitError = $state<string | null>(null);
  let fieldErrors = $state<Record<string, string>>({});
  let showSuccess = $state(false);

  // --- Data loading ---

  $effect(() => {
    if (!dataLoader) {
      loading = false;
      return;
    }

    loading = true;
    loadError = null;

    const load = async () => {
      try {
        const data = await dataLoader(fetch, null);
        if (data) {
          initialValues = data;
        }
      } catch (e) {
        loadError = e instanceof Error ? e.message : "Failed to load data";
      } finally {
        loading = false;
      }
    };

    load();
  });

  // --- Actions ---

  async function handleSubmit(values: Record<string, unknown>) {
    submitting = true;
    submitError = null;
    fieldErrors = {};
    showSuccess = false;

    try {
      await onSubmit(values);
      showSuccess = true;
    } catch (e) {
      if (e && typeof e === "object" && "fieldErrors" in e) {
        // Structured error with field errors
        fieldErrors = (e as { fieldErrors: Record<string, string> }).fieldErrors;
        submitError = (e as { message?: string }).message ?? "Validation failed";
      } else {
        submitError = e instanceof Error ? e.message : "Failed to save";
      }
    } finally {
      submitting = false;
    }
  }

  function handleCancel() {
    onCancel?.();
  }
</script>

<div class="entity-form-page">
  <PageHeader
    {title}
    {section}
    backHref={backHref ?? null}
    backLabel={backLabel}
    bannerMessage={bannerMessage}
    bannerTone={bannerTone}
  >
    {#snippet actions()}
      {#if headerActions}
        {@render headerActions()}
      {/if}
    {/snippet}
  </PageHeader>

  {#if loadError}
    <Callout tone="danger" message={loadError} announceMode="polite" />
  {:else if showSuccess}
    <Callout tone="success" message={successMessage} announceMode="polite" />
  {/if}

  <EntityForm
    {fields}
    {initialValues}
    fieldErrors={fieldErrors}
    error={submitError}
    {submitting}
    {loading}
    {submitLabel}
    {cancelLabel}
    {showCancel}
    onSubmit={handleSubmit}
    onCancel={handleCancel}
    {validate}
  >
    {#if children}
      {@render children()}
    {/if}
  </EntityForm>
</div>

<style>
  .entity-form-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
