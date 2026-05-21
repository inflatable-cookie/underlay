// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";

import NightfireEditorHarness from "../fixtures/NightfireEditorHarness.svelte";

describe("nightfire/NightfireEditor defaults", () => {
  it("prefers schema default type over the first block option in the type picker", async () => {
    const view = render(NightfireEditorHarness, {
      schema: "acow:content/description@1",
      initialValue: {
        schema: "acow:content/description@1",
      },
      modeOverride: "single",
      defaultTypeOverride: "markdown",
      blockOptions: [
        { type: "content.list", label: "Content List" },
        { type: "markdown", label: "Markdown" },
      ],
    });

    const typeSelect = await screen.findByLabelText("Block type");
    expect((typeSelect as HTMLSelectElement).value).toBe("markdown");
    expect(view.getByTestId("nightfire-value").textContent).toContain("\"schema\": \"acow:content/description@1\"");
  });
});
