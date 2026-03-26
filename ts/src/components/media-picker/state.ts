import type { MediaSummary } from "../../patterns/index.js";

export type MediaPickerUploadStep =
  | "select"
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

export interface MediaPickerBrowseState {
  browseItems: MediaSummary[];
  browseNextCursor: string | null;
  browseHasMore: boolean;
}

export function createResetBrowseState(): MediaPickerBrowseState {
  return {
    browseItems: [],
    browseNextCursor: null,
    browseHasMore: false
  };
}
