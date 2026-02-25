// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import { MediaKind } from "../../src/patterns/media-types/enums.js";
import MediaThumbnailHarness from "../fixtures/MediaThumbnailHarness.svelte";

describe("components/MediaThumbnail.svelte", () => {
  it("renders thumbnail image with alt text and fixed dimensions", () => {
    const view = render(MediaThumbnailHarness, {
      thumbnailUrl: "https://cdn.example.com/thumb.png",
      alt: "Poster image",
      size: 64
    });

    const root = view.container.querySelector(".underlay-media-thumbnail") as HTMLElement;
    const image = view.container.querySelector("img.underlay-media-thumbnail__image") as HTMLImageElement;
    expect(root.classList.contains("underlay-media-thumbnail--has-image")).toBe(true);
    expect(root.getAttribute("style")).toContain("width: 64px;");
    expect(root.getAttribute("style")).toContain("height: 64px;");
    expect(image.getAttribute("src")).toBe("https://cdn.example.com/thumb.png");
    expect(image.getAttribute("alt")).toBe("Poster image");
  });

  it("renders icon fallback with accent and fill mode", () => {
    const view = render(MediaThumbnailHarness, {
      thumbnailUrl: null,
      kind: MediaKind.Video,
      showAccent: true,
      size: "fill",
      className: "custom-thumb"
    });

    const root = view.container.querySelector(".underlay-media-thumbnail") as HTMLElement;
    expect(root.classList.contains("underlay-media-thumbnail--fill")).toBe(true);
    expect(root.classList.contains("underlay-media-thumbnail--has-accent")).toBe(true);
    expect(root.classList.contains("custom-thumb")).toBe(true);
    expect(view.container.querySelector("img")).toBeNull();
    expect(view.container.querySelector(".underlay-media-thumbnail__icon svg")).toBeTruthy();
    expect(root.getAttribute("style")).toContain("background-color:");
  });
});
