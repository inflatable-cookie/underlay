// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import AudioPlayerHarness from "../fixtures/AudioPlayerHarness.svelte";

describe("components/AudioPlayer.svelte", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.spyOn(HTMLMediaElement.prototype, "play").mockImplementation(function () {
      this.dispatchEvent(new Event("play"));
      return Promise.resolve();
    });
    vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(function () {
      this.dispatchEvent(new Event("pause"));
    });
  });

  it("renders controls, responds to media events, and toggles play/mute", async () => {
    const view = render(AudioPlayerHarness, {
      src: "https://example.com/audio.mp3",
      title: "Episode One",
      volume: 0.5,
      showVolume: true
    });

    const audio = view.container.querySelector("audio") as HTMLAudioElement;
    const playButton = screen.getByRole("button", { name: "Play" }) as HTMLButtonElement;
    expect(playButton.disabled).toBe(true);
    expect(screen.getByText("Episode One")).toBeTruthy();
    await Promise.resolve();

    let current = 65;
    Object.defineProperty(audio, "duration", {
      configurable: true,
      get: () => 125
    });
    Object.defineProperty(audio, "currentTime", {
      configurable: true,
      get: () => current,
      set: (next: number) => {
        current = next;
      }
    });

    audio.dispatchEvent(new Event("canplay"));
    audio.dispatchEvent(new Event("durationchange"));
    audio.dispatchEvent(new Event("timeupdate"));
    await waitFor(() => {
      expect(playButton.disabled).toBe(false);
    });
    expect(screen.getByText("1:05")).toBeTruthy();
    expect(screen.getByText("2:05")).toBeTruthy();

    await fireEvent.click(playButton);
    expect(screen.getByRole("button", { name: "Pause" })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Mute" }));
    expect(screen.getByRole("button", { name: "Unmute" })).toBeTruthy();
  });

  it("renders error state when audio element emits error", async () => {
    const view = render(AudioPlayerHarness, {
      src: "https://example.com/broken.mp3"
    });

    const audio = view.container.querySelector("audio") as HTMLAudioElement;
    await Promise.resolve();
    audio.dispatchEvent(new Event("error"));
    await waitFor(() => {
      expect(screen.getByText("Failed to load audio")).toBeTruthy();
    });
  });
});
