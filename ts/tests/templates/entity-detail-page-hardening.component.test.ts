import { fireEvent, render, screen } from "@testing-library/svelte";
import { createRawSnippet } from "svelte";
import { expect, it, vi } from "vitest";
import EntityDetailPage from "../../src/templates/EntityDetailPage.svelte";
import type { DetailTabConfig } from "../../src/templates/template.types";

const project = { id: "1" };

it("keeps confirmation open and pending until the action settles, then announces rejection", async () => {
	let rejectAction: (error: Error) => void = () => {};
	const handler = vi.fn(
		() =>
			new Promise<void>((_, reject) => {
				rejectAction = reject;
			}),
	);
	render(EntityDetailPage<typeof project>, {
		title: "Project",
		item: project,
		actions: [{ label: "Archive", tone: "danger", confirm: true, handler }],
	});

	await fireEvent.click(screen.getByRole("button", { name: "Project actions", expanded: false }));
	await fireEvent.click(await screen.findByRole("menuitem", { name: "Archive" }));
	await fireEvent.click(await screen.findByRole("button", { name: "Archive" }));

	expect(screen.getByRole("alertdialog")).toBeTruthy();
	expect(screen.getByRole("button", { name: "Working…" })).toHaveProperty("disabled", true);
	expect(screen.getByRole("button", { name: "Cancel" })).toHaveProperty("disabled", true);
	expect(handler).toHaveBeenCalledTimes(1);

	rejectAction(new Error("The project could not be archived"));
	expect(await screen.findByText("The project could not be archived")).toBeTruthy();
	expect(screen.queryByRole("alertdialog")).toBeNull();
});

it("preserves a visited data tab when the tab configuration is reordered", async () => {
	const renderTab = createRawSnippet<[unknown, typeof project | null]>((data) => ({
		render: () => "<span></span>",
		setup: (element) => {
			const value = data();
			element.textContent = typeof value === "string" ? value : "";
		},
	}));
	const firstLoader = vi.fn(async () => "First tab data");
	const secondLoader = vi.fn(async () => "Second tab data");
	const tabs: DetailTabConfig<typeof project>[] = [
		{ id: "first", label: "First", dataLoader: firstLoader, render: renderTab },
		{ id: "second", label: "Second", dataLoader: secondLoader, render: renderTab },
	];
	const { rerender } = render(EntityDetailPage<typeof project>, {
		title: "Project",
		item: project,
		tabs,
		tabsHistoryKey: "",
		keepMountedTabs: true,
	});

	const firstContent = await screen.findByText("First tab data");
	await fireEvent.click(screen.getByRole("tab", { name: "Second" }));
	const secondContent = await screen.findByText("Second tab data");
	await rerender({ tabs: [tabs[1], tabs[0]] });

	expect(screen.getByText("First tab data")).toBe(firstContent);
	expect(screen.getByText("Second tab data")).toBe(secondContent);
	expect(firstLoader).toHaveBeenCalledTimes(1);
	expect(secondLoader).toHaveBeenCalledTimes(1);
});
