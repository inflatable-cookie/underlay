import type { EditableListItem } from "@inflatable-cookie/poodle-svelte";
import { onDestroy, untrack } from "svelte";
import type { QueryParams } from "../client/query";
import { getAuthConfig } from "../patterns/auth";
import { runAuthenticatedFetch } from "../patterns/auth-fetch";
import {
	createReorderController,
	type ReorderController,
	type ReorderableItem,
} from "../patterns/reorder-controller.svelte";
import { useToasts } from "../runtime/feedback";
import type { EntityListSharedProps, LoadedReorderConfig, ReorderConfig } from "./template.types";

export function isListReorderAvailable<T>(
	config: ReorderConfig<T> | undefined,
	counts: { items: number; total: number; page: number; pageSize: number },
): boolean {
	if (!config?.enabled) return false;
	if (config.strategy === "custom") return counts.total > 0;
	if (config.strategy === "loaded") return counts.total > 1;
	return (
		counts.page === 1 && counts.items > 1 && counts.total > 0 && counts.total <= counts.pageSize
	);
}

async function loadReorderItems<T>(
	config: LoadedReorderConfig<T>,
	query: QueryParams,
): Promise<T[]> {
	const auth = getAuthConfig();
	if (!auth) throw new Error("Auth is not configured");

	let items: T[] = [];
	let failure: Error | undefined;
	const attempted = await runAuthenticatedFetch({
		getToken: auth.getToken,
		onRefresh: auth.onRefresh,
		fetcher: async (token) => {
			const result = await config.loadItems(fetch, token, query);
			if (result.error) throw new Error(result.error);
			return result.items;
		},
		onSuccess: (result) => (items = result),
		onError: (error) => (failure = error),
		fallbackErrorMessage: "Failed to load reorder items",
	});

	if (failure) throw failure;
	if (!attempted) throw new Error("Session expired");
	return items;
}

type ReorderSession<T> =
	| { strategy: "custom" }
	| {
			strategy: "inline" | "loaded";
			items: Array<T & ReorderableItem>;
			controller: ReorderController<T & ReorderableItem>;
	  };

export function useEntityListReorder<T extends object>(options: {
	config: () => ReorderConfig<T> | undefined;
	active: () => boolean;
	available: () => boolean;
	setActive: (active: boolean) => void;
	items: () => T[];
	getItemId: (item: T) => string;
	query: () => QueryParams;
	onEnter: () => void;
	onSaved: () => Promise<void>;
	onError: () => EntityListSharedProps<T>["onReorderError"];
}) {
	const toasts = useToasts();
	let session = $state.raw<ReorderSession<T> | null>(null);
	let request: { fromActiveMode: boolean } | null = null;
	let error = $state<string | null>(null);
	let highlightedIds = $state<string[]>([]);

	const controller = $derived(session && session.strategy !== "custom" ? session.controller : null);
	const editableItems = $derived(
		controller?.pending.map((item) => ({
			id: item.id,
			label: "label" in item && typeof item.label === "string" ? item.label : item.id,
		})) ?? [],
	);

	function reset() {
		request = null;
		controller?.reset();
		session = null;
		error = null;
		highlightedIds = [];
	}

	function exit() {
		reset();
		options.setActive(false);
	}

	async function prepare(fromActiveMode: boolean) {
		const config = options.config();
		if (!config?.enabled || request) return false;
		if (config.strategy === "custom") {
			session = { strategy: "custom" };
			return true;
		}

		const pending = { fromActiveMode };
		request = pending;
		const strategy = config.strategy ?? "inline";
		const withId = (item: T) => ({ ...item, id: options.getItemId(item) });

		try {
			const source =
				config.strategy === "loaded"
					? await loadReorderItems(config, options.query())
					: options.items();
			if (request !== pending) return false;
			if (!options.available() || (options.config()?.strategy ?? "inline") !== strategy)
				return false;

			const items =
				config.strategy === "loaded" && config.mapItems
					? config.mapItems(source)
					: source.map(withId);
			session = {
				strategy,
				items,
				controller: createReorderController(items, (orderedIds) => {
					const current = options.config();
					return current && "handler" in current ? current.handler(orderedIds) : Promise.resolve();
				}),
			};
			return true;
		} catch (cause) {
			if (request === pending) {
				toasts.push({
					message: cause instanceof Error ? cause.message : "Failed to load reorder items",
					variant: "error",
				});
			}
			return false;
		} finally {
			if (request === pending) request = null;
		}
	}

	async function enter() {
		if (!options.available() || request || session) return;
		options.onEnter();
		if (await prepare(false)) options.setActive(true);
	}

	$effect(() => {
		const active = options.active();
		const available = options.available();
		const strategy = options.config()?.strategy ?? "inline";
		untrack(() => {
			if (!active) {
				if (session || request?.fromActiveMode) reset();
			} else if (!available) {
				exit();
			} else {
				if (session && session.strategy !== strategy) reset();
				if (!session && !request) {
					void prepare(true).then((ready) => {
						if (!ready && options.active() && !session && !request) exit();
					});
				}
			}
		});
	});

	onDestroy(() => {
		request = null;
		session = null;
	});

	async function save() {
		const current = session;
		if (!current || current.strategy === "custom" || current.controller.isPending) return;
		error = null;
		highlightedIds = [];

		try {
			await current.controller.submit();
			if (session !== current) return;
			const config = options.config();
			if (config && "successMessage" in config && config.successMessage) {
				toasts.push({ message: config.successMessage, variant: "success" });
			}
		} catch (cause) {
			if (session !== current) return;
			try {
				const recover = options.onError();
				const result = await recover?.({
					error: cause,
					controller: current.controller,
					items: current.items,
				});
				if (session !== current) return;
				error =
					typeof result === "string"
						? result
						: (result?.message ?? (cause instanceof Error ? cause.message : String(cause)));
				highlightedIds = typeof result === "object" ? (result.highlightedIds ?? []) : [];
			} catch (recoveryError) {
				if (session === current) {
					error = recoveryError instanceof Error ? recoveryError.message : String(recoveryError);
				}
			}
			return;
		}

		exit();
		await options.onSaved();
	}

	return {
		get controller() {
			return controller;
		},
		get editableItems() {
			return editableItems;
		},
		get error() {
			return error;
		},
		enter,
		exit,
		save,
		reset,
		updateOrder(nextItems: EditableListItem[]) {
			if (!controller || nextItems.length !== controller.pending.length) return;
			const byId = new Map(controller.pending.map((item) => [item.id, item]));
			const next = nextItems.flatMap(({ id }) => {
				const item = byId.get(id);
				return item && byId.delete(id) ? [item] : [];
			});
			if (next.length === controller.pending.length) controller.updatePending(next);
		},
		originalItem(id: string): T | undefined {
			return session?.strategy === "loaded"
				? session.items.find((item) => item.id === id)
				: options.items().find((item) => options.getItemId(item) === id);
		},
		isHighlighted: (id: string) => highlightedIds.includes(id),
	};
}
