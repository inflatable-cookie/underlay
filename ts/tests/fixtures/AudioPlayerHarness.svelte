<script lang="ts">
  import AudioPlayer from "../../src/components/AudioPlayer.svelte";

  interface Props {
    src?: string;
    type?: string;
    title?: string;
    showVolume?: boolean;
    autoplay?: boolean;
    volume?: number;
    className?: string;
  }

  let {
    src = "https://example.com/audio.mp3",
    type = "audio/mpeg",
    title = "Sample audio",
    showVolume = true,
    autoplay = false,
    volume = 1,
    className = ""
  }: Props = $props();
</script>

<AudioPlayer {src} {type} {title} {showVolume} {autoplay} {volume} class={className} />
