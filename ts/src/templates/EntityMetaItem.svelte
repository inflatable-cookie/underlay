<script lang="ts">
  import { Code, MetaItem } from "@inflatable-cookie/poodle-svelte";
  import type { DetailMetaItemConfig } from "./template.types";

  interface Props {
    item: DetailMetaItemConfig;
  }

  let { item }: Props = $props();

  const codeLabels = new Set(["id", "slug"]);

  let shouldRenderCode = $derived(
    typeof item.value === "string" &&
      (item.code === true || codeLabels.has(item.label.trim().toLowerCase()))
  );
</script>

<MetaItem label={item.label} separator={item.separator ?? true}>
  {#if typeof item.value === "string"}
    {#if shouldRenderCode}
      <Code source={item.value} inline inlineVariant="plain" typography="inline" showCopyButton size="md" />
    {:else}
      {item.value}
    {/if}
  {:else}
    {@render item.value()}
  {/if}
</MetaItem>
