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
});
