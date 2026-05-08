<script lang="ts" generics="T">
  import type { Snippet } from "svelte";
  import { InlineListSection } from "@poodle/svelte";
  import EntityDetailModule from "./EntityDetailModule.svelte";

  interface Props {
    title: string;
    items: T[];
    item: Snippet<[T]>;
    actions?: Snippet;
    emptyMessage?: string | null;
    span?: "half" | "full";
  }

  let {
    title,
    items,
    item,
    actions,
    emptyMessage = "No items yet.",
    span = "half"
  }: Props = $props();
</script>

<EntityDetailModule {span}>
  <InlineListSection
    {title}
    {items}
    {item}
    {actions}
    {emptyMessage}
    framed={false}
  />
</EntityDetailModule>
