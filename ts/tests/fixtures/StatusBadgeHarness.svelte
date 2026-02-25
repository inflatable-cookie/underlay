<script lang="ts">
  import StatusBadge from "../../src/components/StatusBadge.svelte";

  interface Props {
    value: boolean;
    variant?: "success" | "danger";
  }

  let { value, variant = "success" }: Props = $props();
</script>

{#snippet trueIconSnippet()}
  <span data-testid="status-icon-true">T</span>
{/snippet}

{#snippet falseIconSnippet()}
  <span data-testid="status-icon-false">F</span>
{/snippet}

<StatusBadge
  {value}
  trueLabel="Enabled"
  falseLabel="Disabled"
  {variant}
  trueIcon={trueIconSnippet}
  falseIcon={falseIconSnippet}
/>
