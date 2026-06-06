import { describe, expect, it, vi } from "vitest";

type BlobUploadModule = typeof import("../../src/patterns/blob-upload.js");

async function loadMediaWorkflowModule(mocks?: {
  hashImpl?: (file: File) => Promise<string>;
  uploadImpl?: BlobUploadModule["uploadToBlob"];
}) {
  vi.resetModules();
  vi.doMock("../../src/patterns/blob-upload.js", () => ({
    computeFileHash: vi.fn(mocks?.hashImpl ?? (async () => "hash-1")),
    uploadToBlob: vi.fn(mocks?.uploadImpl ?? (async () => undefined)),
  }));

  const mod = await import("../../src/patterns/media-workflow.js");
  const blob = await import("../../src/patterns/blob-upload.js");
  return { mod, blob };
}

function fakeFile(overrides?: Partial<File>): File {
  return {
    name: "photo.jpg",
    type: "image/jpeg",
    size: 1234,
    ...overrides,
  } as File;
}

describe("patterns/media-workflow", () => {
  it("loads and merges browse pages with stable defaults", async () => {
    const { mod } = await loadMediaWorkflowModule();
    const listPage = vi.fn(async () => ({
      data: [{ id: "m-1" }],
    }));

    const page = await mod.loadMediaBrowsePage({ listPage });

    expect(listPage).toHaveBeenCalledWith({
      cursor: undefined,
      limit: 12,
    });
    expect(page).toEqual({
      items: [{ id: "m-1" }],
      nextCursor: null,
      hasMore: false,
    });
    expect(mod.mergeMediaBrowseItems([{ id: "old" }], page.items)).toEqual(
      page.items,
    );
    expect(
      mod.mergeMediaBrowseItems([{ id: "old" }], page.items, "cursor-1"),
    ).toEqual([{ id: "old" }, { id: "m-1" }]);
    expect(mod.createResetMediaBrowseState()).toEqual({
      items: [],
      nextCursor: null,
      hasMore: false,
    });
  });

  it("keeps pipeline upload sequencing and plan defaults behind the public barrel", async () => {
    const events: string[] = [];
    const { mod, blob } = await loadMediaWorkflowModule({
      uploadImpl: async (plan, _file, callbacks) => {
        events.push("upload");
        callbacks?.onProgress?.({ loaded: 1, total: 1, percent: 100 });
        expect(plan).toEqual({
          uploadUrl: "https://upload",
          method: "PUT",
          requiredHeaders: {},
          expiresAt: "2099-01-01T00:00:00Z",
          maxBytes: mod.DEFAULT_MEDIA_UPLOAD_MAX_FILE_SIZE,
          allowedContentTypes: [],
          objectKey: "",
        });
      },
    });
    const onProgress = vi.fn();
    const createMedia = vi.fn(async () => {
      events.push("create");
      return { id: "media-1" };
    });
    const initiateUpload = vi.fn(async () => {
      events.push("initiate");
      return {
        versionId: "v-1",
        uploadPlan: {
          uploadUrl: "https://upload",
          method: "PUT",
          headers: null,
          expiresAt: "2099-01-01T00:00:00Z",
        },
      };
    });
    const finaliseUpload = vi.fn(async () => {
      events.push("finalise");
    });

    const result = await mod.createMediaAndUpload({
      file: fakeFile(),
      title: undefined,
      visibility: "private",
      onProgress,
      detectKind: () => "image",
      createMedia,
      initiateUpload,
      finaliseUpload,
    });

    expect(result).toEqual({ mediaId: "media-1", hash: "hash-1" });
    expect(createMedia).toHaveBeenCalledWith({
      kind: "image",
      visibility: "public",
      originalFilename: "photo.jpg",
      title: null,
    });
    expect(initiateUpload).toHaveBeenCalledWith("media-1", {
      contentType: "image/jpeg",
      contentLength: 1234,
    });
    expect(finaliseUpload).toHaveBeenCalledWith("media-1", "v-1", {
      sha256: "hash-1",
      contentType: "image/jpeg",
    });
    expect(onProgress).toHaveBeenCalledWith({
      loaded: 1,
      total: 1,
      percent: 100,
    });
    expect(events).toEqual(["create", "initiate", "upload", "finalise"]);
    expect(blob.computeFileHash).toHaveBeenCalledWith(
      expect.objectContaining({
        name: "photo.jpg",
      }),
    );
  });
});
