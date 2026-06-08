<script lang="ts">
  import { onMount } from "svelte";
  import { AlertDialog, IconButton, Menu } from "@poodle/svelte";
  import type { ControlSize, MenuItem } from "@poodle/svelte";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import type { ToastStore } from "../runtime/feedback";
  import type { TemplateSurface } from "./template.types";

  type CopyItem = {
    label: string;
    text: string;
    successMessage: string;
    failureMessage?: string;
  };

  type MenuAction = {
    label?: string;
    onSelect?: (() => void | Promise<void>) | undefined;
    disabled?: boolean;
    destructive?: boolean;
    separator?: boolean;
  };

  type DeleteConfig = {
    entityLabel?: string | null;
    title: string;
    description: string;
    confirmLabel: string;
    execute: () => void | Promise<void>;
  };

  interface Props {
    toastStore?: ToastStore;
    copies?: CopyItem[];
    trigger?: TemplateSurface;
    children?: TemplateSurface;
    content?: TemplateSurface;
    align?: "start" | "center" | "end";
    showTrigger?: boolean;
    triggerAriaLabel?: string;
    triggerTooltip?: string;
    onEdit?: (() => void | Promise<void>) | undefined;
    editLabel?: string;
    editDisabled?: boolean;
    customActions?: MenuAction[];
    deleteConfig?: DeleteConfig | undefined;
    onDeleteSuccess?: (() => void | Promise<void>) | undefined;
  }

  let {
    toastStore,
    copies = [],
    trigger,
    children,
    content,
    align = "end",
    showTrigger = true,
    triggerAriaLabel = "Entity actions",
    triggerTooltip = "Actions",
    onEdit,
    editLabel = "Edit",
    editDisabled = false,
    customActions = [],
    deleteConfig,
    onDeleteSuccess,
  }: Props = $props();

  const fallbackToastStore = useToasts();
  const resolvedToastStore = $derived(toastStore ?? fallbackToastStore);

  let deleteOpen = $state(false);
  let deleteBusy = $state(false);
  let isNarrowViewport = $state(false);
  const responsiveTriggerSize = $derived<ControlSize>(isNarrowViewport ? "sm" : "md");

  const menuEntries = $derived.by(() => {
    const entries: Array<
      | ({ key: string } & MenuAction)
      | ({ key: string } & CopyItem)
      | { key: string; separator: true }
    > = [];

    if (onEdit) {
      entries.push({
        key: "edit",
        label: editLabel,
        disabled: editDisabled,
        onSelect: () => void onEdit(),
      });
    }

    customActions.forEach((action, index) => {
      entries.push({
        key: `custom-${index}`,
        ...action,
      });
    });

    if (deleteConfig) {
      if (entries.length > 0) {
        entries.push({ key: "separator-delete", separator: true });
      }

      entries.push({
        key: "delete",
        label: deleteConfig.confirmLabel,
        destructive: true,
        disabled: deleteBusy,
        onSelect: () => {
          deleteOpen = true;
        },
      });
    }

    const copyEntries = copies
      .filter((copy) => Boolean(copy.text))
      .map((copy, index) => ({
        key: `copy-${index}`,
        ...copy,
      }));

    if (entries.length > 0 && copyEntries.length > 0) {
      entries.push({ key: "separator-copy", separator: true });
    }

    entries.push(...copyEntries);

    return entries;
  });

  const menuItems = $derived<MenuItem[]>(
    menuEntries.map((entry) =>
      "separator" in entry && entry.separator
        ? { value: entry.key, label: "", kind: "separator" }
        : {
            value: entry.key,
            label: entry.label ?? "Action",
            disabled: "disabled" in entry ? entry.disabled : undefined,
            tone: "destructive" in entry && entry.destructive ? "danger" : undefined,
          }
    )
  );

  const placement = $derived(
    align === "start" ? "bottom-start" : align === "center" ? "bottom" : "bottom-end"
  );

  async function handleAction(value: string): Promise<void> {
    const entry = menuEntries.find((item) => item.key === value);
    if (!entry || ("separator" in entry && entry.separator)) return;

    if ("text" in entry) {
      await copyToClipboard(
        resolvedToastStore,
        entry.text,
        entry.successMessage,
        entry.failureMessage
      );
      return;
    }

    await entry.onSelect?.();
  }

  function handleMenuAction(value: string): void {
    void handleAction(value);
  }

  async function handleDeleteConfirm(): Promise<void> {
    if (!deleteConfig || deleteBusy) return;

    deleteBusy = true;
    try {
      await deleteConfig.execute();
      deleteOpen = false;
      await onDeleteSuccess?.();
    } catch (error) {
      const message = error instanceof Error ? error.message : "Delete failed";
      resolvedToastStore.push({ variant: "error", message });
    } finally {
      deleteBusy = false;
    }
  }

  onMount(() => {
    const mediaQuery = window.matchMedia("(max-width: 45rem)");
    const sync = () => {
      isNarrowViewport = mediaQuery.matches;
    };

    sync();
    mediaQuery.addEventListener("change", sync);

    return () => {
      mediaQuery.removeEventListener("change", sync);
    };
  });
</script>

{#if content}
  {@render content({
    items: menuItems,
    ariaLabel: triggerAriaLabel,
    onAction: handleMenuAction
  })}
{:else if showTrigger}
  <Menu
    items={menuItems}
    ariaLabel="Entity actions"
    triggerAriaLabel={triggerAriaLabel}
    {placement}
    onAction={handleMenuAction}
  >
    {#snippet trigger()}
      {#if trigger}
        {@render trigger()}
      {:else if children}
        {@render children()}
      {:else}
        <IconButton
          type="button"
          icon="ellipsis"
          variant="secondary"
          size={responsiveTriggerSize}
          ariaLabel={triggerAriaLabel}
          tooltip={triggerTooltip}
        />
      {/if}
    {/snippet}
  </Menu>
{/if}

{#if deleteConfig && (deleteOpen || deleteBusy)}
  <AlertDialog
    bind:open={deleteOpen}
    title={deleteConfig.title}
    description={deleteConfig.description}
    itemLabel={deleteConfig.entityLabel ? "Item" : null}
    itemValue={deleteConfig.entityLabel ?? null}
    confirmLabel={deleteBusy ? "Working..." : deleteConfig.confirmLabel}
    onConfirm={handleDeleteConfirm}
    onCancel={() => {
      if (!deleteBusy) {
        deleteOpen = false;
      }
    }}
  />
{/if}
