<script lang="ts">
  import Pagination from "../../src/components/Pagination.svelte";
  import type { PaginationController } from "../../src/patterns/pagination-types";

  type Item = { id: string };

  interface Props {
    controller?: PaginationController<Item>;
    page?: number;
    limit?: number;
    total?: number;
    showLimitSelector?: boolean;
    limitOptions?: number[];
    showInfo?: boolean;
    compact?: boolean;
    variant?: "full" | "simple";
    className?: string;
    onPage?: (page: number) => void;
    onLimit?: (limit: number) => void;
    scrollTarget?: HTMLElement | string | false;
    scrollOffset?: number;
  }

  let {
    controller = undefined,
    page = 1,
    limit = 20,
    total = 0,
    showLimitSelector = false,
    limitOptions = [30, 50, 100],
    showInfo = true,
    compact = false,
    variant = "full",
    className = "",
    onPage = undefined,
    onLimit = undefined,
    scrollTarget = false,
    scrollOffset = 16
  }: Props = $props();
</script>

<Pagination
  {controller}
  {page}
  {limit}
  {total}
  {showLimitSelector}
  {limitOptions}
  {showInfo}
  {compact}
  {variant}
  {className}
  {onPage}
  {onLimit}
  {scrollTarget}
  {scrollOffset}
/>
