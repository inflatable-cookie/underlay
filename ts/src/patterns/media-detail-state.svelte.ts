interface MediaEditSource<TVisibility extends string = string> {
  title?: string | null;
  originalFilename?: string | null;
  visibility: TVisibility;
}

export interface MediaEditDialogDraft<TVisibility extends string = string> {
  title: string;
  filename: string;
  visibility: TVisibility;
}

export interface MediaEditDialogState {
  open: boolean;
  error: string | null;
  submitting: boolean;
}

export function createMediaEditDialogDraft<TVisibility extends string = string>(
  media: MediaEditSource<TVisibility>
): MediaEditDialogDraft<TVisibility> {
  return {
    title: media.title || "",
    filename: media.originalFilename || "",
    visibility: media.visibility
  };
}

export function createClosedMediaEditDialogState(): MediaEditDialogState {
  return {
    open: false,
    error: null,
    submitting: false
  };
}

export function createMediaVersionDialogStateController<TVersion>() {
  let activateDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let selectedVersion = $state<TVersion | null>(null);

  function requestActivate(version: TVersion) {
    activateDialogOpen = true;
    deleteDialogOpen = false;
    selectedVersion = version;
  }

  function requestDelete(version: TVersion) {
    activateDialogOpen = false;
    deleteDialogOpen = true;
    selectedVersion = version;
  }

  function clear() {
    activateDialogOpen = false;
    deleteDialogOpen = false;
    selectedVersion = null;
  }

  return {
    get activateDialogOpen() {
      return activateDialogOpen;
    },
    set activateDialogOpen(value: boolean) {
      activateDialogOpen = value;
    },
    get deleteDialogOpen() {
      return deleteDialogOpen;
    },
    set deleteDialogOpen(value: boolean) {
      deleteDialogOpen = value;
    },
    get selectedVersion() {
      return selectedVersion;
    },
    set selectedVersion(value: TVersion | null) {
      selectedVersion = value;
    },
    requestActivate,
    requestDelete,
    clear
  };
}
