/**
 * Utility functions for Underlay.
 */

// Base64URL encoding/decoding
export {
  base64urlToArrayBuffer,
  arrayBufferToBase64url,
} from "./base64url.js";

// WebAuthn credential helpers
export {
  toPublicKeyRequestOptions,
  toPublicKeyCreationOptions,
  assertionToJson,
  credentialCreationToJson,
  sanitizePasskeyErrorMessage,
  isPasskeySupported,
  supportsConditionalMediation,
  mapWebAuthnError,
  type PasskeyError,
  type PasskeyErrorCode,
  type RegistrationCredentialJson,
  type AuthenticationCredentialJson,
} from "./webauthn.js";

// Sequence utilities
export {
  getNextLetter,
  getNextNumber,
  type GetNextLetterOptions,
} from "./sequence.js";

// HTML sanitization helpers
export {
  sanitizeHtml,
  sanitizeEmbedHtml,
  sanitizeSvgHtml,
} from "./html.js";
