import { describe, expect, it } from "vitest";
import { createAiRoutingOpsController } from "@decodelabs/underlay/runtime/ai";
import { configureAuth } from "@decodelabs/underlay/runtime/auth";
import { createKeyboardShortcuts } from "@decodelabs/underlay/runtime/browser";
import { createSelectionHistory } from "@decodelabs/underlay/runtime/data";
import { createToastStore } from "@decodelabs/underlay/runtime/feedback";
import { createFormState } from "@decodelabs/underlay/runtime/forms";
import { formatFileSize } from "@decodelabs/underlay/runtime/media";
import { configureNavigationContext } from "@decodelabs/underlay/runtime/navigation";
import { createLocalSearchFns } from "@decodelabs/underlay/runtime/relations";

describe("runtime package compatibility", () => {
	it("exposes retained runtime subpaths in a Svelte-aware test environment", () => {
		expect(typeof createAiRoutingOpsController).toBe("function");
		expect(typeof configureAuth).toBe("function");
		expect(typeof createKeyboardShortcuts).toBe("function");
		expect(typeof createSelectionHistory).toBe("function");
		expect(typeof createToastStore).toBe("function");
		expect(typeof createFormState).toBe("function");
		expect(formatFileSize(1024)).toBe("1.0 KB");
		expect(typeof configureNavigationContext).toBe("function");
		expect(typeof createLocalSearchFns).toBe("function");
	});
});
