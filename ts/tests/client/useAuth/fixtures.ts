import { vi } from "vitest";

export function makeSession(id = "s1") {
	return {
		user: {
			id: "u1",
			email: "user@example.com",
			displayName: "User",
			status: "active" as const,
			createdAt: "2026-01-01T00:00:00Z",
			updatedAt: "2026-01-01T00:00:00Z",
		},
		session: {
			id,
			userId: "u1",
			accessTokenFingerprint: "a",
			refreshTokenFingerprint: "r",
			accessTokenExpiresAt: "2026-01-01T01:00:00Z",
			refreshTokenExpiresAt: "2026-01-02T01:00:00Z",
			createdAt: "2026-01-01T00:00:00Z",
			lastUsedAt: "2026-01-01T00:00:00Z",
			ipAddress: null,
			userAgent: null,
			status: "active" as const,
			revocationReason: null,
			revokedAt: null,
		},
		accessToken: `access-${id}`,
		refreshToken: `refresh-${id}`,
	};
}

export function makeDeps() {
	const commands = {
		session: vi.fn(),
		refresh: vi.fn(),
		register: vi.fn(),
		loginWithPassword: vi.fn(),
		loginWithPasskey: vi.fn(),
		logout: vi.fn(),
	};

	const tokenStore = {
		setAccessToken: vi.fn(async () => {}),
		setRefreshToken: vi.fn(async () => {}),
		clear: vi.fn(async () => {}),
		getAccessToken: vi.fn(() => null),
		getRefreshToken: vi.fn(() => null),
	};

	return { commands, tokenStore };
}
