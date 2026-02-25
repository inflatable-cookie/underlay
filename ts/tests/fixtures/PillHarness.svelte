<script lang="ts">
  import Pill from "../../src/components/Pill.svelte";

  interface Props {
    accent?: string;
    className?: string;
    label?: string;
  }

  let {
    accent = undefined,
    className = "",
    label = "Status"
  }: Props = $props();
</script>

<Pill {accent} {className}>
  {#snippet children()}
    {label}
  {/snippet}
</Pill>
