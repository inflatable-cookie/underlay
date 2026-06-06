import { describe, expect, it } from "vitest";
import { createCspResolveOptions } from "../../../src/server/csp";

describe("createCspResolveOptions", () => {
	it("returns options object with transformPageChunk", () => {
		const nonce = "test-nonce";
		const options = createCspResolveOptions(nonce);
		expect(typeof options.transformPageChunk).toBe("function");
	});

	it("replaces nonce placeholder in HTML", async () => {
		const nonce = "my-secure-nonce";
		const options = createCspResolveOptions(nonce);
		const html = '<script nonce="%sveltekit.nonce%">console.log("hi")</script>';

		const result = await options.transformPageChunk({ html, done: true });
		expect(result).toBe(`<script nonce="${nonce}">console.log("hi")</script>`);
	});

	it("replaces multiple nonce placeholders", async () => {
		const nonce = "multi-nonce";
		const options = createCspResolveOptions(nonce);
		const html = `
			<script nonce="%sveltekit.nonce%">one</script>
			<script nonce="%sveltekit.nonce%">two</script>
		`;

		const result = await options.transformPageChunk({ html, done: true });
		expect(result).not.toContain("%sveltekit.nonce%");
		expect(result.match(new RegExp(nonce, "g"))?.length).toBe(2);
	});

	it("preserves existing options", () => {
		const nonce = "test";
		const filterFn = (name: string) => name === "content-type";
		const options = createCspResolveOptions(nonce, {
			filterSerializedResponseHeaders: filterFn,
		});

		expect(options.filterSerializedResponseHeaders).toBe(filterFn);
	});

	it("chains existing transformPageChunk", async () => {
		const nonce = "chained";
		const options = createCspResolveOptions(nonce, {
			transformPageChunk: async ({ html }) => html.toUpperCase(),
		});

		const html = '<script nonce="%sveltekit.nonce%">test</script>';
		const result = await options.transformPageChunk({ html, done: true });

		expect(result).toContain(nonce.toUpperCase());
		expect(result).not.toContain("%sveltekit.nonce%");
	});
});
