import { vi } from "vitest";

export function createCookiesMock() {
	const jar = new Map<string, string>();
	const deleted: Array<{ key: string; options: Record<string, unknown> }> = [];
	const setCalls: Array<{ key: string; value: string; options: Record<string, unknown> }> = [];

	return {
		jar,
		deleted,
		setCalls,
		get: vi.fn((key: string) => jar.get(key)),
		set: vi.fn((key: string, value: string, options: Record<string, unknown>) => {
			jar.set(key, value);
			setCalls.push({ key, value, options });
		}),
		delete: vi.fn((key: string, options: Record<string, unknown>) => {
			jar.delete(key);
			deleted.push({ key, options });
		}),
	};
}

export const authRoutes = {
	register: "/register",
	loginPassword: "/login/password",
	loginPasskey: "/login/passkey",
	logout: "/logout",
	refresh: "/refresh",
	session: "/session",
};
