<script lang="ts" generics="TItem, TData">
  import { Button } from "@inflatable-cookie/poodle-svelte";
  import { untrack, type Snippet } from "svelte";

  interface Props {
    item: TItem | null;
    loader: (item: TItem | null) => Promise<TData>;
    render: Snippet<[TData | null, TItem | null]>;
    reloadKey?: string | number | null;
    loadingMessage?: string;
    errorMessage?: string;
  }

  let {
    item,
    loader,
    render,
    reloadKey,
    loadingMessage = "Loading…",
    errorMessage = "Failed to load"
  }: Props = $props();

  let result = $state<
    | { status: "loading" }
    | { status: "loaded"; data: TData }
    | { status: "failed"; message: string }
  >({ status: "loading" });
  let retryCount = $state(0);
  // A parent can replace its whole props object while keeping the same item and key.
  const loadItem = $derived(item);
  const loadKey = $derived(reloadKey);

  $effect(() => {
    const input = loadItem;
    void loadKey;
    void retryCount;
    let cancelled = false;
    result = { status: "loading" };

    async function load() {
      try {
        const data = await untrack(() => loader(input));
        if (!cancelled) result = { status: "loaded", data };
      } catch (error) {
        const message = error instanceof Error ? error.message : typeof error === "string" ? error : "";
        if (!cancelled) result = { status: "failed", message };
      }
    }

    void load();
    return () => {
      cancelled = true;
    };
  });
</script>

{#if result.status === "failed"}
  <p role="alert">{result.message || errorMessage}</p>
  <Button type="button" variant="ghost" size="sm" onClick={() => retryCount++}>Retry</Button>
{:else if result.status === "loading"}
  <p role="status" aria-live="polite">{loadingMessage}</p>
{:else}
  {@render render(result.data, item)}
{/if}
