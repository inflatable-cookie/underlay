import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, beforeEach, describe, expect, it } from "vitest";

import { loadConfigStack } from "../../src/server/config-stack";

describe("loadConfigStack", () => {
	let configDir: string;

	beforeEach(() => {
		configDir = mkdtempSync(join(tmpdir(), "underlay-config-stack-"));
	});

	afterEach(() => {
		rmSync(configDir, { recursive: true, force: true });
	});

	it("merges default, environment, and local overlay files", () => {
		writeFileSync(join(configDir, "default.toml"), "[app]\nname = \"base\"\n");
		writeFileSync(join(configDir, "dev.toml"), "[app]\ndebug = true\n");
		writeFileSync(join(configDir, "local.toml"), "[app]\nname = \"local\"\n");

		const config = loadConfigStack({ configDir, environment: "dev" });
		expect(config.app).toEqual({ name: "local", debug: true });
	});

	it("does not pollute Object.prototype from __proto__ tables", () => {
		writeFileSync(join(configDir, "default.toml"), "[__proto__]\npolluted = true\n");

		loadConfigStack({ configDir, localOverlay: false });

		expect((Object.prototype as Record<string, unknown>).polluted).toBeUndefined();
		expect(({} as Record<string, unknown>).polluted).toBeUndefined();
	});

	it("rejects env overrides with forbidden key segments", () => {
		writeFileSync(join(configDir, "default.toml"), "[app]\nname = \"base\"\n");

		expect(() =>
			loadConfigStack({
				configDir,
				localOverlay: false,
				envOverrides: { "__proto__.polluted": true }
			})
		).toThrow(/forbidden segment/);
		expect(({} as Record<string, unknown>).polluted).toBeUndefined();
	});
});
