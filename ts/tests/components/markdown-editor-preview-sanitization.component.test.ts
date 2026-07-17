// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";

import MarkdownEditorSurface from "../../src/nightfire/markup/MarkdownEditorSurface.svelte";

describe("nightfire/markup MarkdownEditorSurface preview sanitization", () => {
  it("does not render script tags or event handlers in the live preview", () => {
    const view = render(MarkdownEditorSurface, {
      value: '<img src=x onerror="window.__pwned = true"><script>window.__pwned = true;</script>\n\n# Safe heading',
      showPreview: true,
    });

    const preview = view.container.querySelector(".poodle-md-editor__preview");
    expect(preview).not.toBeNull();
    expect(preview!.innerHTML).not.toContain("onerror");
    expect(preview!.querySelector("script")).toBeNull();
    expect(preview!.querySelector("h1")?.textContent).toBe("Safe heading");
    expect((window as unknown as { __pwned?: boolean }).__pwned).toBeUndefined();
  });
});
