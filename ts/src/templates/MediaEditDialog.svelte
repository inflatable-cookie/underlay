<script lang="ts">
  import { Button, Field as PoodleField, FormActions as PoodleFormActions, FormDialog, Select, TextInput } from "@poodle/svelte";

  interface VisibilityOption {
    value: string;
    label: string;
  }

  interface Props {
    open: boolean;
    title?: string;
    subtitle?: string;
    error: string | null;
    submitting: boolean;
    titleValue: string;
    filenameValue: string;
    visibilityValue: string;
    visibilityOptions: VisibilityOption[];
    titlePlaceholder?: string;
    filenamePlaceholder?: string;
    filenameHint?: string;
    saveLabel?: string;
    savingLabel?: string;
    cancelLabel?: string;
    onCancel: () => void;
    onSubmit: (event: SubmitEvent) => void;
  }

  let {
    open = $bindable(),
    title = "Edit Media",
    subtitle,
    error,
    submitting,
    titleValue = $bindable(),
    filenameValue = $bindable(),
    visibilityValue = $bindable(),
    visibilityOptions,
    titlePlaceholder = "Enter a title for this media",
    filenamePlaceholder = "e.g. document.pdf",
    filenameHint = "The filename shown when downloading",
    saveLabel = "Save",
    savingLabel = "Saving...",
    cancelLabel = "Cancel",
    onCancel,
    onSubmit
  }: Props = $props();
</script>

<FormDialog
  bind:open
  {title}
  {subtitle}
  {error}
  {submitting}
  showDefaultActions={false}
  onCancel={onCancel}
>
  {#snippet body(submitting)}
    <form id="media-edit-form" onsubmit={(event) => onSubmit(event)}>
      <div class="underlay-media-edit-dialog__fields">
        <PoodleField id="media-edit-title" label="Title">
          <TextInput
            id="media-edit-title-input"
            name="title"
            value={titleValue}
            placeholder={titlePlaceholder}
            disabled={submitting}
            onValueChange={(nextValue) => {
              titleValue = nextValue;
            }}
          />
        </PoodleField>

        <PoodleField id="media-edit-filename" label="Filename" hint={filenameHint}>
          <TextInput
            id="media-edit-filename-input"
            name="filename"
            value={filenameValue}
            placeholder={filenamePlaceholder}
            disabled={submitting}
            onValueChange={(nextValue) => {
              filenameValue = nextValue;
            }}
          />
        </PoodleField>

        <PoodleField id="media-edit-visibility" label="Visibility">
          <Select
            id="media-edit-visibility-input"
            name="visibility"
            bind:value={visibilityValue}
            options={visibilityOptions}
            disabled={submitting}
          />
        </PoodleField>
      </div>
    </form>
  {/snippet}

  {#snippet actions(submitting)}
    <PoodleFormActions align="end" showTopBorder>
      <Button type="button" variant="ghost" onClick={onCancel} disabled={submitting}>
        {cancelLabel}
      </Button>
      <Button type="submit" variant="primary" disabled={submitting} form="media-edit-form">
        {submitting ? savingLabel : saveLabel}
      </Button>
    </PoodleFormActions>
  {/snippet}
</FormDialog>

<style>
  .underlay-media-edit-dialog__fields {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
</style>
