<script lang="ts">
  import DateTimeInput from "../../src/components/DateTimeInput.svelte";

  interface Props {
    initialValue?: string;
    defaultToNow?: boolean;
    name?: string;
    onChange?: (value: string) => void;
  }

  let {
    initialValue = "",
    defaultToNow = false,
    name = undefined,
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

<DateTimeInput bind:value {defaultToNow} {name} onchange={onChange} />
<p data-testid="datetime-input-value">{value}</p>
