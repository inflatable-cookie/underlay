<script lang="ts">
  import { onMount } from "svelte";
  import Play from "lucide-svelte/icons/play";
  import Pause from "lucide-svelte/icons/pause";
  import Volume2 from "lucide-svelte/icons/volume-2";
  import VolumeX from "lucide-svelte/icons/volume-x";
  import AlertCircle from "lucide-svelte/icons/alert-circle";
  import Loader from "lucide-svelte/icons/loader";

  interface Props {
    /** Audio source URL */
    src: string;
    /** MIME type (default: audio/mpeg) */
    type?: string;
    /** Title to display */
    title?: string;
    /** Show volume control */
    showVolume?: boolean;
    /** Auto-play on load */
    autoplay?: boolean;
    /** Initial volume (0-1) */
    volume?: number;
    /** Additional CSS class */
    class?: string;
  }

  let {
    src,
    type = "audio/mpeg",
    title,
    showVolume = true,
    autoplay = false,
    volume: initialVolume = 1,
    class: className,
  }: Props = $props();

  let audioElement: HTMLAudioElement | null = $state(null);
  let isPlaying = $state(false);
  let isLoading = $state(true);
  let hasError = $state(false);
  let errorMessage = $state<string | null>(null);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(initialVolume);
  let isMuted = $state(false);
  let isDragging = $state(false);

  // Format time as MM:SS or HH:MM:SS
  function formatTime(seconds: number): string {
    if (!isFinite(seconds) || isNaN(seconds)) return "0:00";

    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);

    if (hrs > 0) {
      return `${hrs}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
    }
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  // Progress percentage
  const progress = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);

  // Handle play/pause toggle
  function togglePlay() {
    if (!audioElement) return;

    if (isPlaying) {
      audioElement.pause();
    } else {
      audioElement.play().catch((e) => {
        console.error("Play failed:", e);
        hasError = true;
        errorMessage = "Failed to play audio";
      });
    }
  }

  // Handle seek
  function handleSeek(event: MouseEvent | TouchEvent) {
    if (!audioElement || duration === 0) return;

    const target = event.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const clientX = "touches" in event ? event.touches[0].clientX : event.clientX;
    const x = clientX - rect.left;
    const percentage = Math.max(0, Math.min(1, x / rect.width));

    audioElement.currentTime = percentage * duration;
  }

  // Handle volume change
  function handleVolumeChange(event: Event) {
    const target = event.target as HTMLInputElement;
    volume = parseFloat(target.value);
    if (audioElement) {
      audioElement.volume = volume;
    }
    if (volume > 0) {
      isMuted = false;
    }
  }

  // Toggle mute
  function toggleMute() {
    if (!audioElement) return;

    isMuted = !isMuted;
    audioElement.muted = isMuted;
  }

  // Setup audio event listeners
  onMount(() => {
    if (!audioElement) return;

    const handlePlay = () => { isPlaying = true; };
    const handlePause = () => { isPlaying = false; };
    const handleEnded = () => { isPlaying = false; currentTime = 0; };
    const handleTimeUpdate = () => {
      if (!isDragging) {
        currentTime = audioElement?.currentTime ?? 0;
      }
    };
    const handleDurationChange = () => { duration = audioElement?.duration ?? 0; };
    const handleLoadedData = () => { isLoading = false; };
    const handleCanPlay = () => { isLoading = false; };
    const handleWaiting = () => { isLoading = true; };
    const handleError = () => {
      isLoading = false;
      hasError = true;
      errorMessage = "Failed to load audio";
    };

    audioElement.addEventListener("play", handlePlay);
    audioElement.addEventListener("pause", handlePause);
    audioElement.addEventListener("ended", handleEnded);
    audioElement.addEventListener("timeupdate", handleTimeUpdate);
    audioElement.addEventListener("durationchange", handleDurationChange);
    audioElement.addEventListener("loadeddata", handleLoadedData);
    audioElement.addEventListener("canplay", handleCanPlay);
    audioElement.addEventListener("waiting", handleWaiting);
    audioElement.addEventListener("error", handleError);

    // Set initial volume
    audioElement.volume = volume;

    return () => {
      audioElement?.removeEventListener("play", handlePlay);
      audioElement?.removeEventListener("pause", handlePause);
      audioElement?.removeEventListener("ended", handleEnded);
      audioElement?.removeEventListener("timeupdate", handleTimeUpdate);
      audioElement?.removeEventListener("durationchange", handleDurationChange);
      audioElement?.removeEventListener("loadeddata", handleLoadedData);
      audioElement?.removeEventListener("canplay", handleCanPlay);
      audioElement?.removeEventListener("waiting", handleWaiting);
      audioElement?.removeEventListener("error", handleError);
    };
  });
</script>

<div class="underlay-audio-player {className ?? ''}">
  <!-- Hidden audio element -->
  <audio
    bind:this={audioElement}
    {autoplay}
    preload="metadata"
  >
    <source {src} {type} />
    Your browser does not support the audio element.
  </audio>

  {#if hasError}
    <div class="underlay-audio-player__error">
      <AlertCircle size={20} />
      <span>{errorMessage ?? "Audio unavailable"}</span>
    </div>
  {:else}
    <div class="underlay-audio-player__controls">
      <!-- Play/Pause button -->
      <button
        type="button"
        class="underlay-audio-player__play-btn"
        onclick={togglePlay}
        disabled={isLoading && !isPlaying}
        aria-label={isPlaying ? "Pause" : "Play"}
      >
        {#if isLoading && !isPlaying}
          <Loader size={24} class="underlay-audio-player__spinner" />
        {:else if isPlaying}
          <Pause size={24} />
        {:else}
          <Play size={24} />
        {/if}
      </button>

      <!-- Progress section -->
      <div class="underlay-audio-player__progress-section">
        {#if title}
          <div class="underlay-audio-player__title">{title}</div>
        {/if}

        <!-- Progress bar -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="underlay-audio-player__progress-bar"
          onclick={handleSeek}
          onmousedown={() => isDragging = true}
          onmouseup={() => isDragging = false}
          ontouchstart={() => isDragging = true}
          ontouchend={() => isDragging = false}
        >
          <div
            class="underlay-audio-player__progress-fill"
            style:width="{progress}%"
          ></div>
          <div
            class="underlay-audio-player__progress-handle"
            style:left="{progress}%"
          ></div>
        </div>

        <!-- Time display -->
        <div class="underlay-audio-player__time">
          <span class="underlay-audio-player__current">{formatTime(currentTime)}</span>
          <span class="underlay-audio-player__separator">/</span>
          <span class="underlay-audio-player__duration">{formatTime(duration)}</span>
        </div>
      </div>

      <!-- Volume control -->
      {#if showVolume}
        <div class="underlay-audio-player__volume">
          <button
            type="button"
            class="underlay-audio-player__mute-btn"
            onclick={toggleMute}
            aria-label={isMuted ? "Unmute" : "Mute"}
          >
            {#if isMuted || volume === 0}
              <VolumeX size={20} />
            {:else}
              <Volume2 size={20} />
            {/if}
          </button>
          <input
            type="range"
            min="0"
            max="1"
            step="0.05"
            value={isMuted ? 0 : volume}
            oninput={handleVolumeChange}
            class="underlay-audio-player__volume-slider"
            aria-label="Volume"
          />
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .underlay-audio-player {
    --player-bg: var(--underlay-audio-player-bg, var(--underlay-color-surface-subtle, #f1f5f9));
    --player-fg: var(--underlay-audio-player-fg, var(--underlay-color-text, #1e293b));
    --player-muted: var(--underlay-audio-player-muted, var(--underlay-color-text-muted, #64748b));
    --player-accent: var(--underlay-audio-player-accent, var(--underlay-color-primary, #2563eb));
    --player-track: var(--underlay-audio-player-track, var(--underlay-color-border, #e2e8f0));
    --player-radius: var(--underlay-audio-player-radius, 0.75rem);

    background: var(--player-bg);
    border-radius: var(--player-radius);
    padding: 1rem 1.25rem;
  }

  .underlay-audio-player__controls {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .underlay-audio-player__play-btn {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border: none;
    border-radius: 50%;
    background: var(--player-accent);
    color: white;
    cursor: pointer;
    transition: transform 0.15s ease, opacity 0.15s ease;
  }

  .underlay-audio-player__play-btn:hover:not(:disabled) {
    transform: scale(1.05);
  }

  .underlay-audio-player__play-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .underlay-audio-player__progress-section {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .underlay-audio-player__title {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--player-fg);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .underlay-audio-player__progress-bar {
    position: relative;
    height: 0.5rem;
    background: var(--player-track);
    border-radius: 0.25rem;
    cursor: pointer;
  }

  .underlay-audio-player__progress-fill {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    background: var(--player-accent);
    border-radius: 0.25rem;
    pointer-events: none;
  }

  .underlay-audio-player__progress-handle {
    position: absolute;
    top: 50%;
    width: 1rem;
    height: 1rem;
    background: var(--player-accent);
    border: 2px solid white;
    border-radius: 50%;
    transform: translate(-50%, -50%);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
    opacity: 0;
    transition: opacity 0.15s ease;
    pointer-events: none;
  }

  .underlay-audio-player__progress-bar:hover .underlay-audio-player__progress-handle {
    opacity: 1;
  }

  .underlay-audio-player__time {
    display: flex;
    gap: 0.25rem;
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    color: var(--player-muted);
  }

  .underlay-audio-player__volume {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .underlay-audio-player__mute-btn {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: none;
    background: transparent;
    color: var(--player-muted);
    cursor: pointer;
    border-radius: 0.25rem;
    transition: color 0.15s ease, background 0.15s ease;
  }

  .underlay-audio-player__mute-btn:hover {
    color: var(--player-fg);
    background: var(--player-track);
  }

  .underlay-audio-player__volume-slider {
    width: 5rem;
    height: 0.375rem;
    -webkit-appearance: none;
    appearance: none;
    background: var(--player-track);
    border-radius: 0.1875rem;
    cursor: pointer;
  }

  .underlay-audio-player__volume-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 0.875rem;
    height: 0.875rem;
    background: var(--player-accent);
    border: 2px solid white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
  }

  .underlay-audio-player__volume-slider::-moz-range-thumb {
    width: 0.875rem;
    height: 0.875rem;
    background: var(--player-accent);
    border: 2px solid white;
    border-radius: 50%;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
  }

  .underlay-audio-player__error {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--underlay-color-danger, #ef4444);
    font-size: 0.875rem;
  }

  :global(.underlay-audio-player__spinner) {
    animation: underlay-audio-spin 1s linear infinite;
  }

  @keyframes underlay-audio-spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Responsive: hide volume on small screens */
  @media (max-width: 480px) {
    .underlay-audio-player__volume {
      display: none;
    }
  }
</style>
