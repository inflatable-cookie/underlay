import { describe, expect, it, vi } from "vitest";

type AuthDataModule = typeof import("../../src/patterns/authenticated-data.svelte");

async function loadAuthDataModule(options?: {
	globalConfig?: Record<string, unknown> | null;
	captureEffects?: boolean;
}): Promise<{ mod: AuthDataModule; runEffects: () => void; dispose: () => void }> {
	vi.resetModules();
	const effects: Array<() => void | (() => void)> = [];
	const teardowns = new Map<(typeof effects)[number], () => void>();
	const runEffect = (effect: (typeof effects)[number]) => {
		const teardown = effect();
		if (teardown) teardowns.set(effect, teardown);
	};

	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);
	(globalThis as any).$effect = (fn: (typeof effects)[number]) => {
		effects.push(fn);
		runEffect(fn);
	};

	vi.doMock("../../src/patterns/auth", () => ({
		getAuthConfig: () => options?.globalConfig ?? null,
	}));

	const mod = await import("../../src/patterns/authenticated-data.svelte");
	return {
		mod,
		runEffects: () => {
			if (!options?.captureEffects) return;
			for (const effect of effects) {
				if (!teardowns.has(effect)) runEffect(effect);
			}
		},
		dispose: () => {
			for (const teardown of teardowns.values()) teardown();
			teardowns.clear();
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

	it("settles the initial request while a coalesced refetch is still running", async () => {
		const { mod } = await loadAuthDataModule();
		let finishRefetch: (value: string) => void = () => {};
		const refetchResult = new Promise<string>((resolve) => {
			finishRefetch = resolve;
		});
		const fetcher = vi.fn<() => Promise<string>>()
			.mockResolvedValueOnce("initial")
			.mockReturnValueOnce(refetchResult);
		const data = mod.useAuthenticatedData(fetcher, { getToken: () => "token" });

		const initial = data.tryFetch(false, { id: "u1" });
		const queued = data.refetch();
		const coalesced = data.refetch();
		await initial;
		expect(data.data).toBe("initial");
		expect(data.refetching).toBe(true);
		expect(fetcher).toHaveBeenCalledTimes(2);

		finishRefetch("updated");
		await Promise.all([queued, coalesced]);
		expect(data.data).toBe("updated");
	});

	it("surfaces rejected success callbacks without replaying the request", async () => {
		const { mod } = await loadAuthDataModule();
		const fetcher = vi.fn(async () => "loaded");
		const onRefresh = vi.fn(async () => "new-token");
		const data = mod.useAuthenticatedData(fetcher, {
			getToken: () => "token",
			onRefresh,
			onSuccess: async () => {
				throw Object.assign(new Error("Post-load action failed"), { status: 401 });
			},
		});

		await data.tryFetch(false, { id: "u1" });
		expect(data.data).toBe("loaded");
		expect(data.error).toBe("Post-load action failed");
		expect(onRefresh).not.toHaveBeenCalled();
		expect(fetcher).toHaveBeenCalledTimes(1);
	});

	it("settles queued work on disposal and ignores late fetch results", async () => {
		const { mod, dispose } = await loadAuthDataModule();
		let finishFetch: (value: string) => void = () => {};
		const fetcher = vi.fn(() => new Promise<string>((resolve) => {
			finishFetch = resolve;
		}));
		const onSuccess = vi.fn();
		const data = mod.useAuthenticatedData(fetcher, { getToken: () => "token", onSuccess });

		const active = data.tryFetch(false, { id: "u1" });
		const queued = data.refetch();
		dispose();
		await queued;
		finishFetch("late result");
		await active;
		await data.refetch();

		expect(onSuccess).not.toHaveBeenCalled();
		expect(data.data).toBeUndefined();
		expect(fetcher).toHaveBeenCalledTimes(1);
	});
});
