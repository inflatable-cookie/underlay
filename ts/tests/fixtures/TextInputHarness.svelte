<script lang="ts">
  import TextInput from "../../src/components/TextInput.svelte";
  import type { ValidationResult } from "../../src/components/TextInput.svelte";

  interface Props {
    initialValue?: string;
    type?: string;
    search?: boolean;
    debounce?: number | undefined;
    prefix?: string | undefined;
    withSuffix?: boolean;
    onInput?: (value: string) => void;
    onChange?: (value: string) => void;
    validate?: ((value: string, context?: unknown) => Promise<ValidationResult>) | undefined;
    validationContext?: unknown;
    showValidationStatus?: boolean;
  }

  let {
    initialValue = "",
    type = "text",
    search = false,
    debounce = undefined,
    prefix = undefined,
    withSuffix = false,
    onInput = undefined,
    onChange = undefined,
    validate = undefined,
    validationContext = undefined,
    showValidationStatus = undefined
  }: Props = $props();

  let value = $state("");
  let initialized = $state(false);
  $effect(() => {
    if (!initialized) {
      value = initialValue;
      initialized = true;
    }
  });
</script>

{#snippet suffixSnippet()}
  <span data-testid="text-input-suffix">SFX</span>
{/snippet}

<TextInput
  bind:value
  {type}
  {search}
  {debounce}
  {prefix}
  oninput={onInput}
  onchange={onChange}
  {validate}
  {validationContext}
  {showValidationStatus}
  suffix={withSuffix ? suffixSnippet : undefined}
/>

<p data-testid="text-input-value">{value}</p>
