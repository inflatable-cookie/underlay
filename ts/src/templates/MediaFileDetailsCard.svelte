<script lang="ts">
  import { Card, Code, DetailItem, DetailSection, TimeAgo } from "@poodle/svelte";

  interface MediaVersionSummary {
    byteSize?: number | null;
    mimeType?: string | null;
  }

  interface MediaDetailSummary {
    originalFilename?: string | null;
    createdAt: string;
    updatedAt: string;
    deletedAt?: string | null;
    currentVersion?: MediaVersionSummary | null;
  }

  interface Props {
    media: MediaDetailSummary;
    formatFileSize: (bytes: number | null) => string;
    originalFilenameLabel?: string;
    fileSizeLabel?: string;
    mimeTypeLabel?: string;
    timestampsTitle?: string;
    fileDetailsTitle?: string;
  }

  let {
    media,
    formatFileSize,
    originalFilenameLabel = "Original Filename",
    fileSizeLabel = "File Size",
    mimeTypeLabel = "MIME Type",
    timestampsTitle = "Timestamps",
    fileDetailsTitle = "File Details"
  }: Props = $props();
</script>

<Card>
  <DetailSection title={fileDetailsTitle}>
    <DetailItem label={originalFilenameLabel} value={media.originalFilename?.trim() || "—"} />
    {#if media.currentVersion}
      <DetailItem label={fileSizeLabel} value={formatFileSize(media.currentVersion.byteSize ?? null)} />
      <DetailItem label={mimeTypeLabel}>
        <Code source={media.currentVersion.mimeType ?? "—"} inline />
      </DetailItem>
    {/if}
  </DetailSection>

  <DetailSection title={timestampsTitle}>
    <DetailItem label="Created">
      <TimeAgo datetime={media.createdAt} short />
    </DetailItem>
    <DetailItem label="Last Updated">
      <TimeAgo datetime={media.updatedAt} short />
    </DetailItem>
    {#if media.deletedAt}
      <DetailItem label="Deleted">
        <span class="underlay-media-file-details-card__deleted-date">
          <TimeAgo datetime={media.deletedAt} short />
        </span>
      </DetailItem>
    {/if}
  </DetailSection>
</Card>

<style>
  .underlay-media-file-details-card__deleted-date {
    color: var(--color-danger, #ef4444);
  }
</style>
