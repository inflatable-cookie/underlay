import MarkdownRenderer from "./MarkdownRenderer.svelte";
import { registerBlockRenderer } from "../render-registry";

// Block renderer for generic markdown content (e.g. pathway/module descriptions).

registerBlockRenderer(null, "markdown", MarkdownRenderer);
