<script lang="ts">
  import type { Snippet } from "svelte";
  import PageHeaderMeta from "../PageHeaderMeta.svelte";
  import PageHeaderMetaRow from "../PageHeaderMetaRow.svelte";

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<PageHeaderMeta>
  <PageHeaderMetaRow>
    {#if children}
      {@render children()}
    {/if}
  </PageHeaderMetaRow>
</PageHeaderMeta>
