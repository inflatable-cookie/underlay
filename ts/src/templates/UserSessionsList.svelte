<script lang="ts">
  import {
    AlertDialog,
    Callout,
    Code,
    DataTable,
    PageLoading,
    Pill as PoodlePill,
    TimeAgo,
    type TableColumn,
    type TableRow,
    type TableRowAction
  } from "@inflatable-cookie/poodle-svelte";
  import type { PillTone } from "@inflatable-cookie/poodle-svelte";
  import { getAuthConfig, useAuthenticatedData } from "../runtime/auth";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import {
    type UserSessionItem,
    type UserSessionListLoader,
    type UserSessionRevokeAction
  } from "./template.types";
  import { getUserSessionStatusTone } from "./template-types/user-tabs";
  import {
    buildUserTabRows,
    resolveUserTabCount,
    USER_TAB_LIST_FIXED_LIMIT
  } from "./user-tab-list.helpers";

  interface Props {
    userId: string;
    dataLoader: UserSessionListLoader;
    revokeAction?: UserSessionRevokeAction;
    active?: boolean;
    getStatusTone?: (status: string) => PillTone;
    onCountChange?: (count: number) => void;
  }

  let {
    userId,
    dataLoader,
    revokeAction,
    active = true,
    getStatusTone = getUserSessionStatusTone,
    onCountChange
  }: Props = $props();

  const toastStore = useToasts();

  const columns: TableColumn[] = [
    { id: "status", label: "Status", width: "100px" },
    { id: "ipAddress", label: "IP Address", width: "140px" },
    { id: "userAgent", label: "User Agent", width: "2fr", hideOnMobile: true },
    { id: "createdAt", label: "Created", width: "110px", hideOnMobile: true },
    { id: "lastUsedAt", label: "Last Used", width: "110px" }
  ];

  let hasActivated = $state(false);
  let pendingRevokeSession = $state<UserSessionItem | null>(null);
  let lastNotifiedCount = $state<number | null>(null);

  // Lazy-load gate: the first activation latches, so deactivating the tab
  // later never unloads already-fetched data.
  $effect(() => {
    if (active) {
      hasActivated = true;
    }
  });

  async function loadSessions(fetch: typeof globalThis.fetch, token: string) {
    return await dataLoader(userId, fetch, token, {
      page: 1,
      limit: USER_TAB_LIST_FIXED_LIMIT
    });
  }

  const listData = useAuthenticatedData(loadSessions, {
    defaultValue: { data: [], total: 0 },
    // Resolve auth getters lazily (configureAuth may run after setup) and hold
    // the auto-fetch until the tab first activates.
    getAuthLoading: () => !hasActivated || (getAuthConfig()?.getAuthLoading?.() ?? false),
    getCurrentUser: () => getAuthConfig()?.getCurrentUser?.() ?? null
  });

  const sessions = $derived(listData.data?.data ?? []);
  const rows = $derived(
    buildUserTabRows(sessions, (session) => ({
      status: session.status,
      ipAddress: session.ipAddress ?? "",
      userAgent: session.userAgent ?? "",
      createdAt: session.createdAt,
      lastUsedAt: session.lastUsedAt
    }))
  );
  const totalCount = $derived(resolveUserTabCount(listData.data, sessions.length));

  $effect(() => {
    if (lastNotifiedCount !== totalCount) {
      lastNotifiedCount = totalCount;
      onCountChange?.(totalCount);
    }
  });

  function getRowActions(row: TableRow<UserSessionItem>): TableRowAction[] {
    const session = row.data;
    if (!session) {
      return [];
    }

    return [
      ...(revokeAction && session.status === "active"
        ? [{ value: "revoke", label: "Revoke", tone: "danger" as const }]
        : []),
      { value: "copy-session-id", label: "Copy Session ID" }
    ];
  }

  function handleRowActionSelect(event: { row: TableRow; action: TableRowAction }) {
    // Poodle's DataTable emits a non-generic TableRow; our row payload is
    // TableRow<UserSessionItem> at runtime.
    const session = event.row.data as UserSessionItem | undefined;
    if (!session) {
      return;
    }

    switch (event.action.value) {
      case "revoke":
        pendingRevokeSession = session;
        break;
      case "copy-session-id":
        void copyToClipboard(toastStore, session.id, "Copied session ID");
        break;
    }
  }

  async function handleRevokeConfirm() {
    const session = pendingRevokeSession;
    if (!session || !revokeAction) {
      return;
    }

    const token = getAuthConfig()?.getToken() ?? null;
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      await revokeAction(session, fetch, token);
      toastStore.push({ variant: "success", message: "Session revoked" });
      pendingRevokeSession = null;
      await listData.refetch();
    } catch (error) {
      toastStore.push({
        variant: "error",
        message: error instanceof Error ? error.message : "Failed to revoke session"
      });
      // Rethrow so AlertDialog keeps the dialog open for retry.
      throw error;
    }
  }

  function handleRevokeCancel() {
    pendingRevokeSession = null;
  }
</script>

{#if hasActivated}
  {#if listData.loading && sessions.length === 0}
    <PageLoading presentation="inline" message="Loading sessions..." />
  {:else if listData.error}
    <Callout tone="danger" message={listData.error} announceMode="polite" />
  {:else}
    <DataTable
      {columns}
      {rows}
      rowActions={getRowActions as unknown as (row: TableRow) => TableRowAction[]}
      emptyMessage="No sessions found"
      ariaLabel="User sessions"
      onRowActionSelect={handleRowActionSelect}
    >
      {#snippet cell(column, row, value)}
        {@const session = row.data as UserSessionItem | undefined}
        {#if column.id === "status" && session}
          <PoodlePill tone={getStatusTone(session.status)} appearance="badge" size="sm">{session.status}</PoodlePill>
        {:else if column.id === "ipAddress" && session?.ipAddress}
          <Code inline inlineVariant="plain" typography="inline" size="md" source={session.ipAddress} showCopyButton={false} />
        {:else if column.id === "createdAt" && session}
          <TimeAgo datetime={session.createdAt} tooltipFormat="datetime" short />
        {:else if column.id === "lastUsedAt" && session}
          <TimeAgo datetime={session.lastUsedAt} tooltipFormat="datetime" short />
        {:else}
          {value || "—"}
        {/if}
      {/snippet}
    </DataTable>
  {/if}
{/if}

{#if pendingRevokeSession}
  <AlertDialog
    open={true}
    title="Revoke session"
    description="This signs the session out immediately. This action cannot be undone."
    itemLabel="Session"
    itemValue={pendingRevokeSession.id}
    tone="danger"
    confirmLabel="Revoke"
    cancelLabel="Cancel"
    onConfirm={handleRevokeConfirm}
    onCancel={handleRevokeCancel}
  />
{/if}
