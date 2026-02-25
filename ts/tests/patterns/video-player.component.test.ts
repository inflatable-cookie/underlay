// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import VideoPlayerHarness from "../fixtures/VideoPlayerHarness.svelte";

describe("components/VideoPlayer.svelte", () => {
  it("renders provider iframe for youtube/vimeo embeds", () => {
    const view = render(VideoPlayerHarness, {
      title: "Video A",
      provider: "youtube",
      externalId: "xyz789"
    });

    const iframe = view.container.querySelector('iframe[title="Video A"]') as HTMLIFrameElement;
    expect(iframe).toBeTruthy();
    expect(iframe.getAttribute("src")).toBe("https://www.youtube.com/embed/xyz789");
    expect(view.container.querySelector("figcaption")?.textContent).toContain("Video A");
  });

  it("falls back to sanitized raw embed and optional thumbnail when provider is unsupported", () => {
    const view = render(VideoPlayerHarness, {
      title: "Fallback video",
      provider: "custom",
      externalId: null,
      thumbnailUrl: "https://cdn.example.com/thumb.jpg",
      embedSource: '<script>alert(1)</script><iframe src="https://example.com/embed/custom"></iframe>'
    });

    const thumbnail = view.container.querySelector('[data-video-thumbnail] img') as HTMLImageElement;
    expect(thumbnail).toBeTruthy();
    expect(thumbnail.getAttribute("src")).toBe("https://cdn.example.com/thumb.jpg");
    expect(view.container.querySelector('[data-video-raw-embed] iframe')).toBeTruthy();
    expect(view.container.querySelector('[data-video-raw-embed] script')).toBeNull();
  });
});
