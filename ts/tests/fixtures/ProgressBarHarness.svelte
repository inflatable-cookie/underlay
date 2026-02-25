<script lang="ts">
  import ProgressBar from "../../src/components/ProgressBar.svelte";

  interface Props {
    value?: number;
    max?: number;
    variant?: "default" | "success" | "warning" | "danger" | "info";
    size?: "sm" | "md" | "lg";
    showLabel?: boolean;
    animated?: boolean;
    className?: string;
    withCustomLabel?: boolean;
    withFormatLabel?: boolean;
  }

  let {
    value = 50,
    max = 100,
    variant = "default",
    size = "md",
    showLabel = false,
    animated = false,
    className = "",
    withCustomLabel = false,
    withFormatLabel = false
  }: Props = $props();

  let formatLabel = $derived(
    withFormatLabel
      ? (value: number, max: number, percentage: number) =>
          `${value}/${max} (${Math.round(percentage)}%)`
      : undefined
  );
</script>

{#snippet labelSnippet({ value, max, percentage })}
  <span data-testid="progress-custom-label">{value} of {max} ({Math.round(percentage)}%)</span>
{/snippet}

<ProgressBar
  {value}
  {max}
  {variant}
  {size}
  {showLabel}
  {animated}
  {className}
  {formatLabel}
  label={withCustomLabel ? labelSnippet : undefined}
/>
