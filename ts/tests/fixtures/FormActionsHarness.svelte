<script lang="ts">
  import FormActions from "../../src/components/FormActions.svelte";
  const dangerItems = [{ label: "Delete", onSelect: () => {} }];
</script>

{#snippet childrenSnippet()}
  <button type="button" data-testid="primary-action">Save</button>
{/snippet}

<FormActions align="end" {dangerItems} children={childrenSnippet}>
  {#snippet danger()}
    <button type="button" data-testid="danger-action">Delete</button>
  {/snippet}
</FormActions>
