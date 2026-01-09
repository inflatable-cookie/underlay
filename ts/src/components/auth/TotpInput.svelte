<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { createStableId } from "../../patterns/dom";

  import Field from "../Field.svelte";
  import TextInput from "../TextInput.svelte";

  const dispatch = createEventDispatcher<{ input: string; change: string }>();

  export let value: string = "";

  export let name: string = "code";
  export let label: string = "Authenticator code";
  export let hint: string | undefined = "6-digit code";
  export let error: string | undefined = undefined;
  export let disabled: boolean = false;

  export let className: string = "";

  const inputId = createStableId("underlay-totp");

  function handleInput(e: CustomEvent<string>) {
    dispatch("input", e.detail);
  }

  function handleChange(e: CustomEvent<string>) {
    dispatch("change", e.detail);
  }
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
    maxlength="8"
    aria-invalid={error ? "true" : "false"}
    on:input={handleInput}
    on:change={handleChange}
  />
</Field>
