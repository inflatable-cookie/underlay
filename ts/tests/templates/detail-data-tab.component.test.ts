import { fireEvent, render, screen } from "@testing-library/svelte";
import { createRawSnippet } from "svelte";
import { expect, it, vi } from "vitest";
import DetailDataTab from "../../src/templates/DetailDataTab.svelte";

const renderResult = createRawSnippet<[string | null, string | null]>((data) => ({
	render: () => "<span></span>",
	setup: (element) => {
		element.textContent = data();
	},
}));

it("ignores a new inline loader until the item or reload key changes", async () => {
	const load = vi.fn(async (item: string | null) => `Details for ${item}`);
	const { rerender } = render(DetailDataTab<string, string>, {
		item: "Mars",
		loader: (item) => load(item),
		render: renderResult,
	});

	expect(await screen.findByText("Details for Mars")).toBeTruthy();
	await rerender({ loader: (item) => load(item) });
	expect(load).toHaveBeenCalledTimes(1);

	await rerender({ item: "Venus" });
	expect(await screen.findByText("Details for Venus")).toBeTruthy();
	expect(load).toHaveBeenCalledTimes(2);

	await rerender({ reloadKey: 1 });
	expect(await screen.findByText("Details for Venus")).toBeTruthy();
	expect(load).toHaveBeenCalledTimes(3);
});

it("shows the loader error and retries with the latest loader", async () => {
	const firstLoader = vi.fn(async () => {
		throw new Error("Access to data was denied");
	});
	const nextLoader = vi.fn(async () => "Recovered data");
	const { rerender } = render(DetailDataTab<string, string>, {
		item: "Mars",
		loader: firstLoader,
		render: renderResult,
	});

	expect((await screen.findByRole("alert")).textContent).toContain("Access to data was denied");
	await rerender({ loader: nextLoader });
	expect(nextLoader).not.toHaveBeenCalled();

	await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
	expect(await screen.findByText("Recovered data")).toBeTruthy();
	expect(nextLoader).toHaveBeenCalledExactlyOnceWith("Mars");
});
