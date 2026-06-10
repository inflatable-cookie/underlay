<script lang="ts">
  import { default as EntityListCard } from "./EntityListCard.svelte";
  import type { SystemScheduledTaskListItem } from "./template.types";
  import type { MenuItem } from "@poodle/svelte";

  interface Props {
    task: SystemScheduledTaskListItem;
    href?: string | null;
    contextMenuItems?: MenuItem[];
    onContextAction?: (value: string) => void;
    onClick?: () => void;
  }

  let {
    task,
    href = null,
    contextMenuItems = [],
    onContextAction,
    onClick
  }: Props = $props();

  function formatTaskName(name: string): string {
    return name
      .split("_")
      .map((word, index) => index === 0 ? word.charAt(0).toUpperCase() + word.slice(1) : word)
      .join(" ");
  }

  function formatLastRun(value: string | null | undefined): string {
    if (!value) {
      return "Never run";
    }

    const then = new Date(value).getTime();
    const now = Date.now();
    const diffMs = Math.max(0, now - then);
    const minute = 60_000;
    const hour = 60 * minute;
    const day = 24 * hour;

    if (diffMs < minute) return "just now";
    if (diffMs < hour) return `${Math.floor(diffMs / minute)}m ago`;
    if (diffMs < day) return `${Math.floor(diffMs / hour)}h ago`;
    return `${Math.floor(diffMs / day)}d ago`;
  }
</script>

<EntityListCard
  title={formatTaskName(task.name)}
  subtitle={task.schedule}
  meta={formatLastRun(task.lastCompletedAt)}
  {href}
  notLive={!task.enabled}
  leadingIcon="calendar"
  badges={task.enabled ? [] : [{ label: "disabled", accent: "#64748b", appearance: "subtle", size: "sm" }]}
  {contextMenuItems}
  contextMenuAriaLabel="Scheduled task actions"
  contextMenuTrigger="leading"
  {onContextAction}
  {onClick}
/>
