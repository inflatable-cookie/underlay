<script lang="ts">
  import { getAuthConfig, useAuthenticatedData } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import { default as EntityTrashPage } from "./EntityTrashPage.svelte";
  import { default as SystemMediaTrashListCard } from "./SystemMediaTrashListCard.svelte";
  import type {
    SystemMediaTrashAction,
    SystemMediaTrashItem,
    SystemMediaTrashListLoader
  } from "./template.types";

  interface Props {
    title?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: SystemMediaTrashListLoader;
    restoreAction: SystemMediaTrashAction;
    purgeAction: SystemMediaTrashAction;
    getMediaHref?: (media: SystemMediaTrashItem) => string | null;
    onMediaClick?: (media: SystemMediaTrashItem) => void;
    restoreSuccessMessage?: string;
    purgeSuccessMessage?: string;
  }

  let {
    title = "Media Trash",
    backHref = "/media",
    backLabel = "Back to media",
    dataLoader,
    restoreAction,
    purgeAction,
    getMediaHref = defaultMediaHref,
    onMediaClick,
    restoreSuccessMessage = "Media restored",
    purgeSuccessMessage = "Media permanently deleted"
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  const pageData = useAuthenticatedData(
    async (fetch, token) => {
      const media = await dataLoader(fetch, token);
      return { media };
    },
    {
      defaultValue: {
        media: {
          data: [],
          total: 0,
          hasMore: false
        }
      }
    }
  );

  async function runAction(
    media: SystemMediaTrashItem,
    action: SystemMediaTrashAction,
    successMessage: string,
    failureMessage: string
  ): Promise<void> {
    const token = authConfig?.getToken?.() ?? null;
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      await action(media, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
      await pageData.refetch();
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  function defaultMediaHref(media: SystemMediaTrashItem): string {
    return `/media/${media.id}`;
  }
</script>

{#snippet renderItem(item)}
  <SystemMediaTrashListCard
    media={item}
    href={onMediaClick ? null : getMediaHref(item)}
    onClick={onMediaClick ? () => onMediaClick(item) : undefined}
    onRestore={(media) => runAction(media, restoreAction, restoreSuccessMessage, "Failed to restore media")}
    onPurge={(media) => runAction(media, purgeAction, purgeSuccessMessage, "Failed to delete media")}
  />
{/snippet}

<EntityTrashPage
  {title}
  {backHref}
  {backLabel}
  loading={pageData.loading}
  loadingMessage="Loading trash..."
  error={pageData.error}
  statusMessage="Items in trash can be restored or permanently deleted. Permanently deleted items cannot be recovered."
  statusTone="warning"
  items={pageData.data?.media.data ?? []}
  renderItem={renderItem}
  emptyTitle="Trash is empty"
  emptyMessage="Deleted media items will appear here."
/>
