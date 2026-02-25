<script lang="ts">
  import NumberInput from "../../src/components/NumberInput.svelte";

  interface Props {
    initialValue?: string;
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
  }

  let {
    initialValue = "",
    min = undefined,
    max = undefined,
    step = 1,
    disabled = false
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

<NumberInput bind:value {min} {max} {step} {disabled} />
<p data-testid="number-value">{value}</p>
