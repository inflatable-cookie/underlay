<script lang="ts">
  import TextArea from "../../src/components/TextArea.svelte";

  interface Props {
    initialValue?: string;
    id?: string;
    className?: string;
    onInput?: (value: string) => void;
    onChange?: (value: string) => void;
  }

  let {
    initialValue = "",
    id = undefined,
    className = "",
    onInput = undefined,
    onChange = undefined
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

<TextArea bind:value {id} class={className} oninput={onInput} onchange={onChange} />
<p data-testid="textarea-value">{value}</p>
