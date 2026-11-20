import { untrack } from "svelte";
import type { QueryParams } from "../client/query";
import { listQueryKey, normalizeListQuery } from "./entity-list-query";

export function createEntityListQuery(options: {
	external: () => QueryParams | undefined;
	onChange: () => ((query: QueryParams) => void) | undefined;
}) {
	let internal = $state<QueryParams>({});
	const external = $derived(options.external());
	const source = $derived(normalizeListQuery(external ?? internal));
	const sourceKey = $derived(listQueryKey(source));

	// Keep control changes responsive while URL state catches up. A changed
	// parent query supersedes the draft, including when navigating back.
	let draft = $state<{ sourceKey: string; query: QueryParams } | null>(null);
	const current = $derived(draft?.sourceKey === sourceKey ? draft.query : source);
	$effect(() => {
		const key = sourceKey;
		untrack(() => {
			if (draft && draft.sourceKey !== key) draft = null;
		});
	});

	return {
		get current() {
			return current;
		},
		set(nextQuery: QueryParams) {
			const query = normalizeListQuery(nextQuery);
			const notify = options.onChange();
			if (notify || external !== undefined) {
				draft = { sourceKey, query };
				notify?.(query);
			} else {
				internal = query;
			}
		},
	};
}
