<script lang="ts">
  import DateRange from "../../src/components/DateRange.svelte";
  import type { DateRangeInput } from "../../src/components/DateRange.svelte";

  interface Props {
    startDate?: DateRangeInput;
    endDate?: DateRangeInput;
    locale?: string;
    style?: "adaptive" | "full";
    emptyText?: string;
    className?: string;
  }

  let {
    startDate = null,
    endDate = null,
    locale = "en-GB",
    style = "adaptive",
    emptyText = "—",
    className = ""
  }: Props = $props();
</script>

<DateRange {startDate} {endDate} {locale} {style} {emptyText} class={className} />
