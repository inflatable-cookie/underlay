import {
	createReorderController,
	type ReorderController,
	type ReorderableItem,
} from "./reorder-controller.svelte";
import { applyReorderConflict, extractReorderConflict } from "./reorder-conflict";

interface CreateLocalReorderSessionParams<TItem extends ReorderableItem> {
	getController: () => ReorderController<TItem>;
	getLatestItems: () => TItem[];
	entityLabel: string;
	pushInfo: (message: string) => void;
	pushSuccess: (message: string) => void;
	onRefresh: () => Promise<void>;
	onDataChange?: () => void;
	clearSelection?: () => void;
	clearSelectionMode?: () => void;
}

export function createLocalReorderSession<TItem extends ReorderableItem>(
	params: CreateLocalReorderSessionParams<TItem>,
) {
	let reorderMode = $state(false);
	let highlightedIds = $state<string[]>([]);

	function enter() {
		params.clearSelectionMode?.();
		params.clearSelection?.();
		highlightedIds = [];
		reorderMode = true;
	}

	function exit() {
		highlightedIds = [];
		reorderMode = false;
	}

	async function handleSuccess(message: string) {
		exit();
		await params.onRefresh();
		params.onDataChange?.();
		params.pushSuccess(message);
	}

	async function handleError(error: unknown): Promise<void | string> {
		const conflict = extractReorderConflict(error);
		if (!conflict) return;

		const recovery = applyReorderConflict(
			params.getController(),
			conflict,
			params.getLatestItems(),
		);

		highlightedIds = recovery.addedIds;
		params.pushInfo(conflict.message);
		return conflict.message;
	}

	return {
		get reorderMode() {
			return reorderMode;
		},
		get highlightedIds() {
			return highlightedIds;
		},
		enter,
		exit,
		handleSuccess,
		handleError,
	};
}

interface CreateLoadedReorderSessionParams<TLoadedItem, TReorderItem extends ReorderableItem> {
	loadItems: () => Promise<{ items: TLoadedItem[]; error?: string }>;
	mapItems: (items: TLoadedItem[]) => TReorderItem[];
	submitReorder: (orderedIds: string[]) => Promise<void>;
	entityLabel: string;
	pushInfo: (message: string) => void;
	pushSuccess?: (message: string) => void;
	pushError: (message: string) => void;
	onRefresh: () => Promise<void>;
	onDataChange?: () => void;
	clearSelection?: () => void;
	clearSelectionMode?: () => void;
}

export function createLoadedReorderSession<TLoadedItem, TReorderItem extends ReorderableItem>(
	params: CreateLoadedReorderSessionParams<TLoadedItem, TReorderItem>,
) {
	const createController = (items: TReorderItem[]) =>
		createReorderController(items, async (orderedIds) => {
			await params.submitReorder(orderedIds);
		});

	let reorderMode = $state(false);
	let highlightedIds = $state<string[]>([]);
	let loadedItems = $state<TLoadedItem[]>([]);
	const reorderItems = $derived(params.mapItems(loadedItems));
	let controller = $state.raw<ReorderController<TReorderItem>>(createController([]));
	let enterGeneration = 0;

	async function enter() {
		const generation = ++enterGeneration;
		params.clearSelectionMode?.();
		params.clearSelection?.();

		const { items, error } = await params.loadItems();
		if (generation !== enterGeneration) {
			return;
		}
		if (error) {
			params.pushError(error);
			return;
		}

		loadedItems = items;
		controller = createController(params.mapItems(items));
		highlightedIds = [];
		reorderMode = true;
	}

	function exit() {
		enterGeneration++;
		reorderMode = false;
		highlightedIds = [];
		loadedItems = [];
		controller = createController([]);
	}

	async function handleSuccess(message?: string) {
		exit();
		await params.onRefresh();
		params.onDataChange?.();
		if (message && params.pushSuccess) {
			params.pushSuccess(message);
		}
	}

	async function handleError(error: unknown): Promise<void | string> {
		const conflict = extractReorderConflict(error);
		if (!conflict) return;

		const recovery = applyReorderConflict(controller, conflict, reorderItems);
		highlightedIds = recovery.addedIds;
		params.pushInfo(conflict.message);
		return conflict.message;
	}

	return {
		get reorderMode() {
			return reorderMode;
		},
		get highlightedIds() {
			return highlightedIds;
		},
		get reorderItems() {
			return reorderItems;
		},
		get controller() {
			return controller;
		},
		enter,
		exit,
		handleSuccess,
		handleError,
	};
}
