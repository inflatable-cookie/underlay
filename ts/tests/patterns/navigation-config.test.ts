import { describe, it, expect } from "vitest";
import {
	getNavigationContextConfig,
	setNavigationContextConfig
} from "../../src/patterns/navigation-config";

describe("navigation-config", () => {
	it("uses defaults initially", () => {
		setNavigationContextConfig({});
		expect(getNavigationContextConfig()).toEqual({
			storageKey: "underlay:nav-context",
			maxDepth: 3
		});
	});

	it("accepts custom config values", () => {
		setNavigationContextConfig({
			storageKey: "custom:key",
			maxDepth: 8
		});
		expect(getNavigationContextConfig()).toEqual({
			storageKey: "custom:key",
			maxDepth: 8
		});
	});

	it("merges with defaults when partial config is supplied", () => {
		setNavigationContextConfig({
			maxDepth: 5
		});
		expect(getNavigationContextConfig()).toEqual({
			storageKey: "underlay:nav-context",
			maxDepth: 5
		});
	});
});
