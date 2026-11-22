import { writable, type Readable } from "svelte/store";

import { UnderlayHttpError } from "./errors";
import type {
	AuthCommands,
	AuthSession,
	PasswordAuthParams,
	PassKeyAuthParams,
	RegisterParams,
	SessionInfo,
} from "./auth";
import { isTransientRefreshError, type TokenStore } from "./http";

export type AuthStatus = "unknown" | "anonymous" | "authenticated";

export type AuthState = {
	status: AuthStatus;
	session: SessionInfo | null;
	loading: boolean;
	error: UnderlayHttpError | null;
};

export type AuthStore = {
	state: Readable<AuthState>;

	init: () => Promise<void>;
	register: (params: RegisterParams) => Promise<AuthSession>;
	loginWithPassword: (params: PasswordAuthParams & { code?: string }) => Promise<AuthSession>;
	loginWithPasskey: (params: PassKeyAuthParams) => Promise<AuthSession>;
	refresh: () => Promise<AuthSession>;
	logout: () => Promise<void>;
};

export interface CreateAuthStoreOptions {
	commands: AuthCommands;
	tokenStore: TokenStore;

	/**
	 * If true, `init()` will attempt a refresh on 401.
	 *
	 * Defaults to true.
	 */
	refreshOnUnauthorized?: boolean;
}

export function createAuthStore(options: CreateAuthStoreOptions): AuthStore {
	const refreshOnUnauthorized = options.refreshOnUnauthorized ?? true;

	const state = writable<AuthState>({
		status: "unknown",
		session: null,
		loading: false,
		error: null,
	});

	function stripTokens(info: SessionInfo): SessionInfo {
		const {
			accessToken: _accessToken,
			refreshToken: _refreshToken,
			...rest
		} = info as SessionInfo & Partial<Pick<AuthSession, "accessToken" | "refreshToken">>;
		return rest;
	}

	function setAuthenticated(session: SessionInfo) {
		state.set({
			status: "authenticated",
			session: stripTokens(session),
			loading: false,
			error: null,
		});
	}

	let authEpoch = 0;
	let tokenMutation = Promise.resolve();

	// Serialize writes and clears so a setter already running when logout starts
	// must finish before logout clears the store.
	function mutateTokens(epoch: number, mutate: () => Promise<void>): Promise<void> {
		if (epoch !== authEpoch) return Promise.resolve();
		const pending = tokenMutation.then(async () => {
			if (epoch === authEpoch) await mutate();
		});
		tokenMutation = pending.catch(() => {});
		return pending;
	}

	async function acceptSession(epoch: number, session: AuthSession) {
		await mutateTokens(epoch, async () => {
			await options.tokenStore.setAccessToken(session.accessToken);
			if (epoch === authEpoch) await options.tokenStore.setRefreshToken(session.refreshToken);
		});
		if (epoch === authEpoch) setAuthenticated(session);
	}

	async function clearTokens(epoch: number) {
		await mutateTokens(epoch, async () => options.tokenStore.clear());
	}

	function setAnonymous(error: UnderlayHttpError | null = null) {
		state.set({ status: "anonymous", session: null, loading: false, error });
	}

	let initInFlight: Promise<void> | null = null;
	let refreshInFlight: Promise<AuthSession> | null = null;
	// Retain an overlapping refresh until init's session check finishes. Its
	// 401 may arrive after the refresh has already rotated the tokens.
	let refreshDuringInit: Promise<AuthSession> | null = null;

	async function runInit(epoch: number): Promise<void> {
		state.update((s) => ({ ...s, loading: true, error: null }));

		try {
			// Session GET carries no token material; tokens only move on
			// login/refresh. Do not write to the token store here.
			const session = await options.commands.session();
			if (epoch === authEpoch) setAuthenticated(session);
		} catch (err) {
			if (epoch !== authEpoch) return;
			if (err instanceof UnderlayHttpError && err.status === 401) {
				if (refreshOnUnauthorized) {
					try {
						await (refreshDuringInit ?? refresh());
					} catch (refreshErr) {
						if (epoch !== authEpoch) return;
						// The shared refresh already clears tokens and state on a 401.
						if (refreshErr instanceof UnderlayHttpError && refreshErr.status === 401) {
							return;
						}
						// Match the HTTP client: a 5xx or network failure may recover, so
						// keep stored tokens instead of signing the user out.
						if (!isTransientRefreshError(refreshErr)) {
							await clearTokens(epoch);
						}
						if (epoch === authEpoch) {
							setAnonymous(refreshErr instanceof UnderlayHttpError ? refreshErr : null);
						}
					}
					return;
				}

				await clearTokens(epoch);
				if (epoch === authEpoch) setAnonymous(err);
				return;
			}

			setAnonymous(err instanceof UnderlayHttpError ? err : null);
		}
	}

	function init(): Promise<void> {
		if (initInFlight !== null) return initInFlight;
		const epoch = authEpoch;
		refreshDuringInit = refreshInFlight;
		const pending = runInit(epoch).finally(() => {
			if (initInFlight !== pending) return;
			refreshDuringInit = null;
			initInFlight = null;
		});
		if (epoch === authEpoch) initInFlight = pending;
		return pending;
	}

	async function authenticate(command: () => Promise<AuthSession>): Promise<AuthSession> {
		const epoch = authEpoch;
		state.update((s) => ({ ...s, loading: true, error: null }));

		try {
			const session = await command();
			await acceptSession(epoch, session);
			return session;
		} catch (err) {
			if (epoch === authEpoch) setError(err);
			throw err;
		}
	}

	function setError(err: unknown) {
		const error = err instanceof UnderlayHttpError ? err : null;
		state.update((s) => ({ ...s, loading: false, error }));
	}

	function loginWithPassword(params: PasswordAuthParams & { code?: string }): Promise<AuthSession> {
		return authenticate(() =>
			options.commands.loginWithPassword({
				email: params.email,
				password: params.password,
				...(params.code ? { code: params.code } : {}),
			}),
		);
	}

	async function runRefresh(epoch: number): Promise<AuthSession> {
		state.update((s) => ({ ...s, loading: true, error: null }));

		try {
			const session = await options.commands.refresh();
			await acceptSession(epoch, session);
			return session;
		} catch (err) {
			if (epoch !== authEpoch) throw err;
			if (err instanceof UnderlayHttpError && err.status === 401) {
				await clearTokens(epoch);
				if (epoch === authEpoch) setAnonymous(err);
			} else {
				setError(err);
			}
			throw err;
		}
	}

	function refresh(): Promise<AuthSession> {
		if (refreshInFlight !== null) return refreshInFlight;
		const epoch = authEpoch;
		const pending = runRefresh(epoch).finally(() => {
			if (refreshInFlight === pending) refreshInFlight = null;
		});
		if (epoch === authEpoch) {
			refreshInFlight = pending;
			if (initInFlight !== null) {
				refreshDuringInit = pending;
			}
		}
		return pending;
	}

	async function logout(): Promise<void> {
		const epoch = ++authEpoch;
		initInFlight = null;
		refreshInFlight = null;
		refreshDuringInit = null;
		state.update((s) => ({ ...s, loading: true, error: null }));

		try {
			await options.commands.logout();
		} finally {
			try {
				await clearTokens(epoch);
			} finally {
				if (epoch === authEpoch) setAnonymous(null);
			}
		}
	}

	return {
		state,
		init,
		register: (params: RegisterParams) => authenticate(() => options.commands.register(params)),
		loginWithPassword,
		loginWithPasskey: (params: PassKeyAuthParams) =>
			authenticate(() => options.commands.loginWithPasskey(params)),
		refresh,
		logout,
	};
}
