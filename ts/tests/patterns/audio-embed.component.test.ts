// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import AudioEmbedHarness from "../fixtures/AudioEmbedHarness.svelte";

describe("components/AudioEmbed.svelte", () => {
  it("renders empty preview when parsed embed is missing", () => {
    render(AudioEmbedHarness, {
      parsed: null
    });

    expect(screen.getByText("No audio source available")).toBeTruthy();
    expect(document.querySelector(".underlay-audio-player")).toBeNull();
  });

  it("uses direct audio player when a direct file URL can be resolved", () => {
    const view = render(AudioEmbedHarness, {
      parsed: {
        provider: "generic",
        id: "audio-1",
        originalUrl: "https://cdn.example.com/media/test.wav"
      },
      title: "Direct audio"
    });

    const source = view.container.querySelector("audio source") as HTMLSourceElement;
    expect(view.container.querySelector(".underlay-audio-player")).toBeTruthy();
    expect(source.getAttribute("src")).toBe("https://cdn.example.com/media/test.wav");
    expect(source.getAttribute("type")).toBe("audio/wav");
  });

  it("respects forceIframe and uses preview fallback instead of direct player", () => {
    const view = render(AudioEmbedHarness, {
      forceIframe: true,
      parsed: {
        provider: "generic",
        id: "audio-2",
        originalUrl: "https://cdn.example.com/media/test.mp3",
        originalEmbed: '<iframe src="https://example.com/embed/audio-2"></iframe>'
      }
    });

    expect(view.container.querySelector(".underlay-embed-preview")).toBeTruthy();
    expect(view.container.querySelector(".underlay-audio-player")).toBeNull();
    expect(view.container.querySelector("iframe")).toBeTruthy();
  });
});
