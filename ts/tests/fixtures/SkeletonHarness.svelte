<script lang="ts">
  import Skeleton from "../../src/components/Skeleton.svelte";
  import type { SkeletonVariant } from "../../src/components/Skeleton.svelte";

  interface Props {
    variant?: SkeletonVariant;
    width?: string;
    height?: string;
    radius?: string;
    lines?: number;
    animate?: boolean;
    className?: string;
    withChildren?: boolean;
  }

  let {
    variant = "text",
    width = undefined,
    height = undefined,
    radius = undefined,
    lines = 1,
    animate = true,
    className = "",
    withChildren = false
  }: Props = $props();
</script>

{#snippet cardChildren()}
  <div data-testid="skeleton-card-child-a">A</div>
  <div data-testid="skeleton-card-child-b">B</div>
{/snippet}

<Skeleton
  {variant}
  {width}
  {height}
  {radius}
  {lines}
  {animate}
  class={className}
  children={withChildren ? cardChildren : undefined}
/>
