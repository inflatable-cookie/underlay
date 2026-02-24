import { describe, expect, it, vi } from "vitest";

async function loadAiRoutingModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	return await import("../../src/patterns/ai-routing-ops.svelte");
}

describe("patterns/ai-routing-ops.svelte.ts", () => {
	it("refreshes all datasets and applies defaults", async () => {
		const {
			createAiRoutingOpsController,
			toPercent,
			estimateSuccessRate,
		} = await loadAiRoutingModule();

		const source = {
			fetchDiagnostics: vi.fn(async () => ({
				configVersion: 1,
				providerCount: 2,
				enabledProviderCount: 1,
				aliasCount: 3,
				enabledAliasCount: 2,
				bindingCount: 4,
				enabledBindingCount: 3,
				latestUpdatedAt: "2026-02-24T00:00:00Z",
			})),
			fetchMetrics: vi.fn(async () => [
				{
					actionKey: "summarize",
					providerName: "openai",
					modelName: "gpt",
					runCount: 10,
					successCount: 9,
					failedCount: 1,
					fallbackRunCount: 1,
					avgLatencyMs: 120,
					p95LatencyMs: 180,
					inputTokensSum: 500,
					outputTokensSum: 700,
					terminalRuntimeFailures: 0,
				},
			]),
			fetchCost: vi.fn(async () => [
				{
					day: "2026-02-23",
					actionKey: "summarize",
					providerName: "openai",
					modelName: "gpt",
					runCount: 3,
					inputTokensSum: 100,
					outputTokensSum: 120,
					estimatedCostUsd: 1.25,
					overBudget: false,
				},
			]),
			fetchCostAnomalies: vi.fn(async () => []),
			fetchAlerts: vi.fn(async () => ({
				deadLetterCount24h: 1,
				runtimeFailureCount24h: 2,
				circuitOpenCount24h: 0,
				fallbackRunCount24h: 3,
			})),
			fetchParity: vi.fn(async () => [
				{
					actionKey: "summarize",
					modelAlias: "primary",
					runCount: 5,
					successCount: 4,
					failedCount: 1,
					successRate: 0.8,
				},
			]),
		};

		const controller = createAiRoutingOpsController(source);
		expect(controller.metricHours).toBe(24);
		expect(controller.costDays).toBe(7);
		expect(controller.parityHours).toBe(24);
		expect(controller.anomalyDays).toBe(7);

		await controller.refreshAll();
		expect(controller.loading).toBe(false);
		expect(controller.error).toBeNull();
		expect(controller.diagnostics?.configVersion).toBe(1);
		expect(controller.metrics).toHaveLength(1);
		expect(controller.cost).toHaveLength(1);
		expect(controller.anomalies).toEqual([]);
		expect(controller.alerts?.deadLetterCount24h).toBe(1);
		expect(controller.parity).toHaveLength(1);

		expect(toPercent(0.125)).toBe("12.5%");
		expect(estimateSuccessRate(0, 10)).toBe(0);
		expect(estimateSuccessRate(10, 8)).toBe(0.8);
	});

	it("clamps range setters and captures first rejection while preserving fulfilled updates", async () => {
		const { createAiRoutingOpsController } = await loadAiRoutingModule();

		const source = {
			fetchDiagnostics: vi.fn(async () => ({
				configVersion: 9,
				providerCount: 1,
				enabledProviderCount: 1,
				aliasCount: 1,
				enabledAliasCount: 1,
				bindingCount: 1,
				enabledBindingCount: 1,
				latestUpdatedAt: null,
			})),
			fetchMetrics: vi.fn(async () => {
				throw new Error("metrics failed");
			}),
			fetchCost: vi.fn(async () => [
				{
					day: "2026-02-24",
					actionKey: "route",
					providerName: "provider",
					modelName: "model",
					runCount: 1,
					inputTokensSum: 1,
					outputTokensSum: 1,
					estimatedCostUsd: 0.01,
					overBudget: false,
				},
			]),
			fetchCostAnomalies: vi.fn(async () => {
				throw "bad anomalies";
			}),
			fetchAlerts: vi.fn(async () => {
				throw { reason: "bad alerts" };
			}),
			fetchParity: vi.fn(async () => []),
		};

		const controller = createAiRoutingOpsController(source, {
			defaultMetricHours: 12,
			defaultCostDays: 14,
			defaultParityHours: 10,
			defaultAnomalyDays: 30,
		});

		controller.setMetricHours(0);
		controller.setMetricHours(24 * 30 + 100);
		expect(controller.metricHours).toBe(24 * 30);
		controller.setMetricHours(7.9);
		expect(controller.metricHours).toBe(7);

		controller.setCostDays(0);
		controller.setCostDays(999);
		expect(controller.costDays).toBe(365);
		controller.setCostDays(8.7);
		expect(controller.costDays).toBe(8);

		controller.setParityHours(0);
		controller.setParityHours(9999);
		expect(controller.parityHours).toBe(24 * 30);
		controller.setParityHours(5.4);
		expect(controller.parityHours).toBe(5);

		controller.setAnomalyDays(1);
		expect(controller.anomalyDays).toBe(2);
		controller.setAnomalyDays(999);
		expect(controller.anomalyDays).toBe(90);
		controller.setAnomalyDays(11.2);
		expect(controller.anomalyDays).toBe(11);

		await controller.refreshAll();
		expect(controller.error).toBe("metrics failed");
		expect(controller.diagnostics?.configVersion).toBe(9);
		expect(controller.cost).toHaveLength(1);
		expect(controller.parity).toEqual([]);
		expect(controller.loading).toBe(false);
	});
});
