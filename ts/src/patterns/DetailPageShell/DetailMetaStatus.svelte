<script lang="ts">
  import { Pill } from "@poodle/svelte-primitives";

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

  const tone = $derived(
    value
      ? variant === "danger"
        ? "danger"
        : "success"
      : "neutral"
  );

  const label = $derived(value ? trueLabel : falseLabel);
</script>

<Pill tone={tone} appearance="badge" size="sm">
  {label}
</Pill>
