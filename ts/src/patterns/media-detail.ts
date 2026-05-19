import { MediaKind, MediaVersionState, MediaVisibility } from "./media-types/enums";
import type { MediaRendition, MediaVersion } from "./media-types/dto";

export interface MediaActionSessionContext {
  browser: boolean;
  getToken: () => string | null;
  onUnauthenticated: () => void;
}

export interface PreviewableMediaVersion {
  state: string;
  url?: string | null;
  renditions?: Array<Pick<MediaRendition, "url" | "mimeType">> | null;
}

export function resolveMediaActionSession(
  context: MediaActionSessionContext
): { token: string; fetchFn: typeof fetch } | null {
  if (!context.browser) return null;

  const token = context.getToken();
  if (!token) {
    context.onUnauthenticated();
    return null;
  }

  return {
    token,
    fetchFn: fetch
  };
}

export function canGenerateMediaRenditions(
  version: Pick<MediaVersion, "state" | "mimeType"> | null | undefined
): boolean {
  if (!version || version.state !== MediaVersionState.Ready) return false;
  const mimeType = version.mimeType ?? "";
  return ["image/jpeg", "image/png", "image/gif", "image/webp"].includes(mimeType);
}

export function isCurrentMediaVersion(
  currentVersionId: string | null | undefined,
  version: Pick<MediaVersion, "id">
): boolean {
  return currentVersionId === version.id;
}

export function canActivateMediaVersion(
  currentVersionId: string | null | undefined,
  version: Pick<MediaVersion, "id" | "state">
): boolean {
  return !isCurrentMediaVersion(currentVersionId, version) && version.state === MediaVersionState.Ready;
}

export function canDeleteMediaVersion(
  currentVersionId: string | null | undefined,
  version: Pick<MediaVersion, "id">
): boolean {
  return !isCurrentMediaVersion(currentVersionId, version);
}

export function getMediaUrl(
  baseApiUrl: string,
  mediaId: string,
  visibility: string
): string {
  const base = baseApiUrl.replace(/\/$/, "");
  if (visibility === MediaVisibility.Restricted) {
    return `${base}/v1/media/${encodeURIComponent(mediaId)}/download`;
  }
  return `${base}/v1/media/${encodeURIComponent(mediaId)}`;
}

export function canPreviewMedia(kind: string, _mimeType?: string | null): boolean {
  return kind === MediaKind.Image || kind === MediaKind.Pdf;
}

export function isImageMedia(kind: string): boolean {
  return kind === MediaKind.Image;
}

export function isPdfMedia(kind: string): boolean {
  return kind === MediaKind.Pdf;
}

export function getMediaVersionPreviewUrl(version: PreviewableMediaVersion): string | null {
  if (version.url) return version.url;
  const imageRendition = version.renditions?.find(
    (rendition) => rendition.url && rendition.mimeType?.startsWith("image/")
  );
  return imageRendition?.url ?? null;
}

export function canPreviewMediaVersion(
  kind: string,
  version: PreviewableMediaVersion | null | undefined
): boolean {
  if (!version || version.state !== MediaVersionState.Ready) return false;
  if (kind === MediaKind.Image) return Boolean(getMediaVersionPreviewUrl(version));
  if (kind === MediaKind.Pdf) return Boolean(version.url);
  return false;
}
