/**
 * Nightfire Strategies Management
 *
 * Provides lazy-loading and caching of Nightfire strategies.
 * Configure once at app level, then NightfireEditor components
 * automatically fetch and use strategies as needed.
 */

import { getContext, setContext } from "svelte";
import { writable, get, derived } from "svelte/store";

export interface NightfireStrategyCardinality {
  mode: "single" | "multi";
  minBlocks?: number;
  maxBlocks?: number | null;
}

export interface NightfireBlockOption {
  type: string;
  label: string;
  category?: string;
  subcategory?: string;
}

export interface NightfireStrategy {
  id: string;
  cardinality: NightfireStrategyCardinality;
  allowedTypes: string[];
  allowedCategories: string[];
  defaultType: string;
  blockOptions?: NightfireBlockOption[];
}

export interface NightfireStrategiesConfig {
  /**
   * Function to fetch strategies from the API.
   * Should return the list of strategies.
   */
  fetchStrategies: () => Promise<NightfireStrategy[]>;
  /**
   * Cache TTL in milliseconds. Defaults to 1 hour.
   */
  cacheTtl?: number;
}

export interface NightfireStrategiesStore {
  strategies: typeof strategiesStore;
  loading: typeof loadingStore;
  error: typeof errorStore;
  ensure: () => Promise<NightfireStrategy[]>;
  refresh: () => Promise<NightfireStrategy[]>;
  invalidate: () => void;
  findById: (id: string) => NightfireStrategy | null;
}

const CONTEXT_KEY = Symbol("underlay-nightfire-strategies");
const DEFAULT_CACHE_TTL = 60 * 60 * 1000; // 1 hour

// Module-level stores (singleton)
const strategiesStore = writable<NightfireStrategy[]>([]);
const loadingStore = writable(false);
const errorStore = writable<string | null>(null);
const byIdStore = derived(strategiesStore, ($strategies) => {
  const index = new Map<string, NightfireStrategy>();
  for (const strategy of $strategies) {
    index.set(strategy.id, strategy);
  }
  return index;
});

let fetchedAt: number | null = null;
let fetchPromise: Promise<NightfireStrategy[]> | null = null;
let config: NightfireStrategiesConfig | null = null;

/**
 * Configure Nightfire strategies loading.
 * Call this once at app startup (e.g., in root layout).
 *
 * @example
 * ```ts
 * configureNightfireStrategies({
 *   fetchStrategies: async () => {
 *     const token = auth.getToken();
 *     if (!token) return [];
 *     return nightfireCommands.listStrategies(fetch, token, { includeOptions: true });
 *   }
 * });
 * ```
 */
export function configureNightfireStrategies(cfg: NightfireStrategiesConfig): void {
  config = cfg;
}

function isStale(): boolean {
  if (fetchedAt === null) return true;
  const ttl = config?.cacheTtl ?? DEFAULT_CACHE_TTL;
  return Date.now() - fetchedAt > ttl;
}

/**
 * Ensure strategies are loaded. Returns cached data if available and fresh,
 * otherwise fetches from the API. Multiple concurrent calls share the same
 * fetch promise.
 */
async function ensure(): Promise<NightfireStrategy[]> {
  // Return cached data if fresh
  const current = get(strategiesStore);
  if (current.length > 0 && !isStale()) {
    return current;
  }

  // If already fetching, return the existing promise
  if (fetchPromise) {
    return fetchPromise;
  }

  // Check if configured
  if (!config) {
    errorStore.set("Nightfire strategies not configured. Call configureNightfireStrategies() first.");
    return [];
  }

  // Start a new fetch
  loadingStore.set(true);
  errorStore.set(null);

  fetchPromise = config
    .fetchStrategies()
    .then((result) => {
      strategiesStore.set(result);
      fetchedAt = Date.now();
      loadingStore.set(false);
      fetchPromise = null;
      return result;
    })
    .catch((e) => {
      errorStore.set(e instanceof Error ? e.message : "Failed to load strategies");
      loadingStore.set(false);
      fetchPromise = null;
      return [];
    });

  return fetchPromise;
}

/**
 * Force a refresh of strategies, ignoring the cache.
 */
async function refresh(): Promise<NightfireStrategy[]> {
  fetchedAt = null;
  fetchPromise = null;
  return ensure();
}

/**
 * Invalidate the cache without fetching. Next call to ensure() will fetch.
 */
function invalidate(): void {
  fetchedAt = null;
  fetchPromise = null;
}

/**
 * Find a strategy by ID. Returns null if not found or not loaded.
 */
function findById(id: string): NightfireStrategy | null {
  return get(byIdStore).get(id) ?? null;
}

/**
 * Create the Nightfire strategies store.
 * Used internally by createNightfireStrategiesContext.
 */
function createStore(): NightfireStrategiesStore {
  return {
    strategies: strategiesStore,
    loading: loadingStore,
    error: errorStore,
    ensure,
    refresh,
    invalidate,
    findById
  };
}

/**
 * Set up the Nightfire strategies context.
 * Call this in your root layout component.
 */
export function createNightfireStrategiesContext(): NightfireStrategiesStore {
  const store = createStore();
  setContext(CONTEXT_KEY, store);
  return store;
}

/**
 * Get the Nightfire strategies store from context.
 * Returns null if not in a component context or not configured.
 */
export function useNightfireStrategies(): NightfireStrategiesStore | null {
  try {
    return getContext<NightfireStrategiesStore>(CONTEXT_KEY) ?? null;
  } catch {
    // Not in component context
    return null;
  }
}

/**
 * Get a strategy by ID, ensuring strategies are loaded first.
 * Convenience function for use in components.
 */
export async function getStrategy(id: string): Promise<NightfireStrategy | null> {
  await ensure();
  return findById(id);
}
