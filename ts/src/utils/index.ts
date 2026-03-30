/**
 * Utility functions for Underlay.
 *
 * Keep this root barrel stable as a convenience surface, but prefer focused
 * `@decodelabs/underlay/utils/*` subpaths for new code and docs.
 */

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

// Pure formatting helpers
export {
  configureFormat,
  format,
  formatDate,
  formatTime,
  formatDateTime,
  formatRelative,
  formatAdaptiveDateRange,
  formatDateWithOrdinal,
  formatNumber,
  formatPercent,
  formatFileSize,
  formatCurrency,
  plural,
  pluralCount,
  type FormatConfig,
  type DateStyle,
  type TimeStyle,
  type DateRangeInput,
  type DateRangeStyle,
  type DateRangeFormatOptions,
  type PluralForms,
} from "./i18n.js";

// Slug helpers
export {
  RESERVED_SLUGS,
  slugify,
  isValidSlugFormat,
  isReservedSlug,
  validateSlug,
  type SlugValidationError,
  type SlugValidationResult,
} from "./slug.js";

// HTML sanitization helpers
export {
  sanitizeHtml,
  sanitizeEmbedHtml,
  sanitizeSvgHtml,
} from "./html.js";
