<script lang="ts">
  import { setContext } from "svelte";
  import MediaActionsMenu from "../../src/components/MediaActionsMenu.svelte";
  import ToastHost from "../../src/components/ToastHost.svelte";
  import {
    UNDERLAY_TOASTS_CONTEXT_KEY,
    createToastStore
  } from "../../src/patterns/toasts";
  import { MediaKind, MediaVisibility } from "../../src/patterns";

  const toastStore = createToastStore();
  setContext(UNDERLAY_TOASTS_CONTEXT_KEY, toastStore);

  const media = {
    id: "media-actions-1",
    kind: MediaKind.Image,
    visibility: MediaVisibility.Public,
    title: "Homepage Hero",
    originalFilename: "hero.jpg",
    currentVersionId: "version-hero",
    createdAt: "2026-03-20T09:00:00Z",
    updatedAt: "2026-03-20T09:00:00Z",
    deletedAt: null,
    byteSize: 248000,
    mimeType: "image/jpeg",
    thumbnailUrl: "https://picsum.photos/seed/hero/320/200"
  };

  async function wait(ms: number) {
    return new Promise<void>((resolve) => setTimeout(resolve, ms));
  }
</script>

<div class="media-actions-menu-demo">
  <MediaActionsMenu
    {media}
    onEditRequest={() => toastStore.push({ variant: "info", message: "Edit requested" })}
    navigateToReplace={() => toastStore.push({ variant: "info", message: "Replace requested" })}
    softDelete={async () => {
      await wait(150);
    }}
    restore={async () => {
      await wait(150);
    }}
    purge={async () => {
      await wait(150);
    }}
  />
  <ToastHost store={toastStore} autoDismissMs={2500} />
</div>

<style>
  .media-actions-menu-demo {
    min-height: 10rem;
    min-width: 18rem;
  }
</style>
