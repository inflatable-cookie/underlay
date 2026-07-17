// @vitest-environment jsdom
import { describe, expect, it } from "vitest";

import { renderSafeMarkdownPreview } from "../../src/nightfire/markup/markdown-preview";

describe("nightfire/markup renderSafeMarkdownPreview", () => {
	it("renders plain markdown to HTML", () => {
		const html = renderSafeMarkdownPreview("# Hello\n\nSome **bold** text.");

		expect(html).toContain("<h1>Hello</h1>");
		expect(html).toContain("<strong>bold</strong>");
	});

	it("strips script tags", () => {
		const html = renderSafeMarkdownPreview('before\n\n<script>window.__pwned = true;</script>\n\nafter');

		expect(html).not.toContain("<script");
		expect(html).toContain("before");
		expect(html).toContain("after");
	});

	it("strips event handler attributes from inline HTML", () => {
		const html = renderSafeMarkdownPreview('<img src=x onerror="window.__pwned = true">');

		expect(html).not.toContain("onerror");
	});

	it("strips javascript: URLs", () => {
		const html = renderSafeMarkdownPreview('[click](javascript:alert(1))');

		expect(html).not.toContain("javascript:");
	});

	it("keeps safe inline HTML", () => {
		const html = renderSafeMarkdownPreview('<em>fine</em> and <a href="https://example.com">link</a>');

		expect(html).toContain("<em>fine</em>");
		expect(html).toContain('href="https://example.com"');
	});
});
