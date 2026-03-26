<script lang="ts">
  import CopyActionsMenu from "../../src/patterns/CopyActionsMenu.svelte";
  import ToastHost from "../../src/components/ToastHost.svelte";
  import { createToastStore } from "../../src/patterns/toasts";

  const toastStore = createToastStore();

  const copies = [
    {
      label: "Copy public URL",
      text: "https://example.test/media/intro-video",
      successMessage: "Public URL copied"
    },
    {
      label: "Copy storage key",
      text: "media/2026/intro-video.mp4",
      successMessage: "Storage key copied"
    }
  ];

  const actions = [
    {
      label: "Open detail page",
      onSelect: () => toastStore.push({ variant: "info", message: "Detail action triggered" })
    },
    {
      label: "Archive item",
      destructive: true,
      onSelect: () => toastStore.push({ variant: "error", message: "Archive action triggered" })
    }
  ];
</script>

<div class="copy-actions-menu-demo">
  <p class="copy-actions-menu-demo__label">Open the menu to test retained copy and toast workflow.</p>

  <CopyActionsMenu
    {toastStore}
    {copies}
    {actions}
    triggerLabel="Actions"
  />

  <ToastHost store={toastStore} autoDismissMs={2500} />
</div>

<style>
  .copy-actions-menu-demo {
    min-height: 12rem;
    min-width: 20rem;
  }

  .copy-actions-menu-demo__label {
    margin: 0 0 0.75rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
