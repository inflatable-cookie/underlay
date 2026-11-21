<script lang="ts">
  import MediaEditor from "../../src/nightfire/media/MediaEditor.svelte";

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
    initialBlock?: MediaBlock;
    onBlockChange?: (block: MediaBlock) => void;
  }

  let {
    initialBlock = { type: "media", version: "initial", data: { media_id: "" } },
    onBlockChange
  }: Props = $props();

  let block = $state<MediaBlock>(initialBlock);

  function handleChange(next: MediaBlock) {
    block = next;
    onBlockChange?.(next);
  }
</script>

<MediaEditor {block} onChange={handleChange} />
