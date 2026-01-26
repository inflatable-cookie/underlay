/**
 * Types for SPA form handling.
 *
 * These types support client-side form submission without SvelteKit actions,
 * useful for SPAs that communicate directly with APIs.
 */

/**
 * Result from an SPA form submission handler.
 *
 * Return `success: true` for successful operations.
 * Return `success: false` with `error`/`fieldErrors` for failures.
 * Return `redirectTo` to navigate after success.
 *
 * @example
 * ```typescript
 * async function handleSubmit(formData: FormData): Promise<SpaFormResult> {
 *   try {
 *     await api.createItem(formData);
 *     return { success: true, redirectTo: '/items' };
 *   } catch (e) {
 *     return {
 *       success: false,
 *       error: e.message,
 *       fieldErrors: { name: 'Name already exists' }
 *     };
 *   }
 * }
 * ```
 */
export interface SpaFormResult {
  /** Whether the submission was successful */
  success: boolean;
  /** Global error message (not field-specific) */
  error?: string | null;
  /** Field-level error messages keyed by field name */
  fieldErrors?: Record<string, string> | null;
  /** URL to navigate to after successful submission */
  redirectTo?: string;
  /** Updated form values to preserve after submission */
  values?: Record<string, unknown>;
}

/**
 * Handler function for SPA form submissions.
 *
 * Called with the FormData when the form is submitted.
 * Should return a SpaFormResult indicating success/failure.
 */
export type SpaSubmitHandler = (formData: FormData) => Promise<SpaFormResult>;

/**
 * Navigation function type for redirects after form submission.
 *
 * Compatible with SvelteKit's `goto` function signature.
 */
export type SpaNavigateFn = (url: string) => Promise<void> | void;
