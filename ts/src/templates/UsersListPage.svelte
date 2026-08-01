<script lang="ts" generics="T extends UsersListUser">
  import { gotoWithContext } from "../client/navigation";
  import type { QueryParams } from "../client/query";
  import { copyToClipboard, useToasts } from "../runtime/feedback";
  import EntityListPage from "./EntityListPage.svelte";
  import {
    DEFAULT_USER_ROLE_OPTIONS,
    DEFAULT_USER_STATUS_OPTIONS,
    getUserRoleTone,
    getUserStatusTone,
    type UsersListLoader,
    type UsersListUser
  } from "./users-list.types";
  import {
    Pill as PoodlePill,
    TimeAgo,
    type TableColumn,
    type TableRow,
    type TableRowAction
  } from "@poodle/svelte";
  import type { PillTone } from "@poodle/svelte";

  interface Props {
    title?: string;
    hideTitle?: boolean;
    subtitle?: string;
    eyebrow?: string;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    backHref?: string;
    backLabel?: string;
    query: QueryParams;
    onQueryChange: (query: QueryParams) => void;
    dataLoader: UsersListLoader<T>;
    roleOptions?: { value: string; label: string }[];
    statusOptions?: { value: string; label: string }[];
    roleTone?: (role: string) => PillTone;
    statusTone?: (status: string) => PillTone;
    usersBaseHref?: string;
    addLabel?: string;
    onAdd?: () => void;
  }

  let {
    title = "Users",
    hideTitle = false,
    subtitle,
    eyebrow,
    headerLevel = 2,
    backHref = "/",
    backLabel = "Back to dashboard",
    query,
    onQueryChange,
    dataLoader,
    roleOptions = DEFAULT_USER_ROLE_OPTIONS,
    statusOptions = DEFAULT_USER_STATUS_OPTIONS,
    roleTone = getUserRoleTone,
    statusTone = getUserStatusTone,
    usersBaseHref = "/users",
    addLabel = "Add user",
    onAdd
  }: Props = $props();

  const toastStore = useToasts();

  const columns: TableColumn[] = [
    { id: "email", label: "Email", width: "2fr" },
    { id: "displayName", label: "Display Name", width: "1.5fr" },
    { id: "role", label: "Role", width: "120px" },
    { id: "status", label: "Status", width: "100px" },
    { id: "createdAt", label: "Created", width: "100px", hideOnMobile: true }
  ];

  const filters = $derived([
    {
      id: "query",
      type: "search" as const,
      label: "Search",
      placeholder: "Search by email or display name..."
    },
    {
      id: "role",
      type: "select" as const,
      label: "Role",
      options: roleOptions
    },
    {
      id: "status",
      type: "select" as const,
      label: "Status",
      options: statusOptions
    },
    {
      id: "sort",
      type: "sort" as const,
      label: "Sort",
      sortFields: [
        { key: "createdAt", label: "Created", defaultDirection: "desc" as const },
        { key: "email", label: "Email" },
        { key: "displayName", label: "Display name" },
        { key: "role", label: "Role" },
        { key: "status", label: "Status" }
      ]
    }
  ]);

  function getRowActions(_row: TableRow<T>): TableRowAction[] {
    return [
      { value: "edit", label: "Edit" },
      { value: "copy-id", label: "Copy ID" },
      { value: "copy-email", label: "Copy Email" }
    ];
  }

  function handleRowActionSelect(row: TableRow<T>, action: TableRowAction) {
    const user = row.data;
    if (!user) return;

    switch (action.value) {
      case "edit":
        void gotoWithContext(`${usersBaseHref}/${user.id}/edit`, {
          label: title,
          href: backHref,
          type: "list"
        });
        break;
      case "copy-id":
        void copyToClipboard(toastStore, user.id, "Copied user ID");
        break;
      case "copy-email":
        void copyToClipboard(toastStore, user.email, "Copied user email");
        break;
    }
  }

  function handleAddUser() {
    if (onAdd) {
      onAdd();
      return;
    }
    void gotoWithContext(`${usersBaseHref}/new`, {
      label: title,
      href: backHref,
      type: "list"
    });
  }
</script>

{#snippet renderCell(column: TableColumn, row: TableRow<T>, value: string)}
  {@const user = row.data}
  {#if column.id === "email" && user}
    <a href={`${usersBaseHref}/${user.id}`} class="underlay-users-list__email-link">{value}</a>
  {:else if column.id === "role" && user}
    <PoodlePill tone={roleTone(user.role)} appearance="badge" size="sm">{user.role}</PoodlePill>
  {:else if column.id === "status" && user}
    <PoodlePill tone={statusTone(user.status)} appearance="badge" size="sm">{user.status}</PoodlePill>
  {:else if column.id === "createdAt" && user}
    <TimeAgo datetime={user.createdAt} tooltipFormat="datetime" short />
  {:else}
    {value || "—"}
  {/if}
{/snippet}

<EntityListPage
  {title}
  {hideTitle}
  {subtitle}
  {eyebrow}
  {headerLevel}
  {backHref}
  {backLabel}
  {dataLoader}
  presentation="table"
  {columns}
  {filters}
  rowActions={getRowActions}
  renderCell={renderCell}
  onRowActionSelect={handleRowActionSelect}
  {query}
  {onQueryChange}
  onAdd={handleAddUser}
  {addLabel}
/>

<style>
  .underlay-users-list__email-link {
    color: inherit;
    text-decoration: none;
    font-weight: 500;
  }

  .underlay-users-list__email-link:hover {
    text-decoration: underline;
  }
</style>
