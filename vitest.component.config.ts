import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

export default defineConfig({
	plugins: [svelte()],
	test: {
		globals: true,
		environment: "jsdom",
		include: ["ts/tests/**/*.component.test.ts"],
		exclude: ["node_modules", "dist", "ts/tests/fixtures/**"],
		setupFiles: ["./ts/tests/setup/vitest-component.setup.ts"],
	},
	resolve: {
		conditions: ["browser", "module", "import", "default"],
		alias: {
			"$app/environment": resolve(
				import.meta.dirname,
				"./ts/tests/setup/sveltekit-environment.mock.ts",
			),
			"$app/navigation": resolve(
				import.meta.dirname,
				"./ts/tests/setup/sveltekit-navigation.mock.ts",
			),
			"$app/state": resolve(
				import.meta.dirname,
				"./ts/tests/setup/sveltekit-state.mock.ts",
			),
			"@inflatable-cookie/underlay": resolve(import.meta.dirname, "./ts/src"),
			"@inflatable-cookie/underlay/client": resolve(import.meta.dirname, "./ts/src/client"),
			"@inflatable-cookie/underlay/patterns": resolve(import.meta.dirname, "./ts/src/patterns"),
		},
	},
});
