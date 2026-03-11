/**
 * WebAuthn credential helpers for converting between API formats.
 *
 * The WebAuthn API uses ArrayBuffer for binary data, but JSON serialization
 * requires Base64URL strings. These helpers handle the conversion.
 */

import { base64urlToArrayBuffer, arrayBufferToBase64url } from "./base64url.js";

export type RegistrationCredentialJson = ReturnType<typeof credentialCreationToJson>;
export type AuthenticationCredentialJson = ReturnType<typeof assertionToJson>;

export type PasskeyErrorCode =
  | "not_supported"
  | "cancelled"
  | "timeout"
  | "invalid_state"
  | "security_error"
  | "network_error"
  | "unknown";

export interface PasskeyError {
  code: PasskeyErrorCode;
  message: string;
  cause?: unknown;
}

export function sanitizePasskeyErrorMessage(message: string): string {
  return message
    .replace(/https?:\/\/\S+/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

export function isPasskeySupported(): boolean {
  return typeof PublicKeyCredential !== "undefined" &&
    typeof navigator !== "undefined" &&
    !!navigator.credentials;
}

export async function supportsConditionalMediation(): Promise<boolean> {
  if (
    typeof PublicKeyCredential === "undefined" ||
    typeof PublicKeyCredential.isConditionalMediationAvailable !== "function"
  ) {
    return false;
  }

  try {
    return await PublicKeyCredential.isConditionalMediationAvailable();
  } catch {
    return false;
  }
}

export function mapWebAuthnError(cause: unknown): PasskeyError {
  if (!isPasskeySupported()) {
    return {
      code: "not_supported",
      message: "Passkeys are not supported in this browser.",
      cause,
    };
  }

  if (cause instanceof DOMException) {
    const message = sanitizePasskeyErrorMessage(cause.message || cause.name);
    switch (cause.name) {
      case "AbortError":
      case "NotAllowedError":
        return {
          code: "cancelled",
          message: message || "Passkey request was cancelled.",
          cause,
        };
      case "TimeoutError":
        return {
          code: "timeout",
          message: message || "Passkey request timed out.",
          cause,
        };
      case "InvalidStateError":
        return {
          code: "invalid_state",
          message: message || "This passkey request is not valid for the current device or account state.",
          cause,
        };
      case "SecurityError":
        return {
          code: "security_error",
          message: message || "The browser blocked this passkey request for security reasons.",
          cause,
        };
      case "NetworkError":
        return {
          code: "network_error",
          message: message || "A network-related passkey error occurred.",
          cause,
        };
      default:
        return {
          code: "unknown",
          message: message || "Passkey request failed.",
          cause,
        };
    }
  }

  if (cause instanceof Error) {
    return {
      code: "unknown",
      message: sanitizePasskeyErrorMessage(cause.message) || "Passkey request failed.",
      cause,
    };
  }

  return {
    code: "unknown",
    message: "Passkey request failed.",
    cause,
  };
}

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
