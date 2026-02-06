/**
 * Pattern types - shared type definitions for patterns module
 *
 * These are extracted from Svelte components to allow TypeScript-only
 * consumers (like cattle-grid) to import them.
 */

export type PageHeaderLevel = 1 | 2 | 3 | 4;

export interface BreadcrumbItem {
  label: string;
  href?: string;
}
