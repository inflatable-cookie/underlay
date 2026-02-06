/**
 * Tests for Base64URL encoding/decoding utilities.
 */

import { describe, it, expect } from "vitest";
import {
	base64urlToArrayBuffer,
	arrayBufferToBase64url,
} from "../../src/utils/base64url";

// ============================================================================
// base64urlToArrayBuffer
// ============================================================================

describe("base64urlToArrayBuffer", () => {
	it("decodes empty string", () => {
		const buffer = base64urlToArrayBuffer("");
		expect(buffer.byteLength).toBe(0);
	});

	it("decodes simple ASCII text", () => {
		// "hello" in Base64URL is "aGVsbG8"
		const buffer = base64urlToArrayBuffer("aGVsbG8");
		const bytes = new Uint8Array(buffer);
		expect(String.fromCharCode(...bytes)).toBe("hello");
	});

	it("decodes binary data", () => {
		// Bytes [0, 1, 2, 3] -> Base64URL "AAECAw"
		const buffer = base64urlToArrayBuffer("AAECAw");
		const bytes = new Uint8Array(buffer);
		expect([...bytes]).toEqual([0, 1, 2, 3]);
	});

	it("handles URL-safe characters (- instead of +)", () => {
		// Standard Base64 uses +, Base64URL uses -
		// "??" in Base64 is "Pz8=" or "Pz8", with + would be different
		// Let's use a known value: bytes [251, 239] = Base64URL "--8"
		const buffer = base64urlToArrayBuffer("--8");
		const bytes = new Uint8Array(buffer);
		expect(bytes[0]).toBe(251);
		expect(bytes[1]).toBe(239);
	});

	it("handles URL-safe characters (_ instead of /)", () => {
		// Bytes [255, 255] = Base64URL "__8"
		const buffer = base64urlToArrayBuffer("__8");
		const bytes = new Uint8Array(buffer);
		expect(bytes[0]).toBe(255);
		expect(bytes[1]).toBe(255);
	});

	it("handles missing padding", () => {
		// Base64URL omits padding, but decoder should handle it
		// "a" = [97] = Base64 "YQ==" or Base64URL "YQ"
		const buffer = base64urlToArrayBuffer("YQ");
		const bytes = new Uint8Array(buffer);
		expect(bytes[0]).toBe(97);
	});

	it("decodes WebAuthn-style challenge", () => {
		// Typical 32-byte random challenge
		const challenge = "dGVzdC1jaGFsbGVuZ2UtdmFsdWUtaGVyZQ";
		const buffer = base64urlToArrayBuffer(challenge);
		const text = String.fromCharCode(...new Uint8Array(buffer));
		expect(text).toBe("test-challenge-value-here");
	});
});

// ============================================================================
// arrayBufferToBase64url
// ============================================================================

describe("arrayBufferToBase64url", () => {
	it("encodes empty buffer", () => {
		const buffer = new ArrayBuffer(0);
		expect(arrayBufferToBase64url(buffer)).toBe("");
	});

	it("encodes simple ASCII text", () => {
		const bytes = new Uint8Array([104, 101, 108, 108, 111]); // "hello"
		expect(arrayBufferToBase64url(bytes.buffer)).toBe("aGVsbG8");
	});

	it("encodes binary data", () => {
		const bytes = new Uint8Array([0, 1, 2, 3]);
		expect(arrayBufferToBase64url(bytes.buffer)).toBe("AAECAw");
	});

	it("uses URL-safe character - instead of +", () => {
		const bytes = new Uint8Array([251, 239]);
		const result = arrayBufferToBase64url(bytes.buffer);
		expect(result).not.toContain("+");
		expect(result).toContain("-");
	});

	it("uses URL-safe character _ instead of /", () => {
		const bytes = new Uint8Array([255, 255]);
		const result = arrayBufferToBase64url(bytes.buffer);
		expect(result).not.toContain("/");
		expect(result).toContain("_");
	});

	it("omits padding", () => {
		// Single byte should produce 2 Base64 chars + 2 padding in standard Base64
		const bytes = new Uint8Array([97]); // "a"
		const result = arrayBufferToBase64url(bytes.buffer);
		expect(result).not.toContain("=");
		expect(result).toBe("YQ");
	});

	it("encodes larger buffers correctly", () => {
		const bytes = new Uint8Array([
			116, 101, 115, 116, 45, 99, 104, 97, 108, 108, 101, 110, 103, 101, 45,
			118, 97, 108, 117, 101, 45, 104, 101, 114, 101,
		]); // "test-challenge-value-here"
		expect(arrayBufferToBase64url(bytes.buffer)).toBe(
			"dGVzdC1jaGFsbGVuZ2UtdmFsdWUtaGVyZQ"
		);
	});
});

// ============================================================================
// Round-trip tests
// ============================================================================

describe("base64url round-trip", () => {
	it("preserves empty data", () => {
		const original = new Uint8Array([]);
		const encoded = arrayBufferToBase64url(original.buffer);
		const decoded = base64urlToArrayBuffer(encoded);
		expect([...new Uint8Array(decoded)]).toEqual([...original]);
	});

	it("preserves single byte", () => {
		const original = new Uint8Array([42]);
		const encoded = arrayBufferToBase64url(original.buffer);
		const decoded = base64urlToArrayBuffer(encoded);
		expect([...new Uint8Array(decoded)]).toEqual([...original]);
	});

	it("preserves random binary data", () => {
		const original = new Uint8Array([
			0, 1, 127, 128, 255, 42, 17, 200, 99, 33,
		]);
		const encoded = arrayBufferToBase64url(original.buffer);
		const decoded = base64urlToArrayBuffer(encoded);
		expect([...new Uint8Array(decoded)]).toEqual([...original]);
	});

	it("preserves all byte values", () => {
		// Test all 256 possible byte values
		const original = new Uint8Array(256);
		for (let i = 0; i < 256; i++) {
			original[i] = i;
		}
		const encoded = arrayBufferToBase64url(original.buffer);
		const decoded = base64urlToArrayBuffer(encoded);
		expect([...new Uint8Array(decoded)]).toEqual([...original]);
	});

	it("preserves typical WebAuthn credential ID length (64 bytes)", () => {
		const original = new Uint8Array(64);
		for (let i = 0; i < 64; i++) {
			original[i] = (i * 7 + 13) % 256;
		}
		const encoded = arrayBufferToBase64url(original.buffer);
		const decoded = base64urlToArrayBuffer(encoded);
		expect([...new Uint8Array(decoded)]).toEqual([...original]);
	});
});
