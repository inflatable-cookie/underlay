<script lang="ts">
	import { onMount } from "svelte";
	import { healthCommands } from "@api-client";

	let healthStatus = $state<string | null>(null);
	let errorMessage = $state<string | null>(null);

	onMount(async () => {
		try {
			const res = await healthCommands.health(fetch);
			healthStatus = res.status;
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : "Failed to fetch health";
		}
	});
</script>

<h1>Acme</h1>

{#if errorMessage}
	<p><strong>API health:</strong> {errorMessage}</p>
{:else if healthStatus}
	<p>
		API health: <strong>{healthStatus}</strong>
	</p>
{:else}
	<p>API health: checking...</p>
{/if}
<p>
	Edit <code>front/src/routes/+page.svelte</code> to start building.
</p>

<style>
	h1 {
		font-size: 2rem;
		letter-spacing: -0.02em;
		margin: 0 0 0.5rem;
	}

	code {
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono",
			"Courier New", monospace;
		font-size: 0.95em;
	}
</style>
