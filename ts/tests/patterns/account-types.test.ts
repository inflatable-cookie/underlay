import { describe, it, expect } from "vitest";
import { deriveDisplayName, getEffectiveDisplayName } from "../../src/patterns/account-types";

describe("deriveDisplayName", () => {
	it("returns null for empty values", () => {
		expect(deriveDisplayName(null)).toBeNull();
		expect(deriveDisplayName(undefined)).toBeNull();
		expect(deriveDisplayName("   ")).toBeNull();
	});

	it("uses first word for space-delimited names", () => {
		expect(deriveDisplayName("Alice Smith")).toBe("Alice");
		expect(deriveDisplayName("  María José García  ")).toBe("María");
	});

	it("returns full CJK names without splitting", () => {
		expect(deriveDisplayName("李明")).toBe("李明");
		expect(deriveDisplayName("山田 太郎")).toBe("山田 太郎");
		expect(deriveDisplayName("홍길동")).toBe("홍길동");
	});
});

describe("getEffectiveDisplayName", () => {
	it("prefers explicit displayName", () => {
		expect(getEffectiveDisplayName({ displayName: "Clay", fullName: "Clay Jones" })).toBe("Clay");
	});

	it("falls back to derived fullName", () => {
		expect(getEffectiveDisplayName({ fullName: "Alice Smith" })).toBe("Alice");
	});

	it("falls back to email username when profile name data is missing", () => {
		expect(getEffectiveDisplayName({}, "user@example.com")).toBe("user");
		expect(getEffectiveDisplayName(undefined, "no-at-symbol")).toBe("no-at-symbol");
	});

	it("returns 'User' as final fallback", () => {
		expect(getEffectiveDisplayName(null)).toBe("User");
		expect(getEffectiveDisplayName({ fullName: null, displayName: null }, null)).toBe("User");
	});
});
