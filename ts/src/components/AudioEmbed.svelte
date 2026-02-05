<script lang="ts">
  import type { ParsedEmbed, RenderOptions } from "../embed/index.js";
  import AudioPlayer from "./AudioPlayer.svelte";
  import EmbedPreview from "./EmbedPreview.svelte";

  interface Props {
    /** Parsed embed to render */
    parsed?: ParsedEmbed | null;
    /** Render options for fallback iframe */
    options?: RenderOptions;
    /** Title to display in player */
    title?: string;
    /** Show volume control in player */
    showVolume?: boolean;
    /** Force iframe mode (skip MP3 extraction) */
    forceIframe?: boolean;
    /** Additional CSS class */
    class?: string;
  }

  let {
    parsed,
    options,
    title,
    showVolume = true,
    forceIframe = false,
    class: className,
  }: Props = $props();

  /**
   * Common audio file extensions and their MIME types
   */
  const AUDIO_EXTENSIONS: Record<string, string> = {
    ".mp3": "audio/mpeg",
    ".wav": "audio/wav",
    ".ogg": "audio/ogg",
    ".oga": "audio/ogg",
    ".m4a": "audio/mp4",
    ".aac": "audio/aac",
    ".flac": "audio/flac",
    ".webm": "audio/webm",
    ".opus": "audio/opus",
  };

  /**
   * Check if a URL is a direct audio file URL
   */
  function getAudioTypeFromUrl(url: string): string | null {
    try {
      const urlObj = new URL(url);
      const pathname = urlObj.pathname.toLowerCase();

      for (const [ext, mimeType] of Object.entries(AUDIO_EXTENSIONS)) {
        if (pathname.endsWith(ext)) {
          return mimeType;
        }
      }
    } catch {
      // Invalid URL, check raw string
      const lowerUrl = url.toLowerCase();
      for (const [ext, mimeType] of Object.entries(AUDIO_EXTENSIONS)) {
        if (lowerUrl.endsWith(ext)) {
          return mimeType;
        }
      }
    }
    return null;
  }

  /**
   * Extract direct audio URL from parsed embed.
   * Returns null if direct URL cannot be determined.
   */
  function getDirectAudioUrl(embed: ParsedEmbed): { url: string; type: string } | null {
    // First, check if the original URL is a direct audio file
    if (embed.originalUrl) {
      const mimeType = getAudioTypeFromUrl(embed.originalUrl);
      if (mimeType) {
        return {
          url: embed.originalUrl,
          type: mimeType,
        };
      }
    }

    // Provider-specific extraction
    // Audioboom single posts have direct MP3 URLs
    if (
      embed.provider === "audioboom" &&
      embed.embedType !== "playlist"
    ) {
      return {
        url: `https://audioboom.com/posts/${embed.id}.mp3`,
        type: "audio/mpeg",
      };
    }

    // Add more providers here as needed
    // Example: SoundCloud, Spotify, etc. would require API calls

    return null;
  }

  // Determine if we can use direct audio playback
  const audioSource = $derived.by(() => {
    if (!parsed || forceIframe) return null;
    return getDirectAudioUrl(parsed);
  });

  // Whether to use the custom player
  const usePlayer = $derived(audioSource !== null);
</script>

<div class="underlay-audio-embed {className ?? ''}">
  {#if !parsed}
    <EmbedPreview
      parsed={null}
      mediaType="audio"
      emptyMessage="No audio source available"
    />
  {:else if usePlayer && audioSource}
    <AudioPlayer
      src={audioSource.url}
      type={audioSource.type}
      {title}
      {showVolume}
    />
  {:else}
    <EmbedPreview
      {parsed}
      {options}
      mediaType="audio"
      aspectRatio="auto"
    />
  {/if}
</div>

<style>
  .underlay-audio-embed {
    /* Minimal container - allows child components to define their own styling */
    display: block;
  }
</style>
