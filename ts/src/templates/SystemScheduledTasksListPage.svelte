<script lang="ts">
  import { getAuthConfig } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import type { QueryParams } from "../client/query";
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import { default as SystemScheduledTaskListCard } from "./SystemScheduledTaskListCard.svelte";
  import type {
    SystemScheduledTaskAction,
    SystemScheduledTaskListItem,
    SystemScheduledTaskListLoader
  } from "./template.types";
  import type { MenuItem } from "@inflatable-cookie/poodle-svelte";

  interface Props {
    title?: string;
    subtitle?: string;
    backHref?: string;
    backLabel?: string;
    detailHref?: (task: SystemScheduledTaskListItem) => string;
    jobHref?: (jobId: string) => string;
    dataLoader: SystemScheduledTaskListLoader;
    triggerAction?: SystemScheduledTaskAction;
    toggleAction?: SystemScheduledTaskAction;
    navigate?: (href: string) => unknown;
    query: QueryParams;
    onQueryChange: (query: QueryParams) => void;
  }

  let {
    title = "Scheduled Tasks",
    subtitle,
    backHref = "/system",
    backLabel = "Back to system",
    detailHref = defaultDetailHref,
    jobHref = defaultJobHref,
    dataLoader,
    triggerAction,
    toggleAction,
    navigate = defaultNavigate,
    query,
    onQueryChange
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  let refreshRevision = $state(0);

  const filters = [
    {
      id: "enabled",
      type: "select" as const,
      label: "Status",
      options: [
        { value: "All", label: "All tasks" },
        { value: "true", label: "Enabled only" },
        { value: "false", label: "Disabled only" }
      ]
    }
  ];

  function defaultNavigate(href: string): void {
    if (typeof globalThis.location !== "undefined") {
      globalThis.location.assign(href);
    }
  }

  function defaultDetailHref(task: SystemScheduledTaskListItem): string {
    return `/system/scheduled-tasks/${encodeURIComponent(task.id)}`;
  }

  function defaultJobHref(jobId: string): string {
    return `/system/jobs/${encodeURIComponent(jobId)}`;
  }

  function getEnabledFilter(nextQuery: QueryParams) {
    const filter = nextQuery.filters?.find((entry) => entry.field === "enabled");
    if (!filter) return undefined;
    if (filter.value === "") return undefined;
    if (filter.value === "All") return undefined;
    return filter.value === "true" ? true : filter.value === "false" ? false : undefined;
  }

  async function loadTasks(fetch: typeof globalThis.fetch, token: unknown, nextQuery: QueryParams) {
    if (typeof token !== "string") throw new Error("Not authenticated");
    void refreshRevision;

    return await dataLoader(fetch, token, {
      enabled: getEnabledFilter(nextQuery),
      page: nextQuery.page ?? 1,
      limit: nextQuery.limit ?? 30
    });
  }

  function getToken() {
    return authConfig?.getToken?.() ?? null;
  }

  async function runTaskAction(
    task: SystemScheduledTaskListItem,
    action: SystemScheduledTaskAction,
    successMessage: string,
    failureMessage: string
  ) {
    const token = getToken();
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      const result = await action(task, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
      refreshRevision += 1;

      if (result && typeof result.jobId === "string" && result.jobId.length > 0) {
        await navigate(jobHref(result.jobId));
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  function getMenuItems(task: SystemScheduledTaskListItem): MenuItem[] {
    const items: MenuItem[] = [{ value: "view", label: "View details" }];

    if (triggerAction) {
      items.push({ value: "trigger", label: "Trigger now" });
    }

    if (toggleAction) {
      items.push({ value: "toggle", label: task.enabled ? "Disable task" : "Enable task" });
    }

    return items;
  }

  function handleMenuAction(task: SystemScheduledTaskListItem, value: string) {
    if (value === "view") {
      navigate(detailHref(task));
      return;
    }

    if (value === "trigger" && triggerAction) {
      runTaskAction(task, triggerAction, "Job created", "Failed to trigger task");
      return;
    }

    if (value === "toggle" && toggleAction) {
      runTaskAction(task, toggleAction, task.enabled ? "Task disabled" : "Task enabled", "Failed to toggle task");
    }
  }
</script>

{#snippet renderItem(task: SystemScheduledTaskListItem)}
  <SystemScheduledTaskListCard
    {task}
    href={detailHref(task)}
    contextMenuItems={getMenuItems(task)}
    onContextAction={(value) => handleMenuAction(task, value)}
    onClick={() => navigate(detailHref(task))}
  />
{/snippet}

{#key refreshRevision}
  <EntityListPage
    {title}
    {subtitle}
    {backHref}
    {backLabel}
    dataLoader={loadTasks}
    presentation="cards"
    {renderItem}
    {filters}
    {query}
    {onQueryChange}
  />
{/key}
