<script lang="ts">
  import DurationInput from "../../src/components/DurationInput.svelte";

  interface Props {
    initialValue?: string;
    name?: string;
    min?: number;
    max?: number;
    unit?: "seconds" | "minutes";
    disabled?: boolean;
  }

  let {
    initialValue = "",
    name = undefined,
    min = undefined,
    max = undefined,
    unit = "seconds",
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

<DurationInput bind:value {name} {min} {max} {unit} {disabled} />
<p data-testid="duration-value">{value}</p>
