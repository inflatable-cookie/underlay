<script lang="ts">
  import Button from "../Button.svelte";
  import EmptyState from "../EmptyState.svelte";
  import FormError from "../FormError.svelte";
  import TextButton from "../TextButton.svelte";
  import TextInput from "../TextInput.svelte";

  export interface PasskeyManagerItem {
    id: string;
    name: string;
    createdAt?: string | null;
    lastUsedAt?: string | null;
    deviceLabel?: string | null;
  }

  interface EmptyStateConfig {
    title: string;
    description?: string;
  }

  interface Props {
    passkeys: PasskeyManagerItem[];
    onRegister?: () => void | Promise<void>;
    onRename?: (id: string, name: string) => void | Promise<void>;
    onDelete?: (id: string) => void | Promise<void>;
    emptyState?: EmptyStateConfig;
    registerLabel?: string;
    disabled?: boolean;
  }

  let {
    passkeys,
    onRegister,
    onRename,
    onDelete,
    emptyState = {
      title: "No passkeys",
      description: "Add a passkey for faster, more secure sign-in.",
    },
    registerLabel = "Add passkey",
    disabled = false,
  }: Props = $props();

  let editingId = $state<string | null>(null);
  let draftName = $state("");
  let error = $state<string | null>(null);

  function formatTimestamp(value: string | null | undefined): string | null {
    if (!value) {
      return null;
    }

    const parsed = new Date(value);
    if (Number.isNaN(parsed.getTime())) {
      return value;
    }

    return parsed.toLocaleString();
  }

  function beginRename(item: PasskeyManagerItem): void {
    error = null;
    editingId = item.id;
    draftName = item.name;
  }

  function cancelRename(): void {
    editingId = null;
    draftName = "";
    error = null;
  }

  async function submitRename(id: string): Promise<void> {
    const nextName = draftName.trim();
    if (!nextName) {
      error = "Passkey name is required";
      return;
    }

    try {
      await onRename?.(id, nextName);
      cancelRename();
    } catch (cause) {
      error =
        cause instanceof Error ? cause.message : "Failed to rename passkey";
    }
  }

  async function handleDelete(id: string): Promise<void> {
    error = null;
    try {
      await onDelete?.(id);
    } catch (cause) {
      error =
        cause instanceof Error ? cause.message : "Failed to delete passkey";
    }
  }

  async function handleRegister(): Promise<void> {
    error = null;
    try {
      await onRegister?.();
    } catch (cause) {
      error =
        cause instanceof Error ? cause.message : "Failed to add passkey";
    }
  }
</script>

<section class="underlay-passkey-manager">
  <header class="underlay-passkey-manager__header">
    <div>
      <h3 class="underlay-passkey-manager__title">Saved passkeys</h3>
      <p class="underlay-passkey-manager__subtitle">
        Manage the passkeys allowed to sign in to this account.
      </p>
    </div>

    {#if onRegister}
      <Button
        size="sm"
        variant="subtle"
        disabled={disabled}
        onclick={handleRegister}
      >
        {registerLabel}
      </Button>
    {/if}
  </header>

  <FormError message={error} />

  {#if passkeys.length === 0}
    <EmptyState
      variant="compact"
      title={emptyState.title}
      description={emptyState.description}
      actionLabel={onRegister ? registerLabel : undefined}
      onaction={onRegister ? handleRegister : undefined}
    />
  {:else}
    <ul class="underlay-passkey-manager__list">
      {#each passkeys as item (item.id)}
        <li class="underlay-passkey-manager__item">
          <div class="underlay-passkey-manager__content">
            {#if editingId === item.id}
              <div class="underlay-passkey-manager__editor">
                <TextInput
                  bind:value={draftName}
                  disabled={disabled}
                  aria-label={`Rename ${item.name}`}
                />
                <div class="underlay-passkey-manager__actions">
                  <TextButton disabled={disabled} onclick={() => submitRename(item.id)}>
                    Save
                  </TextButton>
                  <TextButton disabled={disabled} onclick={cancelRename}>
                    Cancel
                  </TextButton>
                </div>
              </div>
            {:else}
              <div class="underlay-passkey-manager__details">
                <div class="underlay-passkey-manager__row">
                  <p class="underlay-passkey-manager__name">{item.name}</p>
                  <div class="underlay-passkey-manager__actions">
                    {#if onRename}
                      <TextButton disabled={disabled} onclick={() => beginRename(item)}>
                        Rename
                      </TextButton>
                    {/if}
                    {#if onDelete}
                      <TextButton
                        variant="danger"
                        disabled={disabled}
                        onclick={() => handleDelete(item.id)}
                      >
                        Delete
                      </TextButton>
                    {/if}
                  </div>
                </div>

                <div class="underlay-passkey-manager__meta">
                  {#if item.deviceLabel}
                    <span>{item.deviceLabel}</span>
                  {/if}
                  {#if formatTimestamp(item.createdAt)}
                    <span>Added {formatTimestamp(item.createdAt)}</span>
                  {/if}
                  {#if formatTimestamp(item.lastUsedAt)}
                    <span>Last used {formatTimestamp(item.lastUsedAt)}</span>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .underlay-passkey-manager {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-passkey-manager__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-passkey-manager__title,
  .underlay-passkey-manager__name,
  .underlay-passkey-manager__subtitle {
    margin: 0;
  }

  .underlay-passkey-manager__subtitle,
  .underlay-passkey-manager__meta {
    color: var(--underlay-color-text-muted, #64748b);
    font-size: var(--underlay-font-size-sm, 0.875rem);
  }

  .underlay-passkey-manager__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-passkey-manager__item {
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.24));
    border-radius: var(--underlay-radius-md, 0.75rem);
    padding: var(--underlay-space-3, 0.75rem);
  }

  .underlay-passkey-manager__content,
  .underlay-passkey-manager__editor,
  .underlay-passkey-manager__details {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-passkey-manager__row,
  .underlay-passkey-manager__actions,
  .underlay-passkey-manager__meta {
    display: flex;
    gap: var(--underlay-space-2, 0.5rem);
    flex-wrap: wrap;
  }

  .underlay-passkey-manager__row {
    align-items: center;
    justify-content: space-between;
  }
</style>
