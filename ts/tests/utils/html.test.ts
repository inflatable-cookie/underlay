import { describe, expect, it } from "vitest";
import {
  sanitizeEmbedHtml,
  sanitizeHtml,
  sanitizeSvgHtml,
} from "../../src/utils/html";

describe("utils/html", () => {
  it("strips script tags, event handlers, and javascript: URLs and handles nullish input", () => {
    const out = sanitizeHtml(
      '<p>Hello</p><script>alert(1)</script><img src=x onerror="alert(2)"><a href="javascript:alert(3)">click</a>',
    );
    expect(out).toContain("<p>Hello</p>");
    expect(out).not.toContain("<script");
    expect(out).not.toContain("onerror");
    expect(out).not.toContain("javascript:");
    expect(sanitizeHtml(null)).toBe("");
    expect(sanitizeHtml(undefined)).toBe("");
  });

  it("preserves iframe sandbox attribute while stripping scripts in embeds", () => {
    const out = sanitizeEmbedHtml(
      '<iframe src="https://example.com/embed" sandbox="allow-scripts" onload="evil()"></iframe><script>x</script>',
    );
    expect(out).toContain("<iframe");
    expect(out).toContain('sandbox="allow-scripts"');
    expect(out).not.toContain("onload");
    expect(out).not.toContain("<script");
  });

  it("strips foreignObject, script, and style attributes from SVGs", () => {
    const out = sanitizeSvgHtml(
      '<svg viewBox="0 0 10 10"><path d="M0 0" style="fill:url(javascript:alert(1))"/><foreignObject><body>hi</body></foreignObject><script>alert(1)</script></svg>',
    );
    expect(out).toContain("<svg");
    expect(out).toContain("<path");
    expect(out).not.toContain("style=");
    expect(out).not.toContain("foreignObject");
    expect(out).not.toContain("<script");
    expect(sanitizeSvgHtml(undefined)).toBe("");
  });
});
