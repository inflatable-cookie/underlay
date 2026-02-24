import { describe, expect, it, vi } from "vitest";

async function loadKeyboardModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = <T>(value: T) => value;
	return await import("../../src/patterns/keyboard-shortcuts.svelte");
}

function createKeyboardEventLike(key: string, modifiers?: {
	ctrl?: boolean;
	alt?: boolean;
	shift?: boolean;
	meta?: boolean;
}) {
	return {
		key,
		ctrlKey: modifiers?.ctrl ?? false,
		altKey: modifiers?.alt ?? false,
		shiftKey: modifiers?.shift ?? false,
		metaKey: modifiers?.meta ?? false,
		preventDefault: vi.fn(),
	} as unknown as KeyboardEvent & { preventDefault: ReturnType<typeof vi.fn> };
}

describe("patterns/keyboard-shortcuts.svelte.ts", () => {
	it("registers, unregisters, and executes highest-priority matching shortcut", async () => {
		const { createKeyboardShortcuts } = await loadKeyboardModule();
		const manager = createKeyboardShortcuts();

		const whenFalse = vi.fn(() => false);
		const lowerPriority = vi.fn();
		const higherPriority = vi.fn();
		const escapeHandler = vi.fn();

		manager.register("ctrl+k", lowerPriority, { priority: 1 });
		const unregisterHigh = manager.register("ctrl+k", higherPriority, {
			priority: 10,
			when: whenFalse,
		});
		manager.register("Escape", escapeHandler);

		const ctrlK = createKeyboardEventLike("k", { ctrl: true });
		manager.handleKeydown(ctrlK);
		expect(lowerPriority).toHaveBeenCalledTimes(1);
		expect(higherPriority).toHaveBeenCalledTimes(0);
		expect(ctrlK.preventDefault).toHaveBeenCalledTimes(1);
		expect(whenFalse).toHaveBeenCalledTimes(1);

		unregisterHigh();
		manager.handleKeydown(ctrlK);
		expect(lowerPriority).toHaveBeenCalledTimes(2);

		const esc = createKeyboardEventLike("Escape");
		manager.handleKeydown(esc);
		expect(escapeHandler).toHaveBeenCalledTimes(1);
		expect(esc.preventDefault).toHaveBeenCalledTimes(1);

		manager.unregister("ctrl+k");
		manager.handleKeydown(ctrlK);
		expect(lowerPriority).toHaveBeenCalledTimes(2);
	});

	it("does not fire for non-matching keys and supports modifier sorting", async () => {
		const { createKeyboardShortcuts } = await loadKeyboardModule();
		const manager = createKeyboardShortcuts();

		const handler = vi.fn();
		manager.register("shift+ctrl+p", handler, { description: "Open palette" });
		manager.register("control", vi.fn());

		const wrong = createKeyboardEventLike("p", { ctrl: true });
		manager.handleKeydown(wrong);
		expect(handler).toHaveBeenCalledTimes(0);
		expect(wrong.preventDefault).toHaveBeenCalledTimes(0);

		const matchSorted = createKeyboardEventLike("p", { shift: true, ctrl: true });
		manager.handleKeydown(matchSorted);
		expect(handler).toHaveBeenCalledTimes(1);
		expect(matchSorted.preventDefault).toHaveBeenCalledTimes(1);

		const modifierOnly = createKeyboardEventLike("Control", { ctrl: true });
		manager.handleKeydown(modifierOnly);
		expect(modifierOnly.preventDefault).toHaveBeenCalledTimes(0);
	});
});
