<script lang="ts">
  import Badge from "../../src/components/Badge.svelte";

  interface Props {
    variant?: "default" | "success" | "warning" | "danger" | "info" | "muted";
    size?: "sm" | "md" | "lg";
    pill?: boolean;
    icon?: string;
    className?: string;
    label?: string;
  }

  let {
    variant = "default",
    size = "md",
    pill = false,
    icon = undefined,
    className = "",
    label = "Badge"
  }: Props = $props();
</script>

<Badge {variant} {size} {pill} {icon} {className} data-testid="badge-node">
  {#snippet children()}
    {label}
  {/snippet}
</Badge>
