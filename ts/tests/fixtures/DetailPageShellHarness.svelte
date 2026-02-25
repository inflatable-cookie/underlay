<script lang="ts">
  import DetailPageShell from "../../src/patterns/DetailPageShell/DetailPageShell.svelte";

  interface TabConfig {
    value: string;
    label: string;
    count?: number;
    separator?: boolean;
  }

  interface Props {
    title?: string;
    section?: string;
    subtitle?: string;
    className?: string;
    useMeta?: boolean;
    useActions?: boolean;
    useChildren?: boolean;
    useTabs?: boolean;
    tabs?: TabConfig[];
    initialActiveTab?: string;
    tabsHistoryKey?: string;
  }

  let {
    title = "Detail page",
    section = "Entities",
    subtitle = "Entity detail",
    className = "",
    useMeta = false,
    useActions = false,
    useChildren = false,
    useTabs = false,
    tabs = [
      { value: "overview", label: "Overview", count: 2 },
      { value: "audit", label: "Audit", separator: true },
    ],
    initialActiveTab = "",
    tabsHistoryKey = "tab",
  }: Props = $props();

  let activeTab = $state("");

  $effect(() => {
    activeTab = initialActiveTab;
  });
</script>

{#snippet metaSnippet()}
  <div data-testid="detail-shell-meta">Meta block</div>
{/snippet}

{#snippet actionsSnippet()}
  <button type="button" data-testid="detail-shell-action">Action</button>
{/snippet}

{#snippet childrenSnippet()}
  <div data-testid="detail-shell-children">Children content</div>
{/snippet}

{#snippet tabContentSnippet(value: string)}
  <div data-testid={"tab-content-" + value}>Tab content: {value}</div>
{/snippet}

<DetailPageShell
  {title}
  {section}
  {subtitle}
  class={className}
  meta={useMeta ? metaSnippet : undefined}
  actions={useActions ? actionsSnippet : undefined}
  tabs={useTabs ? tabs : undefined}
  bind:activeTab
  {tabsHistoryKey}
  tabContent={useTabs ? tabContentSnippet : undefined}
  children={useChildren ? childrenSnippet : undefined}
/>

<div data-testid="detail-shell-active-tab">{activeTab}</div>
