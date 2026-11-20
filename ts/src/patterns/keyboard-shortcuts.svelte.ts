/**
 * Keyboard Shortcut Manager
 *
 * Centralized keyboard shortcut registration with priority, conditional
 * activation, and cross-platform `mod+` prefix (Cmd on Mac, Ctrl elsewhere).
 *
 * @example
 * ```svelte
 * <script>
 *   import { createKeyboardShortcuts } from '@inflatable-cookie/underlay/runtime/browser';
 *
 *   const shortcuts = createKeyboardShortcuts();
 *   shortcuts.register("Escape", () => closePanel());
 *   shortcuts.register("mod+k", () => openSearch(), { description: "Open search" });
 * </script>
 *
 * <svelte:window onkeydown={shortcuts.handleKeydown} />
 * ```
 */

export interface ShortcutOptions {
  /** Description of the shortcut (for help display) */
  description?: string;
  /** Conditional activation — shortcut only fires when this returns true */
  when?: () => boolean;
  /** Priority — higher wins on conflict (default: 0) */
  priority?: number;
  /**
   * Fire even when focus is in a text-editing element (input, textarea,
   * select, or contenteditable). Off by default so typing never triggers
   * global shortcuts.
   */
  allowInInputs?: boolean;
}

export interface RegisteredShortcut {
  /** Key pattern (e.g., "mod+k", "Escape") */
  key: string;
  /** Human-readable description */
  description: string;
  /** Priority level */
  priority: number;
}

export interface KeyboardShortcutManager {
  /** Register a keyboard shortcut. Returns an unregister function. */
  register: (key: string, handler: () => void, options?: ShortcutOptions) => () => void;
  /** Unregister a shortcut by key pattern */
  unregister: (key: string) => void;
  /** All registered shortcuts (for display in a shortcut palette) */
  readonly shortcuts: RegisteredShortcut[];
  /** Keydown handler — bind to svelte:window */
  handleKeydown: (event: KeyboardEvent) => void;
}

interface InternalShortcut {
  key: string;
  normalizedKey: string;
  handler: () => void;
  description: string;
  when?: () => boolean;
  priority: number;
  allowInInputs: boolean;
}

const isMac = typeof navigator !== "undefined" && /Mac|iPhone|iPad/.test(navigator.userAgent);

/**
 * Split a key pattern on `+`. The `+` key is special: a bare `+` or a
 * trailing `++` (e.g. `mod++`) names the plus key rather than an empty part.
 */
function splitKeyPattern(key: string): string[] {
  const lower = key.trim().toLowerCase();
  if (lower === "+") return ["+"];
  if (lower.endsWith("++")) {
    const modifiers = lower.slice(0, -2).split("+").map((p) => p.trim()).filter((p) => p.length > 0);
    return [...modifiers, "+"];
  }
  return lower.split("+").map((p) => p.trim()).filter((p) => p.length > 0);
}

/**
 * Normalize a key pattern: resolve `mod+` to platform key,
 * lowercase, and sort modifiers for consistent matching.
 */
function normalizeKey(key: string): string {
  const parts = splitKeyPattern(key);

  // Resolve mod+ to platform modifier
  const modIndex = parts.indexOf("mod");
  if (modIndex !== -1) {
    parts[modIndex] = isMac ? "meta" : "ctrl";
  }

  // Sort modifiers before the main key for consistent comparison
  const modifiers = parts.filter((p) => ["ctrl", "alt", "shift", "meta"].includes(p));
  const mainKey = parts.filter((p) => !["ctrl", "alt", "shift", "meta"].includes(p));
  modifiers.sort();

  return [...modifiers, ...mainKey].join("+");
}

/**
 * Build a normalized key string from a KeyboardEvent.
 */
function eventToKey(event: KeyboardEvent): string {
  const parts: string[] = [];
  if (event.ctrlKey) parts.push("ctrl");
  if (event.altKey) parts.push("alt");
  if (event.shiftKey) parts.push("shift");
  if (event.metaKey) parts.push("meta");
  parts.sort();

  const key = event.key.toLowerCase();
  // Avoid duplicating modifier keys
  if (!["control", "alt", "shift", "meta"].includes(key)) {
    parts.push(key);
  }

  return parts.join("+");
}

/**
 * Whether a keyboard event originated from an element that accepts text input.
 * Duck-typed so it stays safe during SSR and in non-DOM test environments.
 */
function isEditableTarget(target: EventTarget | null): boolean {
  if (!target || typeof target !== "object") return false;
  const el = target as {
    tagName?: unknown;
    isContentEditable?: unknown;
    getAttribute?: (name: string) => string | null;
  };
  if (el.isContentEditable === true) return true;
  const editableAttr = typeof el.getAttribute === "function" ? el.getAttribute("contenteditable") : null;
  if (editableAttr !== null && editableAttr.toLowerCase() !== "false") return true;
  return typeof el.tagName === "string" && ["INPUT", "TEXTAREA", "SELECT"].includes(el.tagName.toUpperCase());
}

/**
 * Create a keyboard shortcut manager.
 */
export function createKeyboardShortcuts(): KeyboardShortcutManager {
  let registrations = $state<InternalShortcut[]>([]);

  function register(
    key: string,
    handler: () => void,
    options: ShortcutOptions = {}
  ): () => void {
    const entry: InternalShortcut = {
      key,
      normalizedKey: normalizeKey(key),
      handler,
      description: options.description ?? "",
      when: options.when,
      priority: options.priority ?? 0,
      allowInInputs: options.allowInInputs ?? false
    };

    registrations = [...registrations, entry];

    // Return unregister function
    return () => {
      registrations = registrations.filter((r) => r !== entry);
    };
  }

  function unregister(key: string): void {
    const normalized = normalizeKey(key);
    registrations = registrations.filter((r) => r.normalizedKey !== normalized);
  }

  function handleKeydown(event: KeyboardEvent): void {
    const eventKey = eventToKey(event);
    const inTextField = isEditableTarget(event.target);

    // Find all matching shortcuts
    const matches = registrations.filter((r) => {
      if (r.normalizedKey !== eventKey) return false;
      if (inTextField && !r.allowInInputs) return false;
      if (r.when && !r.when()) return false;
      return true;
    });

    if (matches.length === 0) return;

    // Pick highest priority
    matches.sort((a, b) => b.priority - a.priority);
    const winner = matches[0];

    event.preventDefault();
    winner.handler();
  }

  const shortcutsList = $derived(
    registrations.map((r) => ({
      key: r.key,
      description: r.description,
      priority: r.priority
    }))
  );

  return {
    register,
    unregister,
    get shortcuts() { return shortcutsList; },
    handleKeydown
  };
}
