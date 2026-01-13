<script lang="ts">
  import type { Snippet } from "svelte";

  import { createStableId } from "../../patterns/dom";

  import Field from "../Field.svelte";
  import TextInput from "../TextInput.svelte";

  interface Props {
    value?: string;
    name?: string;
    label?: string;
    hint?: string;
    error?: string;
    disabled?: boolean;
    className?: string;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    name = "code",
    label = "Authenticator code",
    hint = "6-digit code",
    error = undefined,
    disabled = false,
    className = "",
    oninput,
    onchange,
  }: Props = $props();

  const inputId = createStableId("underlay-totp");
</script>

<Field {label} forId={inputId} {hint} {error}>
  <TextInput
    id={inputId}
    class={className}
    bind:value
    {disabled}
    {name}
    type="text"
    autocomplete="one-time-code"
    inputmode="numeric"
    pattern="[0-9]*"
    maxlength={8}
    aria-invalid={error ? "true" : "false"}
    {oninput}
    {onchange}
  />
</Field>
