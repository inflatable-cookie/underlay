// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from "vitest";
import {
	usePasskeyAuthentication,
	usePasskeyRegistration,
} from "../../src/patterns/passkey.svelte";
import { arrayBufferToBase64url } from "../../src/utils/base64url";
import { createEffectRoot } from "./effect-root.svelte";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

describe("patterns/passkey.svelte.ts (compiled)", () => {
	const originalNavigator = globalThis.navigator;
	const originalPublicKeyCredential = globalThis.PublicKeyCredential;

	afterEach(() => {
		vi.unstubAllGlobals();
		if (originalNavigator) {
			vi.stubGlobal("navigator", originalNavigator);
		}
		if (originalPublicKeyCredential) {
			vi.stubGlobal("PublicKeyCredential", originalPublicKeyCredential);
		}
	});

	it("guards WebAuthn registration against double-click re-entrancy", async () => {
		const gate = deferred<PublicKeyCredential>();
		const create = vi.fn(() => gate.promise);
		vi.stubGlobal("navigator", {
			credentials: { create, get: vi.fn() },
		});

		const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
		const userId = arrayBufferToBase64url(new Uint8Array([4, 5, 6]).buffer);
		const onStart = vi.fn(async () => ({
			challenge,
			user: { id: userId, name: "user@example.com", displayName: "User" },
		}));
		const onFinish = vi.fn(async () => undefined);

		let registration!: ReturnType<typeof usePasskeyRegistration>;
		const dispose = createEffectRoot(() => {
			registration = usePasskeyRegistration({ onStart, onFinish });
		});

		try {
			const first = registration.start();
			const second = registration.start();
			expect(registration.loading).toBe(true);

			gate.resolve({
				id: "cred-1",
				rawId: new Uint8Array([7, 8, 9]).buffer,
				type: "public-key",
				response: {
					clientDataJSON: new Uint8Array([10]).buffer,
					attestationObject: new Uint8Array([11]).buffer,
				},
				getClientExtensionResults: () => ({}),
			} as unknown as PublicKeyCredential);

			await Promise.all([first, second]);

			expect(onStart).toHaveBeenCalledTimes(1);
			expect(create).toHaveBeenCalledTimes(1);
			expect(onFinish).toHaveBeenCalledTimes(1);
			expect(registration.loading).toBe(false);
		} finally {
			dispose();
		}
	});

	it("guards WebAuthn authentication against double-click re-entrancy", async () => {
		const gate = deferred<PublicKeyCredential>();
		const get = vi.fn(() => gate.promise);
		vi.stubGlobal("navigator", {
			credentials: { create: vi.fn(), get },
		});

		const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
		const onStart = vi.fn(async () => ({ challenge }));
		const onFinish = vi.fn(async () => undefined);

		let authentication!: ReturnType<typeof usePasskeyAuthentication>;
		const dispose = createEffectRoot(() => {
			authentication = usePasskeyAuthentication({ onStart, onFinish });
		});

		try {
			const first = authentication.start();
			const second = authentication.start();
			expect(authentication.loading).toBe(true);

			gate.resolve({
				id: "cred-1",
				rawId: new Uint8Array([7, 8, 9]).buffer,
				type: "public-key",
				response: {
					clientDataJSON: new Uint8Array([10]).buffer,
					authenticatorData: new Uint8Array([11]).buffer,
					signature: new Uint8Array([12]).buffer,
					userHandle: null,
				},
				getClientExtensionResults: () => ({}),
			} as unknown as PublicKeyCredential);

			await Promise.all([first, second]);

			expect(onStart).toHaveBeenCalledTimes(1);
			expect(get).toHaveBeenCalledTimes(1);
			expect(onFinish).toHaveBeenCalledTimes(1);
			expect(authentication.loading).toBe(false);
		} finally {
			dispose();
		}
	});

	it("releases registration and authentication guards after a synchronous onStart throw so retry works", async () => {
		const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
		const userId = arrayBufferToBase64url(new Uint8Array([4, 5, 6]).buffer);

		const create = vi.fn(
			async () =>
				({
					id: "cred-reg",
					rawId: new Uint8Array([7, 8, 9]).buffer,
					type: "public-key",
					response: {
						clientDataJSON: new Uint8Array([10]).buffer,
						attestationObject: new Uint8Array([11]).buffer,
					},
					getClientExtensionResults: () => ({}),
				}) as unknown as PublicKeyCredential,
		);
		const get = vi.fn(
			async () =>
				({
					id: "cred-auth",
					rawId: new Uint8Array([7, 8, 9]).buffer,
					type: "public-key",
					response: {
						clientDataJSON: new Uint8Array([10]).buffer,
						authenticatorData: new Uint8Array([11]).buffer,
						signature: new Uint8Array([12]).buffer,
						userHandle: null,
					},
					getClientExtensionResults: () => ({}),
				}) as unknown as PublicKeyCredential,
		);
		vi.stubGlobal("navigator", {
			credentials: { create, get },
		});
		vi.stubGlobal("PublicKeyCredential", class {});

		let regAttempts = 0;
		const regOnStart = vi.fn(() => {
			regAttempts += 1;
			if (regAttempts === 1) {
				throw new Error("sync reg start failure");
			}
			return Promise.resolve({
				challenge,
				user: { id: userId, name: "user@example.com", displayName: "User" },
			});
		});
		const regOnFinish = vi.fn(async () => undefined);

		let authAttempts = 0;
		const authOnStart = vi.fn(() => {
			authAttempts += 1;
			if (authAttempts === 1) {
				throw new Error("sync auth start failure");
			}
			return Promise.resolve({ challenge });
		});
		const authOnFinish = vi.fn(async () => undefined);

		let registration!: ReturnType<typeof usePasskeyRegistration>;
		let authentication!: ReturnType<typeof usePasskeyAuthentication>;
		const dispose = createEffectRoot(() => {
			registration = usePasskeyRegistration({ onStart: regOnStart, onFinish: regOnFinish });
			authentication = usePasskeyAuthentication({ onStart: authOnStart, onFinish: authOnFinish });
		});

		try {
			await expect(registration.start()).rejects.toMatchObject({
				message: "sync reg start failure",
			});
			expect(registration.loading).toBe(false);
			await registration.start();
			expect(regOnStart).toHaveBeenCalledTimes(2);
			expect(create).toHaveBeenCalledTimes(1);
			expect(regOnFinish).toHaveBeenCalledTimes(1);

			await expect(authentication.start()).rejects.toMatchObject({
				message: "sync auth start failure",
			});
			expect(authentication.loading).toBe(false);
			await authentication.start();
			expect(authOnStart).toHaveBeenCalledTimes(2);
			expect(get).toHaveBeenCalledTimes(1);
			expect(authOnFinish).toHaveBeenCalledTimes(1);
		} finally {
			dispose();
		}
	});

	it("blocks synchronous re-entrancy triggered from inside onStart for both registration and authentication", async () => {
		const regGate = deferred<PublicKeyCredential>();
		const authGate = deferred<PublicKeyCredential>();
		const create = vi.fn(() => regGate.promise);
		const get = vi.fn(() => authGate.promise);
		vi.stubGlobal("navigator", {
			credentials: { create, get },
		});
		vi.stubGlobal("PublicKeyCredential", class {});

		const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
		const userId = arrayBufferToBase64url(new Uint8Array([4, 5, 6]).buffer);

		let registration!: ReturnType<typeof usePasskeyRegistration>;
		let authentication!: ReturnType<typeof usePasskeyAuthentication>;
		let innerRegPromise: Promise<void> | undefined;
		let innerAuthPromise: Promise<void> | undefined;

		const regOnStart = vi.fn(() => {
			innerRegPromise = registration.start();
			return Promise.resolve({
				challenge,
				user: { id: userId, name: "user@example.com", displayName: "User" },
			});
		});
		const regOnFinish = vi.fn(async () => undefined);

		const authOnStart = vi.fn(() => {
			innerAuthPromise = authentication.start();
			return Promise.resolve({ challenge });
		});
		const authOnFinish = vi.fn(async () => undefined);

		const dispose = createEffectRoot(() => {
			registration = usePasskeyRegistration({ onStart: regOnStart, onFinish: regOnFinish });
			authentication = usePasskeyAuthentication({ onStart: authOnStart, onFinish: authOnFinish });
		});

		try {
			let innerRegSettled = false;
			const outerReg = registration.start();
			await Promise.resolve();
			expect(innerRegPromise).toBeDefined();
			void innerRegPromise!.then(() => {
				innerRegSettled = true;
			});
			await Promise.resolve();
			expect(innerRegSettled).toBe(false);
			expect(registration.loading).toBe(true);

			regGate.resolve({
				id: "cred-reg",
				rawId: new Uint8Array([7, 8, 9]).buffer,
				type: "public-key",
				response: {
					clientDataJSON: new Uint8Array([10]).buffer,
					attestationObject: new Uint8Array([11]).buffer,
				},
				getClientExtensionResults: () => ({}),
			} as unknown as PublicKeyCredential);
			await Promise.all([outerReg, innerRegPromise]);

			expect(innerRegSettled).toBe(true);
			expect(regOnStart).toHaveBeenCalledTimes(1);
			expect(create).toHaveBeenCalledTimes(1);
			expect(regOnFinish).toHaveBeenCalledTimes(1);
			expect(registration.loading).toBe(false);

			let innerAuthSettled = false;
			const outerAuth = authentication.start();
			await Promise.resolve();
			expect(innerAuthPromise).toBeDefined();
			void innerAuthPromise!.then(() => {
				innerAuthSettled = true;
			});
			await Promise.resolve();
			expect(innerAuthSettled).toBe(false);
			expect(authentication.loading).toBe(true);

			authGate.resolve({
				id: "cred-auth",
				rawId: new Uint8Array([7, 8, 9]).buffer,
				type: "public-key",
				response: {
					clientDataJSON: new Uint8Array([10]).buffer,
					authenticatorData: new Uint8Array([11]).buffer,
					signature: new Uint8Array([12]).buffer,
					userHandle: null,
				},
				getClientExtensionResults: () => ({}),
			} as unknown as PublicKeyCredential);
			await Promise.all([outerAuth, innerAuthPromise]);

			expect(innerAuthSettled).toBe(true);
			expect(authOnStart).toHaveBeenCalledTimes(1);
			expect(get).toHaveBeenCalledTimes(1);
			expect(authOnFinish).toHaveBeenCalledTimes(1);
			expect(authentication.loading).toBe(false);
		} finally {
			dispose();
		}
	});
});
