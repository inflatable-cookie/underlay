<script lang="ts">
  import Switch from "../../src/components/Switch.svelte";

  interface Props {
    name?: string;
    initialChecked?: boolean;
    leftLabel?: string;
    rightLabel?: string;
    disabled?: boolean;
    variant?: "default" | "danger-off";
    leftVariant?: "default" | "primary" | "success" | "warning" | "danger";
    rightVariant?: "default" | "primary" | "success" | "warning" | "danger";
  }

  let {
    name = undefined,
    initialChecked = false,
    leftLabel = "Off",
    rightLabel = "On",
    disabled = false,
    variant = "default",
    leftVariant = "default",
    rightVariant = "primary"
  }: Props = $props();

  let checked = $state(false);
  let initialized = $state(false);
  $effect(() => {
    if (!initialized) {
      checked = initialChecked;
      initialized = true;
    }
  });
</script>

<Switch
  {name}
  bind:checked
  {initialChecked}
  {leftLabel}
  {rightLabel}
  {disabled}
  {variant}
  {leftVariant}
  {rightVariant}
/>

<p data-testid="switch-checked">{checked ? "true" : "false"}</p>
