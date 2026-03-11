<script lang="ts">
  import DataSkeleton from "../../src/components/DataSkeleton.svelte";
  import type {
    DataSkeletonDetailSection,
    DataSkeletonType
  } from "../../src/components/data-skeleton";

  interface Props {
    type?: DataSkeletonType;
    pattern?: string | null;
    count?: number;
    columns?: number;
    rows?: number;
    header?: boolean;
    sections?: DataSkeletonDetailSection[] | null;
    animate?: boolean;
    className?: string;
  }

  let {
    type = "list",
    pattern = null,
    count = 3,
    columns = 3,
    rows = 5,
    header = true,
    sections = null,
    animate = true,
    className = ""
  }: Props = $props();
</script>

<DataSkeleton
  {type}
  {pattern}
  {count}
  {columns}
  {rows}
  {header}
  {sections}
  {animate}
  class={className}
/>
