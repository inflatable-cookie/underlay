<script lang="ts">
  import EntityTrashPage from "../../src/templates/EntityTrashPage.svelte";

  interface TrashItem {
    id: string;
    title: string;
  }

  interface Props {
    loading?: boolean;
    error?: string | null;
    statusMessage?: string | null;
    items?: TrashItem[];
  }

  let {
    loading = false,
    error = null,
    statusMessage = null,
    items = []
  }: Props = $props();
</script>

<EntityTrashPage
  title="Trash"
  backHref="/chapters"
  backLabel="Back to chapters"
  {loading}
  {error}
  {statusMessage}
  {items}
  emptyTitle="Trash is empty"
  emptyMessage="Deleted chapters will appear here."
>
  {#snippet renderItem(item)}
    <div data-testid={`trash-item-${item.id}`}>{item.title}</div>
  {/snippet}
</EntityTrashPage>
