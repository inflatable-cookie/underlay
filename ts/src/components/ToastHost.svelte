<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import type { ToastStore, Toast } from "../patterns/toasts";

  export let store: ToastStore;
  export let autoDismissMs = 6000;

  let toasts: Toast[] = [];

  const timers = new Map<string, ReturnType<typeof setTimeout>>();

  const unsubscribe = store.toasts.subscribe((value) => {
    toasts = value;
  });

  onDestroy(() => {
    unsubscribe();
    for (const timer of timers.values()) clearTimeout(timer);
    timers.clear();
  });

  onMount(() => {
    const unsubscribeTimers = store.toasts.subscribe((next) => {
      const ids = new Set(next.map((t) => t.id));

      for (const [id, timer] of timers.entries()) {
        if (!ids.has(id)) {
          clearTimeout(timer);
          timers.delete(id);
        }
      }

      for (const toast of next) {
        if (toast.variant === "error") continue;
        if (autoDismissMs <= 0) continue;
        if (timers.has(toast.id)) continue;

        const timer = setTimeout(() => {
          store.dismiss(toast.id);
          timers.delete(toast.id);
        }, autoDismissMs);

        timers.set(toast.id, timer);
      }
    });

    return () => {
      unsubscribeTimers();
    };
  });

</script>

<div class="underlay-toast-host" aria-live="polite" aria-relevant="additions">
  {#each toasts as toast (toast.id)}
    <div
      class={`underlay-toast underlay-toast--${toast.variant}`}
      role={toast.variant === "error" ? "alert" : "status"}
    >
      <div class="underlay-toast__content">
        {#if toast.title}
          <div class="underlay-toast__title">{toast.title}</div>
        {/if}
        <div class="underlay-toast__message">{toast.message}</div>
      </div>
      <button
        type="button"
        class="underlay-toast__dismiss"
        aria-label="Dismiss"
        onclick={() => store.dismiss(toast.id)}
      >
        ×
      </button>
    </div>
  {/each}
</div>

<style>
  :global(.underlay-toast-host) {
    position: fixed;
    right: 1rem;
    bottom: 1rem;
    z-index: 80;

    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    width: min(28rem, calc(100vw - 2rem));
  }

  :global(.underlay-toast) {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 0.75rem;

    border-radius: 0.75rem;
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5))
      );
    background: var(
      --underlay-color-bg-surface,
      var(--underlay-color-bg-surface, #020617)
    );
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));

    padding: 0.75rem 0.85rem;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.55);
  }

  :global(.underlay-toast--success) {
    border-color: rgba(34, 197, 94, 0.5);
  }

  :global(.underlay-toast--error) {
    border-color: rgba(239, 68, 68, 0.55);
  }

  :global(.underlay-toast--info) {
    border-color: rgba(59, 130, 246, 0.5);
  }

  :global(.underlay-toast__title) {
    font-weight: 650;
    margin-bottom: 0.2rem;
  }

  :global(.underlay-toast__message) {
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    line-height: 1.35;
  }

  :global(.underlay-toast__dismiss) {
    border: none;
    background: transparent;
    color: inherit;
    opacity: 0.8;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0.15rem 0.35rem;
    border-radius: 0.5rem;
  }

  :global(.underlay-toast__dismiss:hover) {
    opacity: 1;
    background: rgba(148, 163, 184, 0.12);
  }
</style>
