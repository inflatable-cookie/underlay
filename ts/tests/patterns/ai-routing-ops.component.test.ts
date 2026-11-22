// @vitest-environment jsdom
import { flushSync } from "svelte";
import { describe, expect, it, vi } from "vitest";
import {
	createAiRoutingOpsController,
	type AiRoutingDiagnostics,
	type AiRoutingOpsSource,
} from "../../src/patterns/ai-routing-ops.svelte";
import { createEffectRoot } from "./effect-root.svelte";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

function makeDiagnostics(configVersion: number): AiRoutingDiagnostics {
	return {
		configVersion,
		providerCount: 1,
		enabledProviderCount: 1,
		aliasCount: 1,
		enabledAliasCount: 1,
		bindingCount: 1,
		enabledBindingCount: 1,
		latestUpdatedAt: null,
	};
}

describe("patterns/ai-routing-ops.svelte.ts (compiled)", () => {
	it("ignores NaN and Infinity inputs in range setters", () => {
		const source: AiRoutingOpsSource = {
			fetchDiagnostics: vi.fn(async () => makeDiagnostics(1)),
			fetchMetrics: vi.fn(async () => []),
			fetchCost: vi.fn(async () => []),
			fetchCostAnomalies: vi.fn(async () => []),
			fetchAlerts: vi.fn(async () => ({
				deadLetterCount24h: 0,
				runtimeFailureCount24h: 0,
				circuitOpenCount24h: 0,
				fallbackRunCount24h: 0,
				exhaustedChainRunCount24h: 0,
			})),
			fetchParity: vi.fn(async () => []),
		};

		let controller!: ReturnType<typeof createAiRoutingOpsController>;
		const dispose = createEffectRoot(() => {
			controller = createAiRoutingOpsController(source, {
				defaultMetricHours: 12,
				defaultCostDays: 14,
				defaultParityHours: 6,
				defaultAnomalyDays: 10,
			});
		});

		try {
			for (const invalid of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
				controller.setMetricHours(invalid);
				controller.setCostDays(invalid);
				controller.setParityHours(invalid);
				controller.setAnomalyDays(invalid);
			}

			expect(controller.metricHours).toBe(12);
			expect(controller.costDays).toBe(14);
			expect(controller.parityHours).toBe(6);
			expect(controller.anomalyDays).toBe(10);
		} finally {
			dispose();
		}
	});

	it("prevents overlapping refreshAll() calls from overwriting newer data or clearing loading early", async () => {
		const first = deferred<AiRoutingDiagnostics>();
		const second = deferred<AiRoutingDiagnostics>();
		let callCount = 0;

		const source: AiRoutingOpsSource = {
			fetchDiagnostics: vi.fn(() => {
				callCount += 1;
				return callCount === 1 ? first.promise : second.promise;
			}),
			fetchMetrics: vi.fn(async () => []),
			fetchCost: vi.fn(async () => []),
			fetchCostAnomalies: vi.fn(async () => []),
			fetchAlerts: vi.fn(async () => ({
				deadLetterCount24h: 0,
				runtimeFailureCount24h: 0,
				circuitOpenCount24h: 0,
				fallbackRunCount24h: 0,
				exhaustedChainRunCount24h: 0,
			})),
			fetchParity: vi.fn(async () => []),
		};

		let controller!: ReturnType<typeof createAiRoutingOpsController>;
		const dispose = createEffectRoot(() => {
			controller = createAiRoutingOpsController(source);
		});
		flushSync();

		try {
			const req1 = controller.refreshAll();
			const req2 = controller.refreshAll();
			expect(controller.loading).toBe(true);

			// Older request resolves first: must NOT overwrite state or clear loading
			first.resolve(makeDiagnostics(1));
			await req1;
			expect(controller.loading).toBe(true);
			expect(controller.diagnostics).toBeNull();

			// Newer request resolves: updates state and clears loading
			second.resolve(makeDiagnostics(2));
			await req2;
			expect(controller.loading).toBe(false);
			expect(controller.diagnostics?.configVersion).toBe(2);
		} finally {
			dispose();
		}
	});

	it("does not mutate state when unmounted while refreshAll() is in flight", async () => {
		const gate = deferred<AiRoutingDiagnostics>();
		const source: AiRoutingOpsSource = {
			fetchDiagnostics: vi.fn(() => gate.promise),
			fetchMetrics: vi.fn(async () => []),
			fetchCost: vi.fn(async () => []),
			fetchCostAnomalies: vi.fn(async () => []),
			fetchAlerts: vi.fn(async () => ({
				deadLetterCount24h: 0,
				runtimeFailureCount24h: 0,
				circuitOpenCount24h: 0,
				fallbackRunCount24h: 0,
				exhaustedChainRunCount24h: 0,
			})),
			fetchParity: vi.fn(async () => []),
		};

		let controller!: ReturnType<typeof createAiRoutingOpsController>;
		const dispose = createEffectRoot(() => {
			controller = createAiRoutingOpsController(source);
		});
		flushSync();

		const inFlight = controller.refreshAll();
		dispose();
		flushSync();

		gate.resolve(makeDiagnostics(42));
		await inFlight;

		expect(controller.diagnostics).toBeNull();
	});
});
