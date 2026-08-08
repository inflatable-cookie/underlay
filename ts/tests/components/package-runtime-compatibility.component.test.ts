import { describe, expect, it } from "vitest";
import { createAiRoutingOpsController } from "@inflatable-cookie/underlay/runtime/ai";
import { configureAuth } from "@inflatable-cookie/underlay/runtime/auth";
import { createKeyboardShortcuts } from "@inflatable-cookie/underlay/runtime/browser";
import { createClientPagination } from "@inflatable-cookie/underlay/runtime/collections";
import { createSelectionHistory } from "@inflatable-cookie/underlay/runtime/data";
import { createToastStore } from "@inflatable-cookie/underlay/runtime/feedback";
import { createFormState } from "@inflatable-cookie/underlay/runtime/forms";
import { formatFileSize } from "@inflatable-cookie/underlay/runtime/media";
import { canPreviewMedia } from "@inflatable-cookie/underlay/runtime/media/detail";
import { MediaKind } from "@inflatable-cookie/underlay/runtime/media/types";
import { validateFileSize } from "@inflatable-cookie/underlay/runtime/media/upload";
import { configureNavigationContext } from "@inflatable-cookie/underlay/runtime/navigation";
import { createLocalSearchFns } from "@inflatable-cookie/underlay/runtime/relations";
import { createReorderController } from "@inflatable-cookie/underlay/runtime/reorder";
import { useBatchSelection } from "@inflatable-cookie/underlay/runtime/selection";

describe("runtime package compatibility", () => {
	it("exposes retained runtime subpaths in a Svelte-aware test environment", () => {
		expect(typeof createAiRoutingOpsController).toBe("function");
		expect(typeof configureAuth).toBe("function");
		expect(typeof createKeyboardShortcuts).toBe("function");
		expect(typeof createClientPagination).toBe("function");
		expect(typeof createSelectionHistory).toBe("function");
		expect(typeof createToastStore).toBe("function");
		expect(typeof createFormState).toBe("function");
		expect(formatFileSize(1024)).toBe("1.0 KB");
		expect(typeof canPreviewMedia).toBe("function");
		expect(MediaKind.Image).toBe("image");
		expect(validateFileSize(new File([""], "empty.txt"), 1024)).toBe(true);
		expect(typeof configureNavigationContext).toBe("function");
		expect(typeof createLocalSearchFns).toBe("function");
		expect(typeof createReorderController).toBe("function");
		expect(typeof useBatchSelection).toBe("function");
	});
});
