<script lang="ts">
	import {
		createAiRoutingOpsController,
		type AiRoutingOpsController,
		type AiRoutingOpsSource,
	} from "../../src/patterns/ai-routing-ops.svelte";
	import {
		createPaginationController,
		type ServerPaginationResult,
	} from "../../src/patterns/pagination-server.svelte";
	import type {
		CursorPaginatedResponse,
		CursorPaginationParams,
	} from "../../src/patterns/pagination-types";

	interface Props {
		paginationFetcher?: (
			fetchFn: typeof fetch,
			token: string,
			params: CursorPaginationParams,
		) => Promise<CursorPaginatedResponse<{ id: string }>>;
		onPaginationReady?: (controller: ServerPaginationResult<{ id: string }>) => void;
		aiSource?: AiRoutingOpsSource;
		onAiReady?: (controller: AiRoutingOpsController) => void;
	}

	let { paginationFetcher, onPaginationReady, aiSource, onAiReady }: Props = $props();

	if (paginationFetcher) {
		const pagination = createPaginationController<{ id: string }>(paginationFetcher, {
			getToken: () => "tok-1",
		});
		onPaginationReady?.(pagination);
	}

	if (aiSource) {
		const ai = createAiRoutingOpsController(aiSource);
		onAiReady?.(ai);
	}
</script>
