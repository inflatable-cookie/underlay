import { describe, expect, it, vi } from "vitest";

type AuthDataModule = typeof import("../../src/patterns/authenticated-data.svelte");

async function loadAuthDataModule(options?: {
	globalConfig?: Record<string, unknown> | null;
	captureEffects?: boolean;
}): Promise<{ mod: AuthDataModule; runEffects: () => void }> {
	vi.resetModules();
	const effects: Array<() => void> = [];

	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);
	(globalThis as any).$effect = (fn: () => void) => {
		effects.push(fn);
		fn();
	};

	vi.doMock("../../src/patterns/auth", () => ({
		getAuthConfig: () => options?.globalConfig ?? null,
	}));

	const mod = await import("../../src/patterns/authenticated-data.svelte");
	return {
		mod,
		runEffects: () => {
			if (!options?.captureEffects) return;
			for (const effect of effects) effect();
		},
	};
}

async function flush(): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 0));
}

describe("patterns/authenticated-data.svelte.ts", () => {
	it("defers missing getToken to the fetch path so setup stays SSR-safe", async () => {
		const { mod } = await loadAuthDataModule();
		const data = mod.useAuthenticatedData(
			async () => ({ ok: true }),
			{}
		);
		expect(data.error).toBeNull();

		await data.tryFetch(false, { id: "u1" });
		expect(data.error).toMatch(/getToken is required/);
		expect(data.loading).toBe(false);
	});

	it("gates fetch by auth readiness and supports refetch lifecycle", async () => {
		const { mod } = await loadAuthDataModule();
		const fetcher = vi.fn(async (_fetch: typeof fetch, token: string) => ({ token }));
		const onSuccess = vi.fn();

		let token: string | null = null;
		const data = mod.useAuthenticatedData(fetcher, {
			getToken: () => token,
			defaultValue: { token: "default" },
			onSuccess,
		});

		await data.tryFetch(true, { id: "u1" });
		await data.tryFetch(false, null);
		await data.tryFetch(false, { id: "u1" });
		expect(fetcher).not.toHaveBeenCalled();

		token = "tok-1";
		await data.tryFetch(false, { id: "u1" });
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(data.data).toEqual({ token: "tok-1" });
		expect(data.loading).toBe(false);
		expect(data.refetching).toBe(false);
		expect(data.error).toBeNull();
		expect(onSuccess).toHaveBeenCalledTimes(1);

		await data.refetch();
		expect(fetcher).toHaveBeenCalledTimes(2);
		expect(onSuccess).toHaveBeenCalledTimes(2);
	});

	it("deduplicates in-flight requests and handles refresh + retry outcomes", async () => {
		const { mod } = await loadAuthDataModule();

		let release!: () => void;
		const gate = new Promise<void>((resolve) => {
			release = resolve;
		});
		const fetcher = vi
			.fn()
			.mockImplementationOnce(async (_fetch: typeof fetch, token: string) => {
				await gate;
				return { token };
			})
			.mockRejectedValueOnce({ status: 401 })
			.mockResolvedValueOnce({ token: "fresh" })
			.mockRejectedValueOnce({ status: 401 })
			.mockRejectedValueOnce(new Error("retry failed"))
			.mockRejectedValueOnce({ status: 401 })
			.mockRejectedValueOnce(new Error("original unauthorized"))
			.mockRejectedValueOnce("unknown failure");

		let token = "tok";
		const onRefresh = vi
			.fn()
			.mockResolvedValueOnce("new-token")
			.mockResolvedValueOnce("new-token")
			.mockResolvedValueOnce(null);

		const data = mod.useAuthenticatedData(fetcher, {
			getToken: () => token,
			onRefresh,
		});

		const p1 = data.tryFetch(false, { id: "u1" });
		const p2 = data.refetch();
		release();
		await Promise.all([p1, p2]);
		expect(fetcher).toHaveBeenCalledTimes(3);
		expect(onRefresh).toHaveBeenCalledTimes(1);
		expect(data.data).toEqual({ token: "fresh" });

		await data.refetch();
		expect(onRefresh).toHaveBeenCalledTimes(2);
		expect(data.error).toBe("retry failed");

		await data.refetch();
		expect(onRefresh).toHaveBeenCalledTimes(3);
		expect(data.error).toBe("Session expired");

		token = "tok-2";
		await data.refetch();
		expect(data.error).toBe("original unauthorized");
	});

	it("supports auto-fetch getters and query-key driven refetch", async () => {
		let authLoading = true;
		let currentUser: unknown = null;
		let queryKey = "a=1";

		const { mod, runEffects } = await loadAuthDataModule({
			captureEffects: true,
			globalConfig: {
				getToken: () => "token",
				getAuthLoading: () => authLoading,
				getCurrentUser: () => currentUser,
			},
		});

		const fetcher = vi.fn(async () => ({ ok: true }));
		const onSuccess = vi.fn();
		mod.useAuthenticatedData(fetcher, {
			onSuccess,
			queryKey: () => queryKey,
		});

		await flush();
		expect(fetcher).toHaveBeenCalledTimes(0);

		authLoading = false;
		currentUser = { id: "u1" };
		runEffects();
		await flush();
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(onSuccess).toHaveBeenCalledTimes(1);

		queryKey = "a=2";
		runEffects();
		await flush();
		expect(fetcher).toHaveBeenCalledTimes(2);
	});
});
