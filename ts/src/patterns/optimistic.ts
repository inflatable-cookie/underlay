/**
 * Optimistic update utilities for Svelte applications.
 *
 * Public entrypoint that re-exports focused optimistic store modules.
 */

export {
	createOptimisticList,
	type OptimisticOperation,
	type OptimisticAddOperation,
	type OptimisticUpdateOperation,
	type OptimisticListOptions,
	type OptimisticList
} from "./optimistic/list";

export { createOptimisticCounter, type OptimisticCounter } from "./optimistic/counter";
export { createOptimisticToggle, type OptimisticToggle } from "./optimistic/toggle";
export {
	createOptimisticValue,
	type OptimisticValue,
	type OptimisticValueOptions
} from "./optimistic/value";
