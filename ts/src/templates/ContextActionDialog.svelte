<script lang="ts">
  import { on } from "svelte/events";
  import { Button, Checkbox, Dialog, NumberInput, Select, TextInput } from "@inflatable-cookie/poodle-svelte";
  import type {
    ContextActionDefinition,
    ContextActionDialogForm,
    ContextActionInputField,
    ContextActionRunState,
    ContextActionSubmitDetail
  } from "./contextual-action.types";

  interface Props {
    open?: boolean;
    action?: ContextActionDefinition | null;
    values?: Record<string, unknown>;
    selectedProviderKey?: string;
    selectedModelId?: string;
    runState?: ContextActionRunState;
    errorMessage?: string | null;
    submitLabel?: string;
    cancelLabel?: string;
    form?: ContextActionDialogForm | null;
    onOpenChange?: (open: boolean) => void;
    onSubmit?: (detail: ContextActionSubmitDetail) => void;
    onCancel?: () => void;
    onValueChange?: (fieldId: string, value: unknown) => void;
    onSelectedModelChange?: (providerKey: string, modelId: string) => void;
  }

  let {
    open = $bindable(false),
    action = null,
    values = $bindable<Record<string, unknown>>({}),
    selectedProviderKey = $bindable(""),
    selectedModelId = $bindable(""),
    runState = "idle",
    errorMessage = null,
    submitLabel = "Generate",
    cancelLabel = "Cancel",
    form = null,
    onOpenChange = undefined,
    onSubmit = undefined,
    onCancel = undefined,
    onValueChange = undefined,
    onSelectedModelChange = undefined
  }: Props = $props();

  const formId = $props.id();
  const modelId = `${formId}-model`;
  const busy = $derived(runState === "validating" || runState === "running");
  const activeForm = $derived(form ?? action?.form ?? null);
  const modelOptions = $derived(
    (action?.modelOptions ?? []).filter(
      (option) =>
        typeof option.providerKey === "string" &&
        option.providerKey.length > 0 &&
        typeof option.modelId === "string" &&
        option.modelId.length > 0
    )
  );
  const groupedModelOptions = $derived.by(() => {
    const groups = new Map<
      string,
      {
        label: string;
        options: { value: string; label: string; disabled?: boolean }[];
      }
    >();

    for (const option of modelOptions) {
      const group = groups.get(option.providerKey) ?? {
        label: option.providerLabel,
        options: []
      };

      group.options.push({
        value: `${option.providerKey}:${option.modelId}`,
        label: option.label,
        disabled: option.disabled
      });
      groups.set(option.providerKey, group);
    }

    return Array.from(groups.values());
  });
  const resolvedSubmitLabel = $derived(action?.submitLabel ?? submitLabel);

  let validationError = $state<string | null>(null);

  function setOpen(nextOpen: boolean): void {
    if (!nextOpen && busy) return;
    if (!nextOpen) {
      validationError = null;
    }
    open = nextOpen;
    onOpenChange?.(nextOpen);
  }

  function setValue(fieldId: string, value: unknown): void {
    validationError = null;
    values = { ...values, [fieldId]: value };
    onValueChange?.(fieldId, value);
  }

  function resolvedModel(): { providerKey?: string; modelId?: string } {
    if (selectedProviderKey && selectedModelId) {
      return { providerKey: selectedProviderKey, modelId: selectedModelId };
    }
    const available = modelOptions.filter((option) => !option.disabled);
    const fallback = available.find((option) =>
      option.providerKey === action?.defaultModelProviderKey && option.modelId === action.defaultModelId
    ) ?? available[0];
    if (fallback) {
      return { providerKey: fallback.providerKey, modelId: fallback.modelId };
    }
    return {};
  }

  function selectedModelValue(): string {
    const model = resolvedModel();
    return model.providerKey && model.modelId ? `${model.providerKey}:${model.modelId}` : "";
  }

  function setSelectedModel(value: string): void {
    const [providerKey, ...modelParts] = value.split(":");
    const modelId = modelParts.join(":");
    selectedProviderKey = providerKey ?? "";
    selectedModelId = modelId;
    onSelectedModelChange?.(selectedProviderKey, selectedModelId);
  }

  function cancel(): void {
    if (busy) return;
    validationError = null;
    onCancel?.();
    setOpen(false);
  }

  function submit(): void {
    if (!action || busy) return;
    if (!activeForm && action.fields) {
      for (const field of action.fields) {
        if (!field.required) continue;
        if (field.type === "checkbox") {
          if (!fieldBoolValue(field)) {
            validationError = `${field.label} is required.`;
            return;
          }
        } else if (field.type === "number") {
          if (fieldNumberValue(field) === null) {
            validationError = `${field.label} is required.`;
            return;
          }
        } else if (fieldStringValue(field).trim() === "") {
          validationError = `${field.label} is required.`;
          return;
        }
      }
    }
    validationError = null;
    const resolvedValues: Record<string, unknown> = { ...values };
    if (!activeForm && action.fields) {
      for (const field of action.fields) {
        if (resolvedValues[field.id] === undefined && field.defaultValue !== undefined) {
          resolvedValues[field.id] = field.defaultValue;
        }
      }
    }
    const model = resolvedModel();
    onSubmit?.({
      action,
      values: resolvedValues,
      selectedProviderKey: model.providerKey,
      selectedModelId: model.modelId
    });
  }

  function handleSubmit(event: SubmitEvent): void {
    event.preventDefault();
    submit();
  }

  // Poodle's NumberInput consumes Enter to commit its draft. Submit the
  // surrounding form after that control handles the key.
  function submitNumberOnEnter(form: HTMLFormElement) {
    const destroy = on(form, "keydown", (event) => {
      const target = event.target;
      if (event.key === "Enter" && event.defaultPrevented && !event.isComposing &&
          target instanceof HTMLInputElement && target.closest("[data-context-action-number]")) {
        form.requestSubmit();
      }
    });
    return { destroy };
  }

  function fieldValue(field: ContextActionInputField): unknown {
    const value = values[field.id];
    return value === undefined ? field.defaultValue : value;
  }

  function fieldStringValue(field: ContextActionInputField): string {
    return String(fieldValue(field) ?? "");
  }

  function fieldNumberValue(field: ContextActionInputField): number | null {
    const value = fieldValue(field);
    if (typeof value === "number") {
      return Number.isFinite(value) ? value : null;
    }
    if (typeof value === "string" && value.trim() !== "") {
      const parsed = Number(value);
      return Number.isFinite(parsed) ? parsed : null;
    }
    return null;
  }

  function fieldBoolValue(field: ContextActionInputField): boolean {
    return fieldValue(field) === true;
  }
</script>

<Dialog
  {open}
  width="lg"
  title={action?.name ?? "AI action"}
  description={action?.description ?? null}
  showCloseButton={!busy}
  dismissOnBackdrop={!busy}
  dismissOnEscape={!busy}
  onOpenChange={setOpen}
  onRequestClose={cancel}
>
  {#if action}
    <form
      id={formId}
      class="underlay-context-action-dialog"
      novalidate={!activeForm}
      onsubmit={handleSubmit}
      use:submitNumberOnEnter
    >
      {#if modelOptions.length > 0}
        <div class="underlay-context-action-dialog__field">
          <label for={modelId}>Model</label>
          <Select
            id={modelId}
            value={selectedModelValue()}
            options={groupedModelOptions}
            disabled={busy}
            onValueChange={setSelectedModel}
          />
        </div>
      {/if}

      {#if activeForm}
        {@const model = resolvedModel()}
        {@render activeForm({
          action,
          values,
          selectedProviderKey: model.providerKey,
          selectedModelId: model.modelId,
          setValue,
          submit,
          cancel
        })}
      {:else if action.fields && action.fields.length > 0}
        <div class="underlay-context-action-dialog__fields">
          {#each action.fields as field (field.id)}
            {@const fieldId = `${formId}-field-${encodeURIComponent(field.id)}`}
            {@const descriptionId = field.description ? `${fieldId}-description` : null}
            <div class="underlay-context-action-dialog__field" data-context-action-number={field.type === "number" ? "" : undefined}>
              {#if field.type !== "checkbox"}
                <label for={fieldId}>{field.label}</label>
              {/if}
              {#if field.description}
                <small id={descriptionId}>{field.description}</small>
              {/if}

              {#if field.type === "checkbox"}
                <Checkbox
                  id={fieldId}
                  describedBy={descriptionId}
                  checked={fieldBoolValue(field)}
                  label={field.placeholder || field.label}
                  disabled={busy}
                  onCheckedChange={(checked) => setValue(field.id, checked)}
                />
              {:else if field.type === "select"}
                <Select
                  id={fieldId}
                  describedBy={descriptionId}
                  value={fieldStringValue(field)}
                  placeholder={field.placeholder ?? null}
                  options={field.options ?? []}
                  required={field.required}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {:else if field.type === "number"}
                <NumberInput
                  id={fieldId}
                  describedBy={descriptionId}
                  value={fieldNumberValue(field)}
                  placeholder={field.placeholder ?? null}
                  required={field.required}
                  min={field.min ?? null}
                  max={field.max ?? null}
                  step={field.step ?? null}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {:else}
                <TextInput
                  id={fieldId}
                  describedBy={descriptionId}
                  type={field.type === "textarea" ? "multiline" : "text"}
                  rows={field.type === "textarea" ? field.rows ?? 6 : null}
                  value={fieldStringValue(field)}
                  placeholder={field.placeholder ?? null}
                  required={field.required}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {/if}
            </div>
          {/each}
        </div>
      {/if}

      {#if validationError || errorMessage}
        <p class="underlay-context-action-dialog__error" role="alert">{validationError ?? errorMessage}</p>
      {/if}
    </form>
  {/if}

  {#snippet actions()}
    <Button type="button" variant="ghost" disabled={busy} onClick={cancel}>
      {cancelLabel}
    </Button>
    <Button type="submit" form={formId} variant="primary" loading={busy} disabled={!action || busy}>
      {resolvedSubmitLabel}
    </Button>
  {/snippet}
</Dialog>

<style>
  .underlay-context-action-dialog {
    display: grid;
    gap: 1rem;
  }

  .underlay-context-action-dialog__fields {
    display: grid;
    gap: 1rem;
  }

  .underlay-context-action-dialog__field {
    display: grid;
    gap: 0.375rem;
    min-width: 0;
  }

  .underlay-context-action-dialog__field > label {
    color: var(--poodle-color-text-primary);
    font-weight: 600;
    line-height: 1.25;
  }

  .underlay-context-action-dialog__field > small {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.35;
  }

  .underlay-context-action-dialog__error {
    margin: 0;
    color: var(--poodle-color-danger-text, #b91c1c);
    font-size: 0.875rem;
    line-height: 1.4;
  }
</style>
