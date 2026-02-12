/**
 * Context injection for the Nightfire media block editor.
 *
 * The media editor lives in Underlay (generic library) but needs to
 * open a MediaPicker that requires app-specific API callbacks.
 * Consuming apps (e.g. Dairy) configure a context provider in their
 * root layout, following the same pattern as NightfireStrategies.
 */

import { getContext, setContext } from "svelte";

/** Result returned by the media picker when the user selects an item. */
export interface NightfireMediaPickResult {
  id: string;
  thumbnailUrl: string | null;
  title: string | null;
  originalFilename?: string | null;
  kind: string;
}

/** Context contract for opening a media picker from block editors. */
export interface NightfireMediaContext {
  /**
   * Open the media picker dialog.
   * Resolves with the selected media item, or `null` if the user cancels.
   */
  pickMedia: (options?: {
    filterKind?: string;
  }) => Promise<NightfireMediaPickResult | null>;
}

const CONTEXT_KEY = Symbol("underlay-nightfire-media");

/**
 * Set up the Nightfire media context.
 * Call this in your root layout component alongside strategy context.
 */
export function createNightfireMediaContext(
  ctx: NightfireMediaContext
): void {
  setContext(CONTEXT_KEY, ctx);
}

/**
 * Retrieve the Nightfire media context.
 * Returns `null` if no context has been provided (graceful degradation).
 */
export function useNightfireMedia(): NightfireMediaContext | null {
  try {
    return getContext<NightfireMediaContext>(CONTEXT_KEY) ?? null;
  } catch {
    return null;
  }
}
