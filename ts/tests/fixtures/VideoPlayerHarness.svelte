<script lang="ts">
  import VideoPlayer from "../../src/components/VideoPlayer.svelte";

  interface Props {
    title?: string;
    provider?: string | null | undefined;
    externalId?: string | null | undefined;
    embedSource?: string;
    thumbnailUrl?: string | null | undefined;
  }

  let {
    title = "Video title",
    provider = "youtube",
    externalId = "abc123",
    embedSource = '<iframe src="https://example.com/embed/1"></iframe>',
    thumbnailUrl = null
  }: Props = $props();
</script>

<VideoPlayer {title} {provider} {externalId} {embedSource} {thumbnailUrl} />
