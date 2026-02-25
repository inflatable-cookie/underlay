<script lang="ts">
  import TextButton from "../../src/components/TextButton.svelte";

  interface Props {
    type?: "button" | "submit" | "reset";
    variant?: "default" | "danger" | "success";
    href?: string | null;
    className?: string;
    disabled?: boolean;
    onclick?: (event: MouseEvent) => void;
    label?: string;
  }

  let {
    type = "button",
    variant = "default",
    href = null,
    className = "",
    disabled = false,
    onclick,
    label = "Click me"
  }: Props = $props();
</script>

<TextButton
  {type}
  {variant}
  {href}
  class={className}
  {disabled}
  {onclick}
>
  {#snippet children()}
    {label}
  {/snippet}
</TextButton>
