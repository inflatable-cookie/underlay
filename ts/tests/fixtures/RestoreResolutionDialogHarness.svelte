<script lang="ts">
  import RestoreResolutionDialog from "../../src/patterns/RestoreResolutionDialog.svelte";

  interface Props {
    initialOpen?: boolean;
  }

  let { initialOpen = true }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

<p data-testid="restore-resolution-dialog-open-state">{open ? "open" : "closed"}</p>

<RestoreResolutionDialog bind:open>
  <div data-testid="restore-resolution-dialog-body">Restore planner content</div>
</RestoreResolutionDialog>
