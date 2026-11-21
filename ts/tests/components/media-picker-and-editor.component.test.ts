// @vitest-environment jsdom
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import MediaPickerWorkflow from "../../src/templates/MediaPickerWorkflow.svelte";
import MediaEditor from "../../src/nightfire/media/MediaEditor.svelte";
import MediaEditorFallbackHarness from "./MediaEditorFallbackHarness.svelte";

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
});
