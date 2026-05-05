<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    PageHeader,
    Tabs,
    Button,
    AlertDialog
  } from "@poodle/svelte";
  import EntityDetail from "./EntityDetail.svelte";

  // --- Types ---

  interface MetaItemConfig {
    label: string;
    value: string | Snippet;
  }

  interface DetailSectionConfig {
    title: string;
    columns?: 1 | 2 | 3;
    separated?: boolean;
    items: { label: string; value: string | Snippet; emptyText?: string }[];
  }

  interface CustomSectionConfig {
    title: string;
    content: Snippet;
  }

  interface TabConfig {
    id: string;
    label: string;
    count?: number;
    content?: Snippet;
    separator?: boolean;
  }

  interface ActionConfig {
    label: string;
    tone?: "default" | "danger" | "warning";
    handler: () => void;
    confirm?: boolean | {
      title: string;
      description: string;
      confirmLabel?: string;
      cancelLabel?: string;
    };
  }

  interface Props {
    /** Page title (entity name) */
    title: string;
    
    /** Section label (e.g., "Project", "User") */
    section?: string;
    
    /** Back link URL */
    backHref?: string;
    
    /** Back link label */
    backLabel?: string;
    
    /** Banner message (e.g., for warnings) */
    bannerMessage?: string;
    
    /** Banner tone */
    bannerTone?: "warning" | "info" | "danger";
    
    /** Data loading function */
    dataLoader: (fetch: typeof window.fetch, token: string | null) => Promise<T | null>;
    
    /** Metadata items */
    meta?: MetaItemConfig[];
    
    /** Detail sections (for the "details" tab) */
    detailSections?: DetailSectionConfig[];
    
    /** Custom sections (for the "details" tab) */
    customSections?: CustomSectionConfig[];
    
    /** Tabs configuration */
    tabs?: TabConfig[];
    
    /** Page actions */
    actions?: ActionConfig[];
    
  }

  type T = $$Generic;

  // --- Props ---

  let {
    title,
    section,
    backHref,
    backLabel,
    bannerMessage,
    bannerTone = "warning",
    dataLoader,
    meta = [],
    detailSections = [],
    customSections = [],
    tabs = [],
    actions: pageActions = []
  }: Props = $props();

  // --- State ---

  let activeTab = $state("details");
  let showConfirmDialog = $state(false);
  let pendingAction: ActionConfig | null = $state(null);

  // --- Actions ---

  function handleAction(action: ActionConfig) {
    if (action.confirm) {
      pendingAction = action;
      showConfirmDialog = true;
    } else {
      action.handler();
    }
  }

  function toButtonTone(tone: ActionConfig["tone"]): "default" | "danger" | undefined {
    if (tone === "danger") {
      return "danger";
    }
    return tone === "default" ? "default" : undefined;
  }

  function handleConfirm() {
    if (pendingAction) {
      pendingAction.handler();
      pendingAction = null;
    }
    showConfirmDialog = false;
  }

  function handleCancel() {
    pendingAction = null;
    showConfirmDialog = false;
  }

  // Combine detail sections into the "details" tab if provided
  const hasDetailsTab = $derived(detailSections.length > 0 || customSections.length > 0);
  const allTabs = $derived<TabConfig[]>(
    hasDetailsTab
      ? [
          {
            id: "details",
            label: "Details"
          },
          ...tabs
        ]
      : tabs
  );
</script>

<div class="underlay-entity-detail-page">
  <PageHeader
    {title}
    {section}
    backHref={backHref ?? null}
    backLabel={backLabel}
    bannerMessage={bannerMessage}
    bannerTone={bannerTone}
  >
    {#snippet actions()}
      {#each pageActions as action}
        <Button
          variant={action.tone === "danger" ? "ghost" : "secondary"}
          tone={toButtonTone(action.tone)}
          on:click={() => handleAction(action)}
        >
          {action.label}
        </Button>
      {/each}
    {/snippet}
  </PageHeader>

  {#if allTabs.length > 0}
    <Tabs
      value={activeTab}
      items={allTabs.map((tab) => ({
        value: tab.id,
        label: tab.label,
        count: tab.count,
        separator: tab.separator
      }))}
      variant="card"
      size="sm"
      ariaLabel={`${title} sections`}
      on:valueChange={(event) => {
        activeTab = event.detail.value;
      }}
    >
      {#each allTabs as tab}
          {#if tab.id === activeTab}
            {#if tab.id === "details" && (detailSections.length > 0 || customSections.length > 0)}
              <EntityDetail
                {dataLoader}
                {meta}
                sections={detailSections}
                {customSections}
              />
            {:else if tab.content}
              {@render tab.content()}
            {/if}
          {/if}
      {/each}
    </Tabs>
  {:else}
    <EntityDetail
      {dataLoader}
      {meta}
      sections={detailSections}
      {customSections}
    />
  {/if}
</div>

<!-- Action confirmation dialog -->
{#if showConfirmDialog && pendingAction}
  <AlertDialog
    open={true}
    title={typeof pendingAction.confirm === "object" 
      ? pendingAction.confirm.title 
      : `${pendingAction.label}?`}
    description={typeof pendingAction.confirm === "object" 
      ? pendingAction.confirm.description 
      : `Are you sure you want to ${pendingAction.label.toLowerCase()}?`}
    confirmLabel={typeof pendingAction.confirm === "object"
      ? pendingAction.confirm.confirmLabel ?? pendingAction.label
      : pendingAction.label}
    cancelLabel={typeof pendingAction.confirm === "object"
      ? pendingAction.confirm.cancelLabel ?? "Cancel"
      : "Cancel"}
    tone={pendingAction.tone === "danger" ? "danger" : "warning"}
    onConfirm={handleConfirm}
    onCancel={handleCancel}
  />
{/if}

<style>
  .underlay-entity-detail-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
