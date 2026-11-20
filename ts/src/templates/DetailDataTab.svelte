<script lang="ts" generics="TItem, TData">
  import type { Snippet } from "svelte";

  interface Props {
    item: TItem | null;
    loader: (item: TItem | null) => Promise<TData>;
    render: Snippet<[TData | null, TItem | null]>;
    loadingMessage?: string;
    errorMessage?: string;
  }

  let { item, loader, render, loadingMessage = "Loading…", errorMessage = "Failed to load" }: Props = $props();

  let data = $state<TData | null>(null);
  let loaded = $state(false);
  let failed = $state(false);

  $effect(() => {
    let cancelled = false;
    data = null;
    loaded = false;
    failed = false;
    loader(item)
      .then((result) => {
        if (!cancelled) {
          data = result;
          loaded = true;
        }
      })
      .catch(() => {
        if (!cancelled) failed = true;
      });
    return () => {
      cancelled = true;
    };
  });
</script>

{#if failed}
  <p>{errorMessage}</p>
{:else if !loaded}
  <p>{loadingMessage}</p>
{:else}
  {@render render(data, item)}
{/if}
