// @vitest-environment jsdom
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import MediaPickerWorkflow from "../../src/templates/MediaPickerWorkflow.svelte";
import MediaEditor from "../../src/nightfire/media/MediaEditor.svelte";
import MediaEditorFallbackHarness from "./MediaEditorFallbackHarness.svelte";
import MediaEditorContextHarness from "./MediaEditorContextHarness.svelte";

describe("templates/MediaPickerWorkflow", () => {
	const baseProps = () => ({
		checkDuplicate: vi.fn(async () => ({ exists: false, item: null })),
		createRecord: vi.fn(async () => ({ id: "m1", title: "Created" })),
		buildCreateRequest: vi.fn((file: File) => ({
			filename: file.name,
			mimeType: file.type || "application/octet-stream",
			sizeBytes: file.size,
		})),
		initiateUpload: vi.fn(async () => ({
			versionId: "v1",
			uploadUrl: "https://example.invalid/upload",
			method: "PUT" as const,
		})),
		buildInitiateRequest: vi.fn((file: File) => ({
			filename: file.name,
			mimeType: file.type || "application/octet-stream",
			sizeBytes: file.size,
		})),
		finaliseUpload: vi.fn(async () => ({})),
		buildFinaliseRequest: vi.fn((_file: File, sha256: string) => ({ sha256 })),
	});

	it("loads once and does not loop when listPage returns an empty page", async () => {
		const listPage = vi.fn(async () => ({ data: [], nextCursor: null, hasMore: false }));
		const { rerender } = render(MediaPickerWorkflow, {
			...baseProps(),
			open: true,
			listPage,
		});

		await waitFor(() => expect(listPage).toHaveBeenCalledTimes(1));
		await new Promise((resolve) => setTimeout(resolve, 40));
		expect(listPage).toHaveBeenCalledTimes(1);

		// Closing and reopening resets hasLoadedBrowse so the picker refreshes on next open.
		await rerender({ open: false });
		await rerender({ open: true });
		await waitFor(() => expect(listPage).toHaveBeenCalledTimes(2));
	});

	it("loads once and does not loop when listPage rejects", async () => {
		const listPage = vi.fn(async () => {
			throw new Error("Browse unavailable");
		});
		render(MediaPickerWorkflow, {
			...baseProps(),
			open: true,
			listPage,
		});

		expect(await screen.findByText("Browse unavailable")).toBeTruthy();
		await new Promise((resolve) => setTimeout(resolve, 40));
		expect(listPage).toHaveBeenCalledTimes(1);
	});
});

describe("nightfire/media/MediaEditor", () => {
	it("keeps the fallback Media ID input mounted across multi-character typing when no mediaContext exists", async () => {
		const onBlockChange = vi.fn();
		render(MediaEditorFallbackHarness, { onBlockChange });

		const input = screen.getByLabelText("Media ID") as HTMLInputElement;
		await fireEvent.input(input, { target: { value: "m" } });
		expect(screen.getByLabelText("Media ID")).toBe(input);

		await fireEvent.input(input, { target: { value: "media-123" } });
		expect(screen.getByLabelText("Media ID")).toBe(input);
		expect(input.value).toBe("media-123");
		expect(onBlockChange).toHaveBeenLastCalledWith(
			expect.objectContaining({
				data: expect.objectContaining({ media_id: "media-123" }),
			}),
		);
	});

	it("renders without mediaContext and reveals metadata fields once a media ID is entered", async () => {
		render(MediaEditor, {
			block: { type: "media", data: { media_id: "existing-id" } },
			onChange: vi.fn(),
		});

		expect((screen.getByLabelText("Media ID") as HTMLInputElement).value).toBe("existing-id");
		expect(screen.getByLabelText("Caption")).toBeTruthy();
		expect(screen.getByLabelText("Alt text")).toBeTruthy();
	});

	it("hydrates saved media metadata for a reopened block through resolveMedia", async () => {
		const onBlockChange = vi.fn();
		const resolveMedia = vi.fn(async (id: string) => ({
			id,
			thumbnailUrl: "https://example.invalid/hero.png",
			title: "Hero image",
			kind: "image",
		}));

		render(MediaEditorContextHarness, {
			mediaId: "media-1",
			resolveMedia,
			onBlockChange,
		});

		expect(await screen.findByText("Hero image")).toBeTruthy();
		expect(resolveMedia).toHaveBeenCalledWith("media-1");
		const thumbnail = document.querySelector<HTMLImageElement>(
			".underlay-media-editor__thumbnail-image",
		);
		expect(thumbnail?.getAttribute("src")).toBe("https://example.invalid/hero.png");
		// Hydration must not rewrite the block it is hydrating.
		expect(onBlockChange).not.toHaveBeenCalled();
	});

	it("ignores a stale resolveMedia response after the block moves to another id", async () => {
		let releaseFirst: (value: {
			id: string;
			title: string;
			thumbnailUrl: string | null;
			kind: string;
		}) => void = () => {};
		const resolveMedia = vi.fn((id: string) => {
			if (id === "media-a") {
				return new Promise((resolve) => {
					releaseFirst = resolve;
				});
			}
			return Promise.resolve({
				id,
				title: "Second media",
				thumbnailUrl: null,
				kind: "document",
			});
		});

		const { rerender } = render(MediaEditorContextHarness, {
			mediaId: "media-a",
			resolveMedia,
		});

		await rerender({ mediaId: "media-b" });
		expect(await screen.findByText("Second media")).toBeTruthy();

		releaseFirst({ id: "media-a", title: "First media", thumbnailUrl: null, kind: "image" });
		await new Promise((resolve) => setTimeout(resolve, 10));

		expect(screen.queryByText("First media")).toBeNull();
		expect(screen.getByText("Second media")).toBeTruthy();
	});

	it("disables Remove while a media pick is in flight", async () => {
		let releasePick: (value: null) => void = () => {};
		const pickMedia = vi.fn(
			() =>
				new Promise<null>((resolve) => {
					releasePick = resolve;
				}),
		);

		render(MediaEditorContextHarness, {
			mediaId: "media-1",
			pickMedia,
			resolveMedia: async (id: string) => ({
				id,
				title: "Existing media",
				thumbnailUrl: null,
				kind: "image",
			}),
		});

		await screen.findByText("Existing media");
		const removeButton = screen.getByRole("button", { name: "Remove" }) as HTMLButtonElement;
		expect(removeButton.disabled).toBe(false);

		await fireEvent.click(screen.getByRole("button", { name: "Change" }));
		expect(pickMedia).toHaveBeenCalledTimes(1);
		expect((screen.getByRole("button", { name: "Remove" }) as HTMLButtonElement).disabled).toBe(
			true,
		);

		releasePick(null);
		await waitFor(() => {
			expect((screen.getByRole("button", { name: "Remove" }) as HTMLButtonElement).disabled).toBe(
				false,
			);
		});
	});

	it("does not hydrate when resolveMedia is not configured", async () => {
		render(MediaEditorContextHarness, { mediaId: "media-1", pickMedia: async () => null });

		expect(screen.getByText("Selected media")).toBeTruthy();
	});
});
