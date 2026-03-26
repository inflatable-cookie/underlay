<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
  import ErrorBoundary from "../../src/components/ErrorBoundary.svelte";
  import ErrorThrower from "./ErrorThrower.svelte";

  let explode = $state(false);
</script>

<ErrorBoundary>
  {#snippet children()}
    <div class="error-boundary-demo">
      <p>Click the button to trigger the retained shared error boundary.</p>
      <Button variant="secondary" onclick={() => (explode = true)}>Trigger render error</Button>
      <ErrorThrower {explode} />
    </div>
  {/snippet}
</ErrorBoundary>

<style>
  .error-boundary-demo {
    display: grid;
    gap: 0.75rem;
    min-width: 18rem;
  }

  .error-boundary-demo p {
    margin: 0;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
