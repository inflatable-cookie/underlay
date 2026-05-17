<script lang="ts">
  import { Button, Checkbox, Dialog, NumberInput, Select, TextInput } from "@poodle/svelte";
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
    selectedModelAlias?: string;
    runState?: ContextActionRunState;
    errorMessage?: string | null;
    submitLabel?: string;
    cancelLabel?: string;
    form?: ContextActionDialogForm | null;
    onOpenChange?: (open: boolean) => void;
    onSubmit?: (detail: ContextActionSubmitDetail) => void;
    onCancel?: () => void;
    onValueChange?: (fieldId: string, value: unknown) => void;
    onSelectedModelChange?: (alias: string) => void;
  }

  let {
    open = $bindable(false),
    action = null,
    values = $bindable<Record<string, unknown>>({}),
    selectedModelAlias = $bindable(""),
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

  const busy = $derived(runState === "validating" || runState === "running");
  const activeForm = $derived(form ?? action?.form ?? null);
  const modelOptions = $derived(action?.modelOptions ?? []);
  const resolvedSubmitLabel = $derived(action?.submitLabel ?? submitLabel);

  function setOpen(nextOpen: boolean): void {
    open = nextOpen;
    onOpenChange?.(nextOpen);
  }

  function setValue(fieldId: string, value: unknown): void {
    values = { ...values, [fieldId]: value };
    onValueChange?.(fieldId, value);
  }

  function setSelectedModelAlias(alias: string): void {
    selectedModelAlias = alias;
    onSelectedModelChange?.(alias);
  }

  function cancel(): void {
    onCancel?.();
    setOpen(false);
  }

  function submit(): void {
    if (!action) return;
    onSubmit?.({
      action,
      values,
      selectedModelAlias: selectedModelAlias || undefined
    });
  }

  function fieldStringValue(field: ContextActionInputField): string {
    const value = values[field.id] ?? field.defaultValue ?? "";
    return typeof value === "string" ? value : String(value ?? "");
  }

  function fieldNumberValue(field: ContextActionInputField): number | string | null {
    const value = values[field.id] ?? field.defaultValue ?? null;
    if (typeof value === "number" || typeof value === "string") return value;
    return null;
  }

  function fieldBoolValue(field: ContextActionInputField): boolean {
    const value = values[field.id] ?? field.defaultValue ?? false;
    return value === true;
  }
</script>

<Dialog
  {open}
  width="xl"
  title={action?.name ?? "AI action"}
  description={action?.description ?? null}
  showCloseButton={true}
  dismissOnBackdrop={!busy}
  dismissOnEscape={!busy}
  onOpenChange={setOpen}
  onRequestClose={cancel}
>
  {#if action}
    <div class="underlay-context-action-dialog">
      {#if modelOptions.length > 0}
        <label class="underlay-context-action-dialog__field">
          <span>Model</span>
          <Select
            value={selectedModelAlias || action.defaultModelAlias || modelOptions[0]?.alias || ""}
            options={modelOptions.map((option) => ({
              value: option.alias,
              label: option.label,
              disabled: option.disabled
            }))}
            disabled={busy}
            onValueChange={setSelectedModelAlias}
          />
        </label>
      {/if}

      {#if activeForm}
        {@render activeForm({
          action,
          values,
          selectedModelAlias: selectedModelAlias || undefined,
          setValue,
          submit,
          cancel
        })}
      {:else if action.fields && action.fields.length > 0}
        <div class="underlay-context-action-dialog__fields">
          {#each action.fields as field (field.id)}
            <label class="underlay-context-action-dialog__field">
              <span>{field.label}</span>
              {#if field.description}
                <small>{field.description}</small>
              {/if}

              {#if field.type === "textarea"}
                <TextInput
                  type="multiline"
                  rows={field.rows ?? 6}
                  value={fieldStringValue(field)}
                  placeholder={field.placeholder ?? null}
                  required={field.required}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {:else if field.type === "select"}
                <Select
                  value={fieldStringValue(field)}
                  placeholder={field.placeholder ?? null}
                  options={(field.options ?? []).map((option) => ({
                    value: option.value,
                    label: option.label,
                    disabled: option.disabled
                  }))}
                  required={field.required}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {:else if field.type === "number"}
                <NumberInput
                  value={fieldNumberValue(field)}
                  placeholder={field.placeholder ?? null}
                  required={field.required}
                  min={field.min ?? null}
                  max={field.max ?? null}
                  step={field.step ?? null}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {:else if field.type === "checkbox"}
                <Checkbox
                  checked={fieldBoolValue(field)}
                  label={field.placeholder ?? field.label}
                  disabled={busy}
                  onCheckedChange={(checked) => setValue(field.id, checked)}
                />
              {:else}
                <TextInput
                  value={fieldStringValue(field)}
                  placeholder={field.placeholder ?? null}
                  required={field.required}
                  disabled={busy}
                  onValueChange={(value) => setValue(field.id, value)}
                />
              {/if}
            </label>
          {/each}
        </div>
      {/if}

      {#if errorMessage}
        <p class="underlay-context-action-dialog__error" role="alert">{errorMessage}</p>
      {/if}
    </div>
  {/if}

  {#snippet actions()}
    <Button type="button" variant="ghost" disabled={busy} onClick={cancel}>
      {cancelLabel}
    </Button>
    <Button type="button" variant="primary" loading={busy} disabled={!action} onClick={submit}>
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

  .underlay-context-action-dialog__field > span {
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
