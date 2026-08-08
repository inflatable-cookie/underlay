<script lang="ts">
  import { Button as PoodleButton, Field as PoodleField, Stack, TextInput } from "@inflatable-cookie/poodle-svelte";

  interface Props {
    id: string;
    name: string;
    label: string;
    value?: string | null;
    displayValue?: string | null;
    displayName?: string;
    onDisplayChange?: (value: string) => void;
    previewUrl?: string | null;
    placeholder?: string;
    browseLabel?: string;
    disabled?: boolean;
    required?: boolean;
    error?: string | null;
    span?: number | "full" | null;
    onBrowse: () => void;
    onClear?: () => void;
  }

  let {
    id,
    name,
    label,
    value = null,
    displayValue = null,
    displayName = undefined,
    onDisplayChange = undefined,
    previewUrl = null,
    placeholder = "No file selected",
    browseLabel = "Choose file",
    disabled = false,
    required = false,
    error = null,
    span = null,
    onBrowse,
    onClear
  }: Props = $props();

  const resolvedDisplay = $derived(displayValue ?? value ?? "");
</script>

<PoodleField {id} {label} {error} {required} {span}>
  <input type="hidden" {name} value={value ?? ""} {required} />
  {#if previewUrl && value}
    <img class="underlay-entity-media-field__preview" src={previewUrl} alt="" />
  {/if}
  <TextInput
    id="{id}-display"
    name={displayName}
    value={resolvedDisplay}
    {disabled}
    {placeholder}
    onValueChange={onDisplayChange}
  />
  <Stack direction="row" gap="sm" align="center" wrap>
    <PoodleButton type="button" variant="secondary" onClick={onBrowse} {disabled}>
      {browseLabel}
    </PoodleButton>
    {#if value && onClear}
      <PoodleButton type="button" variant="ghost" onClick={onClear} {disabled}>
        Clear
      </PoodleButton>
    {/if}
  </Stack>
</PoodleField>

<style>
  .underlay-entity-media-field__preview {
    max-width: 8rem;
    max-height: 8rem;
    border-radius: 0.4rem;
    object-fit: cover;
    margin-bottom: 0.5rem;
  }
</style>
