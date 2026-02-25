// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import EmbedPreviewHarness from "../fixtures/EmbedPreviewHarness.svelte";

describe("components/EmbedPreview.svelte", () => {
  it("renders loading and error states", () => {
    const loadingView = render(EmbedPreviewHarness, {
      loading: true
    });
    expect(screen.getByText("Loading preview...")).toBeTruthy();
    loadingView.unmount();

    render(EmbedPreviewHarness, {
      error: "Preview unavailable"
    });
    expect(screen.getByText("Preview unavailable")).toBeTruthy();
  });

  it("sanitizes embed html and uses aspect ratio container for video", () => {
    const view = render(EmbedPreviewHarness, {
      mediaType: "video",
      aspectRatio: 16 / 9,
      embedHtml: '<script>alert(1)</script><iframe src="https://example.com/embed/abc"></iframe>'
    });

    expect(view.container.querySelector(".underlay-embed-preview__aspect")).toBeTruthy();
    expect(view.container.querySelector("iframe")).toBeTruthy();
    expect(view.container.querySelector("script")).toBeNull();
  });

  it("shows audio empty state message when no source exists", () => {
    render(EmbedPreviewHarness, {
      parsed: null,
      mediaType: "audio",
      emptyMessage: "No audio preview"
    });

    expect(screen.getByText("No audio preview")).toBeTruthy();
    expect(document.querySelector(".underlay-embed-preview__empty")).toBeTruthy();
  });
});
