<script lang="ts">
  import FormActions from "../../src/components/FormActions.svelte";

  interface Props {
    align?: "start" | "end";
    withDangerSlot?: boolean;
    dangerItems?: Array<{ label: string; onSelect: () => void; destructive?: boolean }>;
  }

  let {
    align = "start",
    withDangerSlot = false,
    dangerItems = []
  }: Props = $props();
</script>

{#snippet childrenSnippet()}
  <button type="button" data-testid="primary-action">Save</button>
{/snippet}

{#snippet dangerSnippet()}
  <button type="button" data-testid="danger-action">Delete</button>
{/snippet}

<FormActions {align} danger={withDangerSlot ? dangerSnippet : undefined} {dangerItems}>
  {#snippet children()}
    {@render childrenSnippet()}
  {/snippet}
</FormActions>
