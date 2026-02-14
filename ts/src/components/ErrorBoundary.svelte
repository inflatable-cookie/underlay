<script lang="ts">
  import type { Snippet } from "svelte";
  import EmptyState from "./EmptyState.svelte";

  interface Props {
    /** Normal content to render */
    children: Snippet;
    /** Custom error UI — receives error and reset function */
    fallback?: Snippet<[Error, () => void]>;
    /** Error reporting callback */
    onError?: (error: Error) => void;
  }

  let {
    children,
    fallback = undefined,
    onError = undefined
  }: Props = $props();

  let currentError = $state<Error | null>(null);

  function handleError(error: unknown) {
    const err = error instanceof Error ? error : new Error(String(error));
    currentError = err;
    onError?.(err);
  }

  function reset() {
    currentError = null;
  }
</script>

{#if currentError}
  {#if fallback}
    {@render fallback(currentError, reset)}
  {:else}
    <EmptyState
      title="Something went wrong"
      description={currentError.message}
      actionLabel="Try again"
      onaction={reset}
    />
  {/if}
{:else}
  <svelte:boundary onerror={handleError}>
    {@render children()}
  </svelte:boundary>
{/if}
