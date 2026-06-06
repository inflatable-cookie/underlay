import { describe, expect, it } from "vitest";
import { generateNonce } from "../../../src/server/csp";

describe("generateNonce", () => {
	it("generates a non-empty string", () => {
		const nonce = generateNonce();
		expect(typeof nonce).toBe("string");
		expect(nonce.length).toBeGreaterThan(0);
	});

	it("generates unique values", () => {
		const nonces = new Set<string>();
		for (let i = 0; i < 100; i++) {
			nonces.add(generateNonce());
		}
		expect(nonces.size).toBe(100);
	});

	it("generates base64-encoded values", () => {
		const nonce = generateNonce();
		expect(nonce).toMatch(/^[A-Za-z0-9+/]+=*$/);
	});

	it("generates 16 bytes (produces ~22-24 char base64)", () => {
		const nonce = generateNonce();
		expect(nonce.length).toBeGreaterThanOrEqual(22);
		expect(nonce.length).toBeLessThanOrEqual(24);
	});
});
