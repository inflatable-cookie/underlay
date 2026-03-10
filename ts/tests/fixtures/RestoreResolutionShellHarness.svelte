<script lang="ts">
  import RestoreResolutionShell from "../../src/patterns/RestoreResolutionShell.svelte";

  interface Props {
    onClose?: () => void;
  }

  let { onClose = () => {} }: Props = $props();
</script>

<RestoreResolutionShell
  title="Resolve restore"
  description="Choose a new home for this item."
  {onClose}
  message="Preview ready"
>
  {#snippet meta()}
    <p>Original parent: Section A</p>
  {/snippet}

  {#snippet controls()}
    <label for="test-target">Target</label>
    <input id="test-target" />
  {/snippet}

  {#snippet body()}
    <div>Planner body</div>
  {/snippet}

  {#snippet actions()}
    <button type="button">Apply</button>
  {/snippet}
</RestoreResolutionShell>
