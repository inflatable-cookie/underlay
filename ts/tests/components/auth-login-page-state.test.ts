import { describe, expect, it } from "vitest";
import {
	getPostTwoFactorOutcome,
	resolveEmailFallbackOutcome,
	resolvePasswordLoginOutcome,
} from "../../src/components/auth/login-page-state";

describe("components/auth/login-page-state", () => {
	it("resolves password login outcomes", () => {
		expect(resolvePasswordLoginOutcome(undefined, "u@example.com")).toEqual({ kind: "complete" });
		expect(resolvePasswordLoginOutcome({ complete: true } as any, "u@example.com")).toEqual({ kind: "complete" });
		expect(resolvePasswordLoginOutcome({ complete: false } as any, "u@example.com")).toEqual({ kind: "noop" });

		expect(
			resolvePasswordLoginOutcome(
				{
					requiresTwoFactor: true,
					loginStateId: "ls1",
					isEmailVerification: true,
					email: "twofa@example.com",
				} as any,
				"fallback@example.com"
			)
		).toEqual({
			kind: "2fa",
			loginStateId: "ls1",
			isEmailVerification: true,
			hadTotpConfigured: false,
			usedEmailFallback: false,
			twoFactorEmail: "twofa@example.com",
		});

		expect(
			resolvePasswordLoginOutcome(
				{
					requiresTwoFactor: true,
					loginStateId: "ls2",
				} as any,
				"fallback@example.com"
			)
		).toEqual({
			kind: "2fa",
			loginStateId: "ls2",
			isEmailVerification: false,
			hadTotpConfigured: true,
			usedEmailFallback: false,
			twoFactorEmail: "fallback@example.com",
		});
	});

	it("resolves post-2fa and email fallback outcomes", () => {
		expect(getPostTwoFactorOutcome(true, true, false)).toBe("setup-prompt");
		expect(getPostTwoFactorOutcome(true, false, true)).toBe("setup-prompt");
		expect(getPostTwoFactorOutcome(true, false, false)).toBe("complete");
		expect(getPostTwoFactorOutcome(false, true, true)).toBe("complete");

		expect(resolveEmailFallbackOutcome({ loginStateId: "ls2", email: "x@example.com" } as any)).toEqual({
			loginStateId: "ls2",
			twoFactorEmail: "x@example.com",
			isEmailVerification: true,
			usedEmailFallback: true,
		});
	});
});
