<script lang="ts">
  import MediaEditor from "../../src/nightfire/media/MediaEditor.svelte";
  import {
    createNightfireMediaContext,
    type NightfireMediaContext
  } from "../../src/nightfire/media/context";

  interface MediaBlock {
    type: string;
    version?: string;
    data?: {
      media_id?: string;
      caption?: string;
      alt?: string;
      display?: string;
    };
  }

  interface Props {
    pickMedia?: NightfireMediaContext["pickMedia"];
    resolveMedia?: NightfireMediaContext["resolveMedia"];
    mediaId?: string;
    caption?: string;
    alt?: string;
    display?: string;
    onBlockChange?: (block: MediaBlock) => void;
  }

  let {
    pickMedia = async () => null,
    resolveMedia = undefined,
    mediaId = "",
    caption = "",
    alt = "",
    display = "inline",
    onBlockChange
  }: Props = $props();

  createNightfireMediaContext({
    get pickMedia() {
      return pickMedia;
    },
    get resolveMedia() {
      return resolveMedia;
    }
  });

  const block = $derived<MediaBlock>({
    type: "media",
    version: "initial",
    data: { media_id: mediaId, caption, alt, display }
  });
</script>

<MediaEditor {block} onChange={(next) => onBlockChange?.(next)} />
