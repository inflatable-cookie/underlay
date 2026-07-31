<script lang="ts" generics="TItem, TData">
  import type { TemplateSurface } from "../template-types/primitives";

  interface Props {
    item: TItem | null;
    loader: (item: TItem | null) => Promise<TData>;
    render: (data: TData | null, item: TItem | null) => TemplateSurface;
    loadingMessage?: string;
    errorMessage?: string;
  }

  let { item, loader, render, loadingMessage = "Loading…", errorMessage = "Failed to load" }: Props = $props();

  let data = $state<TData | null>(null);
  let failed = $state(false);

  $effect(() => {
    let cancelled = false;
    failed = false;
    loader(item)
      .then((result) => {
        if (!cancelled) data = result;
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
{:else if data === null}
  <p>{loadingMessage}</p>
{:else}
  {@render render(data, item)}
{/if}
