<script lang="ts">
  import type { Snippet } from "svelte";
  import Dialog from "../components/Dialog.svelte";

  interface Props {
    open?: boolean;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    children = undefined,
  }: Props = $props();
</script>

<Dialog
  bind:open
  showTrigger={false}
  showCloseX={false}
  title={null}
  description={null}
  contentClassName="underlay-restore-resolution-dialog"
>
  {@render children?.()}
</Dialog>

<style>
  :global(.underlay-restore-resolution-dialog) {
    width: min(72rem, calc(100vw - 2rem)) !important;
    max-height: min(90vh, 56rem) !important;
  }
</style>
