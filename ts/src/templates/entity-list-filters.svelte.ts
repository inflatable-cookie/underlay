import { onDestroy, untrack } from "svelte";
import { SvelteMap } from "svelte/reactivity";
import type { FilterConfig, TemplateFilterOption } from "./template.types";

export function useEntityListFilterOptions(getFilters: () => FilterConfig[]) {
	const loaded = new SvelteMap<string, TemplateFilterOption[]>();
	const requests = new Map<string, { key: string | undefined }>();

	$effect(() => {
		const filters = getFilters().flatMap(({ id, loadKey, loadOptions, searchable }) =>
			loadOptions && !searchable ? [{ id, loadKey, loadOptions }] : [],
		);

		untrack(() => {
			const activeIds = new Set(filters.map(({ id }) => id));
			for (const id of requests.keys()) {
				if (!activeIds.has(id)) {
					requests.delete(id);
					loaded.delete(id);
				}
			}

			for (const { id, loadKey, loadOptions } of filters) {
				if (requests.has(id) && requests.get(id)?.key === loadKey) continue;

				const request = { key: loadKey };
				requests.set(id, request);
				loaded.delete(id);
				void load();

				async function load() {
					try {
						const options = await loadOptions();
						if (requests.get(id) === request) loaded.set(id, options);
					} catch {
						// Static options remain available if the asynchronous load fails.
					}
				}
			}
		});
	});

	onDestroy(() => requests.clear());

	return (filter: FilterConfig) => loaded.get(filter.id) ?? filter.options ?? [];
}

export function useEntityListSearch(getFilters: () => FilterConfig[]) {
	const drafts = new SvelteMap<string, string>();
	const timers = new Map<string, ReturnType<typeof setTimeout>>();

	function cancel(id: string) {
		clearTimeout(timers.get(id));
		timers.delete(id);
		drafts.delete(id);
	}

	function clear() {
		for (const id of timers.keys()) cancel(id);
	}

	$effect(() => {
		const searchIds = new Set(
			getFilters()
				.filter(({ type }) => type === "search")
				.map(({ id }) => id),
		);
		untrack(() => {
			for (const id of timers.keys()) {
				if (!searchIds.has(id)) cancel(id);
			}
		});
	});

	onDestroy(clear);

	return {
		value: (filter: FilterConfig, appliedValue: string) => drafts.get(filter.id) ?? appliedValue,
		change(filter: FilterConfig, value: string, delay: number, apply: () => void) {
			cancel(filter.id);
			if (filter.type !== "search" || delay <= 0) {
				apply();
				return;
			}

			drafts.set(filter.id, value);
			timers.set(
				filter.id,
				setTimeout(() => {
					cancel(filter.id);
					apply();
				}, delay),
			);
		},
		clear,
	};
}
