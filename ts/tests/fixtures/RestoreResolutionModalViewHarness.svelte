<script lang="ts">
  import RestoreResolutionModalView from "../../src/patterns/RestoreResolutionModalView.svelte";

  interface Props {
    initialOpen?: boolean;
    mode?: "ready" | "blocked" | null;
  }

  let {
    initialOpen = true,
    mode = "ready",
  }: Props = $props();

  let open = $state(false);

  $effect(() => {
    open = initialOpen;
  });
</script>

<RestoreResolutionModalView bind:open {mode}>
  {#snippet ready()}
    <div data-testid="restore-ready-content">Ready content</div>
  {/snippet}

  {#snippet blocked()}
    <div data-testid="restore-blocked-content">Blocked content</div>
  {/snippet}
</RestoreResolutionModalView>
