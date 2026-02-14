<script lang="ts">
  import type { Snippet } from "svelte";
  import StatusBadge from "../../components/StatusBadge.svelte";

  interface Props {
    /** Current status value */
    value: boolean;
    /** Label for true state */
    trueLabel?: string;
    /** Label for false state */
    falseLabel?: string;
    /** Badge variant */
    variant?: "success" | "danger";
  }

  let {
    value,
    trueLabel = "Active",
    falseLabel = "Inactive",
    variant = "success"
  }: Props = $props();
</script>

<StatusBadge {value} {trueLabel} {falseLabel} {variant} />
