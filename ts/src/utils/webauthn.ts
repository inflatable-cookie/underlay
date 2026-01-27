/**
 * WebAuthn credential helpers for converting between API formats.
 *
 * The WebAuthn API uses ArrayBuffer for binary data, but JSON serialization
 * requires Base64URL strings. These helpers handle the conversion.
 */

import { base64urlToArrayBuffer, arrayBufferToBase64url } from "./base64url.js";

/**
 * Convert server-provided options to PublicKeyCredentialRequestOptions.
 *
 * Used for passkey authentication (login).
 *
 * @param options - Options object from the server (may have Base64URL strings)
 * @returns PublicKeyCredentialRequestOptions with ArrayBuffer values
 */
export function toPublicKeyRequestOptions(
  options: unknown
): PublicKeyCredentialRequestOptions {
  const raw = (options as { publicKey?: unknown })?.publicKey ?? options;
  const publicKey: Record<string, unknown> = { ...(raw as object) };

  if (typeof publicKey.challenge === "string") {
    publicKey.challenge = base64urlToArrayBuffer(publicKey.challenge);
  }

  if (Array.isArray(publicKey.allowCredentials)) {
    publicKey.allowCredentials = publicKey.allowCredentials.map(
      (cred: { id?: string | ArrayBuffer; [key: string]: unknown }) => ({
        ...cred,
        id:
          typeof cred.id === "string"
            ? base64urlToArrayBuffer(cred.id)
            : cred.id,
      })
    );
  }

  return publicKey as unknown as PublicKeyCredentialRequestOptions;
}

/**
 * Convert server-provided options to PublicKeyCredentialCreationOptions.
 *
 * Used for passkey registration.
 *
 * @param options - Options object from the server (may have Base64URL strings)
 * @returns PublicKeyCredentialCreationOptions with ArrayBuffer values
 */
export function toPublicKeyCreationOptions(
  options: unknown
): PublicKeyCredentialCreationOptions {
  const raw = (options as { publicKey?: unknown })?.publicKey ?? options;
  const publicKey: Record<string, unknown> = { ...(raw as object) };

  if (typeof publicKey.challenge === "string") {
    publicKey.challenge = base64urlToArrayBuffer(publicKey.challenge);
  }

  if (
    publicKey.user &&
    typeof (publicKey.user as { id?: string }).id === "string"
  ) {
    publicKey.user = {
      ...(publicKey.user as object),
      id: base64urlToArrayBuffer((publicKey.user as { id: string }).id),
    };
  }

  if (Array.isArray(publicKey.excludeCredentials)) {
    publicKey.excludeCredentials = publicKey.excludeCredentials.map(
      (cred: { id?: string | ArrayBuffer; [key: string]: unknown }) => ({
        ...cred,
        id:
          typeof cred.id === "string"
            ? base64urlToArrayBuffer(cred.id)
            : cred.id,
      })
    );
  }

  return publicKey as unknown as PublicKeyCredentialCreationOptions;
}

/**
 * Convert an authentication assertion to JSON for sending to the server.
 *
 * Used after navigator.credentials.get() for passkey login.
 *
 * @param cred - PublicKeyCredential from the browser
 * @returns JSON-serializable object with Base64URL strings
 */
export function assertionToJson(cred: PublicKeyCredential): {
  id: string;
  rawId: string;
  type: string;
  response: {
    clientDataJSON: string;
    authenticatorData: string;
    signature: string;
    userHandle: string | null;
  };
  clientExtensionResults: AuthenticationExtensionsClientOutputs;
} {
  const response = cred.response as AuthenticatorAssertionResponse;

  return {
    id: cred.id,
    rawId: arrayBufferToBase64url(cred.rawId),
    type: cred.type,
    response: {
      clientDataJSON: arrayBufferToBase64url(response.clientDataJSON),
      authenticatorData: arrayBufferToBase64url(response.authenticatorData),
      signature: arrayBufferToBase64url(response.signature),
      userHandle: response.userHandle
        ? arrayBufferToBase64url(response.userHandle)
        : null,
    },
    clientExtensionResults: cred.getClientExtensionResults(),
  };
}

/**
 * Convert a credential creation result to JSON for sending to the server.
 *
 * Used after navigator.credentials.create() for passkey registration.
 *
 * @param cred - PublicKeyCredential from the browser
 * @returns JSON-serializable object with Base64URL strings
 */
export function credentialCreationToJson(cred: PublicKeyCredential): {
  id: string;
  rawId: string;
  type: string;
  response: {
    clientDataJSON: string;
    attestationObject?: string;
  };
  clientExtensionResults: AuthenticationExtensionsClientOutputs;
} {
  const response = cred.response as AuthenticatorAttestationResponse;

  return {
    id: cred.id,
    rawId: arrayBufferToBase64url(cred.rawId),
    type: cred.type,
    response: {
      clientDataJSON: arrayBufferToBase64url(response.clientDataJSON),
      attestationObject: response.attestationObject
        ? arrayBufferToBase64url(response.attestationObject)
        : undefined,
    },
    clientExtensionResults: cred.getClientExtensionResults(),
  };
}
