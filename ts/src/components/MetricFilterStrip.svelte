<script lang="ts">
  import Badge from "./Badge.svelte";
  import SegmentedControl from "./SegmentedControl.svelte";

  type BadgeVariant = "default" | "success" | "warning" | "danger" | "info" | "muted";

  type MetricFilterOption = {
    label: string;
    value: string;
    disabled?: boolean;
    title?: string;
  };

  type MetricFilterBadge = {
    label: string;
    variant?: BadgeVariant;
    title?: string;
  };

  interface Props {
    label: string;
    value: string;
    options: MetricFilterOption[];
    onchange: (value: string) => void;
    ariaLabel?: string;
    badges?: MetricFilterBadge[];
    class?: string;
  }

  let {
    label,
    value,
    options,
    onchange,
    ariaLabel = label,
    badges = [],
    class: className = ""
  }: Props = $props();
</script>

<div class={`underlay-metric-filter-strip ${className}`}>
  <span class="underlay-metric-filter-strip__label">{label}</span>
  {#each badges as badge, index (`${badge.label}-${index}`)}
    <Badge variant={badge.variant ?? "muted"} size="sm" title={badge.title}>
      {badge.label}
    </Badge>
  {/each}
  <SegmentedControl
    ariaLabel={ariaLabel}
    size="xs"
    equalWidth={false}
    {value}
    {options}
    {onchange}
  />
</div>

<style>
  .underlay-metric-filter-strip {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.45rem;
  }

  .underlay-metric-filter-strip__label {
    margin-right: 0.2rem;
    font-size: 0.76rem;
    color: var(--underlay-color-text-muted, #94a3b8);
    font-weight: 600;
    white-space: nowrap;
  }
</style>
