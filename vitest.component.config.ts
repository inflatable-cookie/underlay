import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

export default defineConfig({
	plugins: [svelte()],
	test: {
		globals: true,
		environment: "jsdom",
		include: ["ts/tests/**/*.component.test.ts"],
		exclude: ["node_modules", "dist"],
		setupFiles: ["./ts/tests/setup/vitest-component.setup.ts"],
	},
	resolve: {
		conditions: ["browser", "module", "import", "default"],
		alias: {
			"$app/environment": resolve(
				__dirname,
				"./ts/tests/setup/sveltekit-environment.mock.ts",
			),
			"$app/navigation": resolve(
				__dirname,
				"./ts/tests/setup/sveltekit-navigation.mock.ts",
			),
			"$app/state": resolve(
				__dirname,
				"./ts/tests/setup/sveltekit-state.mock.ts",
			),
			"@decodelabs/underlay": resolve(__dirname, "./ts/src"),
			"@decodelabs/underlay/client": resolve(__dirname, "./ts/src/client"),
			"@decodelabs/underlay/patterns": resolve(__dirname, "./ts/src/patterns"),
		},
	},
});
