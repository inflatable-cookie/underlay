<script lang="ts">
  import Button from "../../src/components/Button.svelte";

  interface Props {
    variant?: "primary" | "secondary" | "subtle" | "danger" | "danger-subtle";
    size?: "sm" | "md" | "lg" | "icon" | "icon-sm";
    type?: "button" | "submit" | "reset";
    pill?: boolean;
    disabled?: boolean;
    className?: string;
    label?: string;
    onclick?: (event: MouseEvent) => void;
  }

  let {
    variant = "primary",
    size = "md",
    type = "button",
    pill = true,
    disabled = false,
    className = "",
    label = "Press",
    onclick
  }: Props = $props();
</script>

<Button {variant} {size} {type} {pill} {disabled} class={className} {onclick}>
  {#snippet children()}
    {label}
  {/snippet}
</Button>
