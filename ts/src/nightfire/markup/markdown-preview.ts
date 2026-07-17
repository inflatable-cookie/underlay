import { marked } from "marked";

import { sanitizeHtml } from "../../utils/html.js";

/**
 * Render markdown to sanitized preview HTML.
 *
 * The Poodle `MarkdownEditor` preview renders via `{@html}`; every Underlay
 * wrapper must route preview HTML through this helper so raw markup like
 * `<img onerror>` cannot execute at authoring time.
 */
export function renderSafeMarkdownPreview(markdown: string): string {
  return sanitizeHtml(marked.parse(markdown, { async: false }) as string);
}
