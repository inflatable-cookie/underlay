<script lang="ts">
  import { onMount } from "svelte";
  import { createStableId } from "../patterns/dom";
  import type { NightfireSlashCommand } from "./slash-commands";

  interface Props {
    commands: NightfireSlashCommand[];
    query?: string;
    onQueryChange?: (query: string) => void;
    onSelect: (command: NightfireSlashCommand) => void;
    onClose?: () => void;
  }

  let {
    commands,
    query = "",
    onQueryChange = () => {},
    onSelect,
    onClose = () => {}
  }: Props = $props();

  const listboxId = createStableId("underlay-nightfire-slash-listbox");
  let queryInput: HTMLInputElement | null = $state(null);
  let selectedIndex = $state(0);

  $effect(() => {
    if (commands.length === 0) {
      selectedIndex = 0;
      return;
    }

    selectedIndex = Math.max(0, Math.min(selectedIndex, commands.length - 1));
  });

  onMount(() => {
    queryInput?.focus();
    queryInput?.select();
  });

  function selectActiveCommand() {
    const command = commands[selectedIndex];
    if (!command) {
      return;
    }

    onSelect(command);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      if (commands.length > 0) {
        selectedIndex = (selectedIndex + 1) % commands.length;
      }
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      if (commands.length > 0) {
        selectedIndex = (selectedIndex - 1 + commands.length) % commands.length;
      }
      return;
    }

    if (event.key === "Enter") {
      event.preventDefault();
      selectActiveCommand();
      return;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      onClose();
    }
  }
</script>

<div
  class="nightfire-slash-palette"
  role="dialog"
  aria-label="Slash commands"
  tabindex="-1"
  onkeydown={handleKeydown}
>
  <label class="nightfire-slash-palette__search">
    <span class="nightfire-slash-palette__label">Filter commands</span>
    <input
      bind:this={queryInput}
      class="nightfire-slash-palette__input"
      type="text"
      value={query}
      oninput={(event) => {
        selectedIndex = 0;
        onQueryChange((event.currentTarget as HTMLInputElement).value);
      }}
      aria-controls={listboxId}
      aria-activedescendant={commands[selectedIndex] ? `${listboxId}-${commands[selectedIndex].id}` : undefined}
      placeholder="Search block types"
    />
  </label>

  <ul
    id={listboxId}
    class="nightfire-slash-palette__list"
    role="listbox"
    aria-label="Available slash commands"
  >
    {#if commands.length > 0}
      {#each commands as command, index (command.id)}
        <li>
          <button
            id={`${listboxId}-${command.id}`}
            type="button"
            role="option"
            class:selected={index === selectedIndex}
            aria-selected={index === selectedIndex}
            class="nightfire-slash-palette__item"
            onclick={() => onSelect(command)}
            onmousemove={() => {
              selectedIndex = index;
            }}
          >
            <span class="nightfire-slash-palette__item-label">{command.label}</span>
            {#if command.description}
              <span class="nightfire-slash-palette__item-description">{command.description}</span>
            {/if}
          </button>
        </li>
      {/each}
    {:else}
      <li class="nightfire-slash-palette__empty" aria-live="polite">
        No matching commands.
      </li>
    {/if}
  </ul>
</div>

<style>
  .nightfire-slash-palette {
    display: grid;
    gap: var(--underlay-space-2, 0.5rem);
    padding: var(--underlay-space-3, 0.75rem);
    border-radius: var(--underlay-radius-md, 0.5rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background:
      linear-gradient(180deg, rgba(15, 23, 42, 0.98), rgba(15, 23, 42, 0.94)),
      var(--underlay-color-surface, rgba(15, 23, 42, 0.96));
    box-shadow: 0 18px 45px rgba(15, 23, 42, 0.28);
  }

  .nightfire-slash-palette__search {
    display: grid;
    gap: var(--underlay-space-1, 0.25rem);
  }

  .nightfire-slash-palette__label {
    font-size: calc(1em * var(--underlay-font-scale-xxs, 0.72));
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.82));
  }

  .nightfire-slash-palette__input {
    width: 100%;
    padding: var(--underlay-space-2, 0.5rem) var(--underlay-space-3, 0.75rem);
    border-radius: var(--underlay-radius-sm, 0.35rem);
    border: 1px solid var(--underlay-color-border-strong, rgba(148, 163, 184, 0.35));
    background: rgba(15, 23, 42, 0.82);
    color: var(--underlay-color-text, #f8fafc);
  }

  .nightfire-slash-palette__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: grid;
    gap: var(--underlay-space-1, 0.25rem);
    max-height: 15rem;
    overflow: auto;
  }

  .nightfire-slash-palette__item {
    width: 100%;
    display: grid;
    gap: 0.15rem;
    text-align: left;
    padding: var(--underlay-space-2, 0.5rem) var(--underlay-space-3, 0.75rem);
    border: 1px solid transparent;
    border-radius: var(--underlay-radius-sm, 0.35rem);
    background: transparent;
    color: inherit;
    cursor: pointer;
  }

  .nightfire-slash-palette__item:hover,
  .nightfire-slash-palette__item.selected {
    border-color: rgba(96, 165, 250, 0.32);
    background: rgba(59, 130, 246, 0.14);
  }

  .nightfire-slash-palette__item-label {
    font-weight: 600;
  }

  .nightfire-slash-palette__item-description,
  .nightfire-slash-palette__empty {
    font-size: calc(1em * var(--underlay-font-scale-xs, 0.82));
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.82));
  }

  .nightfire-slash-palette__empty {
    padding: var(--underlay-space-2, 0.5rem) var(--underlay-space-3, 0.75rem);
  }
</style>
