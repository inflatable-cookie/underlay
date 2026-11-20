import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { beforeEach, expect, it, vi } from "vitest";
import type { QueryParams } from "../../src/client/query";
import { configureAuth } from "../../src/patterns/auth";
import EntityList from "../../src/templates/EntityList.svelte";
import type {
	EntityListDataLoader,
	FilterConfig,
	TemplateFilterOption,
} from "../../src/templates/template.types";

const project = { id: "1", name: "Project Mars" };
const columns = [{ id: "name", label: "Name" }];
const loadProjects: EntityListDataLoader<typeof project> = async () => ({
	data: [project],
	total: 2,
});

const onRefresh = vi.fn(async () => "new-token");

beforeEach(() => {
	onRefresh.mockClear();
	configureAuth({
		getToken: () => "old-token",
		onRefresh,
		getAuthLoading: () => false,
		getCurrentUser: () => ({ id: "user" }),
	});
});

it("keeps the search draft visible and fetches once when a delayed parent echoes the query", async () => {
	const dataLoader = vi.fn(loadProjects);
	const onQueryChange = vi.fn<(query: QueryParams) => void>();
	const { rerender } = render(EntityList<typeof project>, {
		columns,
		presentation: "table",
		dataLoader,
		filters: [{ id: "search", type: "search", label: "Search" }],
		query: {},
		onQueryChange,
	});

	await screen.findByText(project.name);
	const search = screen.getByRole("searchbox", { name: "Search" });
	await fireEvent.input(search, { target: { value: "Mars" } });
	expect(search).toHaveProperty("value", "Mars");
	expect(dataLoader).toHaveBeenCalledTimes(1);

	await waitFor(() => expect(dataLoader).toHaveBeenCalledTimes(2));
	const query = onQueryChange.mock.lastCall?.[0];
	expect(query?.filters).toEqual([{ field: "search", operator: "like", value: "%Mars%" }]);
	await rerender({ query });
	expect(dataLoader).toHaveBeenCalledTimes(2);

	await rerender({ query: {} });
	await waitFor(() => expect(dataLoader).toHaveBeenCalledTimes(3));
	expect(search).toHaveProperty("value", "");
});

it("replaces an in-flight filter option request when its key changes", async () => {
	let completeFirst: (options: TemplateFilterOption[]) => void = () => {};
	const first = new Promise<TemplateFilterOption[]>((resolve) => {
		completeFirst = resolve;
	});
	const loadOptions = vi
		.fn<NonNullable<FilterConfig["loadOptions"]>>()
		.mockReturnValueOnce(first)
		.mockResolvedValue([{ value: "fresh", label: "Fresh" }]);
	const category: FilterConfig = { id: "category", type: "select", label: "Category", loadOptions };
	const { rerender } = render(EntityList<typeof project>, {
		presentation: "table",
		columns,
		dataLoader: loadProjects,
		filters: [{ ...category, loadKey: "first" }],
	});

	await waitFor(() => expect(loadOptions).toHaveBeenCalledTimes(1));
	await rerender({ filters: [{ ...category, loadKey: "second" }] });
	await waitFor(() => expect(loadOptions).toHaveBeenCalledTimes(2));
	completeFirst([{ value: "stale", label: "Stale" }]);
	await fireEvent.click(screen.getByRole("button", { name: "Category" }));

	expect(await screen.findByRole("option", { name: "Fresh" })).toBeTruthy();
	expect(screen.queryByRole("option", { name: "Stale" })).toBeNull();
	expect(loadOptions).toHaveBeenCalledTimes(2);
});

it("retries a loaded reorder session with a refreshed token", async () => {
	const expired = Object.assign(new Error("Expired"), { status: 401 });
	const loadItems = vi
		.fn()
		.mockRejectedValueOnce(expired)
		.mockResolvedValueOnce({ items: [{ id: "2", name: "Loaded project" }, project] });
	render(EntityList<typeof project>, {
		title: "Projects",
		columns,
		presentation: "table",
		dataLoader: loadProjects,
		reorder: { strategy: "loaded", enabled: true, loadItems, handler: vi.fn() },
	});

	await fireEvent.click(await screen.findByRole("button", { name: "Reorder items" }));
	expect(await screen.findByRole("button", { name: "Cancel reorder" })).toBeTruthy();
	expect(onRefresh).toHaveBeenCalledTimes(1);
	expect(loadItems.mock.calls.map(([, token]) => token)).toEqual(["old-token", "new-token"]);
});

it.each([undefined, "Projects"])(
	"can retry a list error without filters (title %s)",
	async (title) => {
		const dataLoader = vi.fn(loadProjects).mockRejectedValueOnce(new Error("List unavailable"));
		render(EntityList<typeof project>, { title, columns, dataLoader, presentation: "table" });

		expect(await screen.findByText("List unavailable")).toBeTruthy();
		await fireEvent.click(screen.getByRole("button", { name: "Retry" }));

		expect(await screen.findByText(project.name)).toBeTruthy();
		expect(dataLoader).toHaveBeenCalledTimes(2);
	},
);

it("shows a capabilities error without hiding results and retries the capabilities loader", async () => {
	const capabilitiesLoader = vi
		.fn()
		.mockRejectedValueOnce(new Error("List options unavailable"))
		.mockResolvedValueOnce({ filters: [], variants: [{ id: "active", label: "Active projects" }] });
	render(EntityList<typeof project>, {
		columns,
		presentation: "table",
		dataLoader: loadProjects,
		capabilitiesLoader,
	});

	expect(await screen.findByText("List options unavailable")).toBeTruthy();
	expect(await screen.findByText(project.name)).toBeTruthy();
	await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
	expect(await screen.findByRole("button", { name: "Active projects" })).toBeTruthy();
	expect(capabilitiesLoader).toHaveBeenCalledTimes(2);
});
