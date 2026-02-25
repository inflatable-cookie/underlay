<script lang="ts">
  import Select from "../../src/components/Select.svelte";

  type SelectItem = {
    value: string;
    label: string;
    disabled?: boolean;
  };

  interface Props {
    initialValue?: string;
    items?: SelectItem[] | null;
    placeholder?: string;
    clearable?: boolean;
    defaultValue?: string;
    disabled?: boolean;
    className?: string;
    onChange?: (value: string) => void;
    onInput?: (value: string) => void;
    withNativeChildren?: boolean;
  }

  let {
    initialValue = "",
    items = [
      { value: "a", label: "Alpha" },
      { value: "b", label: "Beta" }
    ],
    placeholder = "Choose",
    clearable = false,
    defaultValue = "",
    disabled = false,
    className = "",
    onChange = undefined,
    onInput = undefined,
    withNativeChildren = false
  }: Props = $props();

  let value = $state("");
  $effect(() => {
    value = initialValue;
  });
</script>

<p data-testid="select-value">{value}</p>

<Select
  bind:value
  {items}
  {placeholder}
  {clearable}
  {defaultValue}
  {disabled}
  class={className}
  onchange={onChange}
  oninput={onInput}
>
  {#if withNativeChildren}
    <option value="">Choose</option>
    <option value="x">X-Ray</option>
    <option value="y">Yankee</option>
  {/if}
</Select>
