<script lang="ts">
  import RangeSlider from "../../src/components/RangeSlider.svelte";
  import type { RangeSliderOption } from "../../src/components/RangeSlider.svelte";

  interface Props {
    options?: Array<string | RangeSliderOption>;
    initialValue?: string;
    name?: string;
    required?: boolean;
    disabled?: boolean;
    showValue?: boolean;
    onInput?: (value: string) => void;
    onChange?: (value: string) => void;
  }

  let {
    options = [
      { value: "low", label: "Low", tone: "default" },
      { value: "med", label: "Medium", tone: "warning" },
      { value: "high", label: "High", tone: "danger" }
    ],
    initialValue = "med",
    name = undefined,
    required = false,
    disabled = false,
    showValue = true,
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

<RangeSlider
  {options}
  bind:value
  {name}
  {required}
  {disabled}
  {showValue}
  oninput={onInput}
  onchange={onChange}
/>

<p data-testid="range-slider-value">{value}</p>
