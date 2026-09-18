// @vitest-environment jsdom
import { describe, expect, it } from "vitest";

import * as underlayEditor from "@inflatable-cookie/underlay/nightfire/editor";
import * as upstreamEditor from "@inflatable-cookie/nightfire/editor";
import * as underlayRenderer from "@inflatable-cookie/underlay/nightfire/renderer";
import * as upstreamRenderer from "@inflatable-cookie/nightfire/renderer";
import * as underlayBlockEditor from "@inflatable-cookie/underlay/nightfire/block-editor";
import * as upstreamBlockEditor from "@inflatable-cookie/nightfire/block-editor";
import * as underlayMarkdown from "@inflatable-cookie/underlay/nightfire/markdown";
import * as upstreamMarkdown from "@inflatable-cookie/nightfire/markdown";

describe("Nightfire Svelte compatibility facades", () => {
  it("re-exports the released package components", () => {
    expect(underlayEditor.NightfireEditor).toBe(upstreamEditor.NightfireEditor);
    expect(underlayEditor.SlashCommandPalette).toBe(upstreamEditor.SlashCommandPalette);
    expect(underlayRenderer.NightfireRenderer).toBe(upstreamRenderer.NightfireRenderer);
    expect(underlayBlockEditor.NightfireBlockEditor).toBe(upstreamBlockEditor.NightfireBlockEditor);
    expect(underlayMarkdown.MarkdownEditor).toBe(upstreamMarkdown.MarkdownEditor);
    expect(underlayMarkdown.MarkdownEditorSurface).toBe(upstreamMarkdown.MarkdownEditorSurface);
    expect(underlayMarkdown.MarkdownRenderer).toBe(upstreamMarkdown.MarkdownRenderer);
  });
});
