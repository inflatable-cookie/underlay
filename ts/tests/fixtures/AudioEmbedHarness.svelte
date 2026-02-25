<script lang="ts">
  import type { ParsedEmbed, RenderOptions } from "../../src/embed/types.js";
  import AudioEmbed from "../../src/components/AudioEmbed.svelte";

  interface Props {
    parsed?: ParsedEmbed | null;
    options?: RenderOptions | undefined;
    title?: string | undefined;
    showVolume?: boolean;
    forceIframe?: boolean;
    className?: string;
  }

  let {
    parsed = null,
    options = undefined,
    title = "Audio title",
    showVolume = true,
    forceIframe = false,
    className = ""
  }: Props = $props();
</script>

<AudioEmbed {parsed} {options} {title} {showVolume} {forceIframe} class={className} />
