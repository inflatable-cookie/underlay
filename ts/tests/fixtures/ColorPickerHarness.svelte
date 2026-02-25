<script lang="ts">
  import ColorPicker from "../../src/components/ColorPicker.svelte";

  interface Props {
    initialValue?: string;
    showSwatch?: boolean;
    presets?: string[];
    size?: "sm" | "md" | "lg";
    disabled?: boolean;
    onInput?: (value: string) => void;
    onChange?: (value: string) => void;
  }

  let {
    initialValue = "#112233",
    showSwatch = true,
    presets = ["#ff0000", "#00ff00", "#0000ff"],
    size = "md",
    disabled = false,
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

<ColorPicker bind:value {showSwatch} {presets} {size} {disabled} oninput={onInput} onchange={onChange} />
<p data-testid="color-picker-value">{value}</p>
