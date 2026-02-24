import { describe, expect, it } from "vitest";
import { arrayBufferToBase64url } from "../../src/utils/base64url";
import {
	assertionToJson,
	credentialCreationToJson,
	toPublicKeyCreationOptions,
	toPublicKeyRequestOptions,
} from "../../src/utils/webauthn";

describe("utils/webauthn", () => {
	it("converts request options from base64url strings", () => {
		const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
		const credId = arrayBufferToBase64url(new Uint8Array([4, 5, 6]).buffer);
		const converted = toPublicKeyRequestOptions({
			publicKey: {
				challenge,
				allowCredentials: [{ id: credId, type: "public-key" }],
			},
		});

		expect(converted.challenge).toBeInstanceOf(ArrayBuffer);
		expect(
			(converted.allowCredentials?.[0].id as ArrayBuffer).byteLength
		).toBeGreaterThan(0);
	});

	it("leaves request options untouched when values are already non-string", () => {
		const challenge = new Uint8Array([1, 2, 3]).buffer;
		const credentialId = new Uint8Array([4, 5, 6]).buffer;
		const converted = toPublicKeyRequestOptions({
			challenge,
			allowCredentials: [{ id: credentialId, type: "public-key" }],
		});

		expect(converted.challenge).toBe(challenge);
		expect(converted.allowCredentials?.[0].id).toBe(credentialId);
	});

	it("converts creation options from base64url strings", () => {
		const challenge = arrayBufferToBase64url(new Uint8Array([9, 8, 7]).buffer);
		const userId = arrayBufferToBase64url(new Uint8Array([6, 5, 4]).buffer);
		const excludeId = arrayBufferToBase64url(new Uint8Array([3, 2, 1]).buffer);
		const converted = toPublicKeyCreationOptions({
			challenge,
			user: { id: userId, name: "test", displayName: "Test User" },
			excludeCredentials: [{ id: excludeId, type: "public-key" }],
		});

		expect(converted.challenge).toBeInstanceOf(ArrayBuffer);
		expect((converted.user.id as ArrayBuffer).byteLength).toBeGreaterThan(0);
		expect(
			(converted.excludeCredentials?.[0].id as ArrayBuffer).byteLength
		).toBeGreaterThan(0);
	});

	it("keeps creation option fields when challenge/user/excludes are not strings", () => {
		const challenge = new Uint8Array([9, 8, 7]).buffer;
		const userId = new Uint8Array([6, 5, 4]).buffer;
		const excludeId = new Uint8Array([3, 2, 1]).buffer;
		const converted = toPublicKeyCreationOptions({
			publicKey: {
				challenge,
				user: { id: userId, name: "test", displayName: "Test User" },
				excludeCredentials: [{ id: excludeId, type: "public-key" }],
			},
		});

		expect(converted.challenge).toBe(challenge);
		expect(converted.user.id).toBe(userId);
		expect(converted.excludeCredentials?.[0].id).toBe(excludeId);
	});

	it("serializes assertion credentials into JSON-safe payload", () => {
		const rawId = new Uint8Array([10, 11, 12]).buffer;
		const response = {
			clientDataJSON: new Uint8Array([1, 1, 1]).buffer,
			authenticatorData: new Uint8Array([2, 2, 2]).buffer,
			signature: new Uint8Array([3, 3, 3]).buffer,
			userHandle: new Uint8Array([4, 4, 4]).buffer,
		} as AuthenticatorAssertionResponse;
		const credential = {
			id: "credential-id",
			rawId,
			type: "public-key",
			response,
			getClientExtensionResults: () => ({ appid: true }),
		} as unknown as PublicKeyCredential;

		const out = assertionToJson(credential);
		expect(out.id).toBe("credential-id");
		expect(out.rawId).toBe(arrayBufferToBase64url(rawId));
		expect(out.response.userHandle).toBe(
			arrayBufferToBase64url(response.userHandle!)
		);
		expect(out.clientExtensionResults).toEqual({ appid: true });
	});

	it("serializes assertion credentials with null user handle", () => {
		const rawId = new Uint8Array([1, 2, 3]).buffer;
		const response = {
			clientDataJSON: new Uint8Array([1]).buffer,
			authenticatorData: new Uint8Array([2]).buffer,
			signature: new Uint8Array([3]).buffer,
			userHandle: null,
		} as AuthenticatorAssertionResponse;
		const credential = {
			id: "cred-null-user",
			rawId,
			type: "public-key",
			response,
			getClientExtensionResults: () => ({}),
		} as unknown as PublicKeyCredential;

		const out = assertionToJson(credential);
		expect(out.response.userHandle).toBeNull();
	});

	it("serializes attestation credentials into JSON-safe payload", () => {
		const rawId = new Uint8Array([20, 21, 22]).buffer;
		const response = {
			clientDataJSON: new Uint8Array([7, 7, 7]).buffer,
			attestationObject: new Uint8Array([8, 8, 8]).buffer,
		} as AuthenticatorAttestationResponse;
		const credential = {
			id: "new-credential-id",
			rawId,
			type: "public-key",
			response,
			getClientExtensionResults: () => ({}),
		} as unknown as PublicKeyCredential;

		const out = credentialCreationToJson(credential);
		expect(out.id).toBe("new-credential-id");
		expect(out.rawId).toBe(arrayBufferToBase64url(rawId));
		expect(out.response.attestationObject).toBe(
			arrayBufferToBase64url(response.attestationObject)
		);
	});

	it("serializes attestation credentials without attestation object", () => {
		const rawId = new Uint8Array([20, 21, 22]).buffer;
		const response = {
			clientDataJSON: new Uint8Array([7, 7, 7]).buffer,
			attestationObject: undefined,
		} as unknown as AuthenticatorAttestationResponse;
		const credential = {
			id: "no-attestation-object",
			rawId,
			type: "public-key",
			response,
			getClientExtensionResults: () => ({}),
		} as unknown as PublicKeyCredential;

		const out = credentialCreationToJson(credential);
		expect(out.response.attestationObject).toBeUndefined();
	});
});
