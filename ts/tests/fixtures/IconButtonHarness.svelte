<script lang="ts">
  import IconButton from "../../src/components/IconButton.svelte";

  interface Props {
    label: string;
    type?: "button" | "submit" | "reset";
    disabled?: boolean;
    sizeRem?: number;
    variant?: "neutral" | "danger";
    className?: string;
    onclick?: (event: MouseEvent) => void;
    withIcon?: boolean;
  }

  let {
    label,
    type = "button",
    disabled = false,
    sizeRem = 1.6,
    variant = "neutral",
    className = "",
    onclick,
    withIcon = true
  }: Props = $props();
</script>

<IconButton
  {label}
  {type}
  {disabled}
  {sizeRem}
  {variant}
  class={className}
  {onclick}
>
  {#if withIcon}
    <span data-testid="icon-content">X</span>
  {/if}
</IconButton>
