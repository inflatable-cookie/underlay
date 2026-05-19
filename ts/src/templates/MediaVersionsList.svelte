<script lang="ts" generics="TVersion extends MediaVersionListItem = MediaVersionListItem">
  import { Code, IconButton, InlineListSection, Pill, TimeAgo } from "@poodle/svelte";
  import type { MediaVersionListItem } from "./template.types";

  interface Props {
    versions: TVersion[];
    onUploadNewVersion: () => void;
    getVersionStateAccent?: (state: TVersion["state"]) => string;
    getVersionStateLabel?: (state: TVersion["state"]) => string;
    canPreviewVersion: (version: TVersion) => boolean;
    onOpenVersionPreview: (version: TVersion) => void;
    formatFileSize: (bytes: number | null) => string;
    isCurrentVersion: (version: TVersion) => boolean;
    canActivateVersion: (version: TVersion) => boolean;
    canDeleteVersion: (version: TVersion) => boolean;
    onRequestActivate: (version: TVersion) => void;
    onRequestDelete: (version: TVersion) => void;
  }

  let {
    versions,
    onUploadNewVersion,
    getVersionStateAccent = defaultVersionStateAccent,
    getVersionStateLabel = defaultVersionStateLabel,
    canPreviewVersion,
    onOpenVersionPreview,
    formatFileSize,
    isCurrentVersion,
    canActivateVersion,
    canDeleteVersion,
    onRequestActivate,
    onRequestDelete
  }: Props = $props();

  function defaultVersionStateLabel(state: string): string {
    return state.replace(/[_-]+/g, " ").replace(/\b\w/g, (letter) => letter.toUpperCase());
  }

  function defaultVersionStateAccent(state: string): string {
    if (state === "ready") return "#22c55e";
    if (state === "failed") return "#ef4444";
    if (state === "pending" || state === "processing") return "#f59e0b";
    return "#64748b";
  }

  function shouldShowVersionState(state: string): boolean {
    return state.toLowerCase() !== "ready";
  }
</script>

<InlineListSection
  title="Versions"
  items={versions}
  emptyMessage="No versions uploaded yet."
>
  {#snippet actions()}
    <IconButton
      type="button"
      variant="primary"
      size="sm"
      icon="plus"
      onClick={onUploadNewVersion}
      ariaLabel="Upload new version"
      tooltip="Upload new version"
    />
  {/snippet}

  {#snippet item(version)}
    {#if canPreviewVersion(version)}
      <button
        type="button"
        class="underlay-media-versions-list__item-content underlay-media-versions-list__item-content--button"
        onclick={() => onOpenVersionPreview(version)}
      >
        <span
          class="underlay-media-versions-list__dot"
          style:--underlay-media-versions-list-accent={getVersionStateAccent(version.state)}
        ></span>
        <span class="underlay-media-versions-list__label-group">
          <span class="underlay-media-versions-list__label">{version.sha256 ?? "No hash"}</span>
          <span class="underlay-media-versions-list__sublabel">
            {formatFileSize(version.byteSize ?? null)} ·
            <Code inline source={version.mimeType ?? "Unknown type"} />
            {#if version.createdAt}
              ·
              <TimeAgo datetime={version.createdAt} short />
            {/if}
          </span>
        </span>
      </button>
    {:else}
      <div class="underlay-media-versions-list__item-content">
        <span
          class="underlay-media-versions-list__dot"
          style:--underlay-media-versions-list-accent={getVersionStateAccent(version.state)}
        ></span>
        <span class="underlay-media-versions-list__label-group">
          <span class="underlay-media-versions-list__label">{version.sha256 ?? "No hash"}</span>
          <span class="underlay-media-versions-list__sublabel">
            {formatFileSize(version.byteSize ?? null)} ·
            <Code inline source={version.mimeType ?? "Unknown type"} />
            {#if version.createdAt}
              ·
              <TimeAgo datetime={version.createdAt} short />
            {/if}
          </span>
        </span>
      </div>
    {/if}

    <div class="underlay-media-versions-list__trailing">
      {#if shouldShowVersionState(version.state)}
        <Pill accent={getVersionStateAccent(version.state)}>
          {getVersionStateLabel(version.state)}
        </Pill>
      {/if}
      {#if isCurrentVersion(version)}
        <Pill accent="#3b82f6">Current</Pill>
      {/if}
    </div>

    <div class="underlay-media-versions-list__actions">
      <IconButton
        type="button"
        variant="secondary"
        size="sm"
        icon="check"
        onClick={() => onRequestActivate(version)}
        disabled={!canActivateVersion(version)}
        ariaLabel="Activate version"
        tooltip="Activate version"
      />
      <IconButton
        type="button"
        variant="secondary"
        tone="danger"
        size="sm"
        icon="trash-2"
        onClick={() => onRequestDelete(version)}
        disabled={!canDeleteVersion(version)}
        ariaLabel="Delete version"
        tooltip="Delete version"
      />
    </div>
  {/snippet}
</InlineListSection>

<style>
  .underlay-media-versions-list__trailing,
  .underlay-media-versions-list__actions {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .underlay-media-versions-list__item-content {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 0.625rem;
  }

  .underlay-media-versions-list__item-content--button {
    padding: 0;
    border: none;
    background: transparent;
    color: inherit;
    text-align: left;
  }

  .underlay-media-versions-list__dot {
    --underlay-media-versions-list-accent: var(--poodle-color-accent-base);
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 999rem;
    background: var(--underlay-media-versions-list-accent);
    flex-shrink: 0;
  }

  .underlay-media-versions-list__label-group {
    min-width: 0;
    display: grid;
    gap: 0.125rem;
  }

  .underlay-media-versions-list__label {
    font-size: 0.9rem;
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-media-versions-list__sublabel {
    font-size: 0.8rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
