import { storage, type StorageWrapper } from "../storage";
import type { FormAutoSaveOptions, FormDraft } from "./types";

interface DraftControlBase {
  name: string;
  type?: string;
  value?: string;
  checked?: boolean;
  disabled?: boolean;
  multiple?: boolean;
  tagName?: string;
  options?: ArrayLike<{ value: string; selected: boolean }>;
  dispatchEvent?: (event: Event) => boolean;
}

type DraftControl = DraftControlBase & {
  type?: string;
  value: string;
};

export function captureFormDraft(formEl: HTMLFormElement): FormDraft {
  const controlsByName = collectDraftControls(formEl);
  const draft: FormDraft = {};

  for (const [name, controls] of controlsByName.entries()) {
    const first = controls[0];

    if (isCheckboxControl(first)) {
      if (controls.length === 1) {
        draft[name] = {
          kind: "boolean",
          checked: Boolean(first.checked),
        };
        continue;
      }

      draft[name] = {
        kind: "multi",
        values: controls
          .filter((control) => control.checked)
          .map((control) => control.value),
      };
      continue;
    }

    if (isRadioControl(first)) {
      const selected = controls.find((control) => control.checked);
      if (selected) {
        draft[name] = {
          kind: "single",
          value: selected.value,
        };
      }
      continue;
    }

    if (isMultiSelectControl(first)) {
      draft[name] = {
        kind: "multi",
        values: Array.from(first.options ?? [])
          .filter((option) => option.selected)
          .map((option) => option.value),
      };
      continue;
    }

    draft[name] = {
      kind: "single",
      value: first.value ?? "",
    };
  }

  return draft;
}

export function restoreFormDraft(
  formEl: HTMLFormElement,
  draft: FormDraft,
): void {
  const controlsByName = collectDraftControls(formEl);

  for (const [name, entry] of Object.entries(draft)) {
    const controls = controlsByName.get(name);
    if (!controls || controls.length === 0) {
      continue;
    }

    const first = controls[0];

    switch (entry.kind) {
      case "boolean": {
        if (!isCheckboxControl(first) || controls.length !== 1) {
          break;
        }

        if (Boolean(first.checked) !== entry.checked) {
          first.checked = entry.checked;
          dispatchDraftEvents(first);
        }
        break;
      }

      case "multi": {
        if (isMultiSelectControl(first)) {
          const nextValues = new Set(entry.values);
          let changed = false;

          for (const option of Array.from(first.options ?? [])) {
            const shouldSelect = nextValues.has(option.value);
            if (option.selected !== shouldSelect) {
              option.selected = shouldSelect;
              changed = true;
            }
          }

          if (changed) {
            dispatchDraftEvents(first);
          }
          break;
        }

        if (isCheckboxControl(first)) {
          const nextValues = new Set(entry.values);
          let changed = false;

          for (const control of controls) {
            const shouldCheck = nextValues.has(control.value);
            if (Boolean(control.checked) !== shouldCheck) {
              control.checked = shouldCheck;
              changed = true;
              dispatchDraftEvents(control);
            }
          }

          if (changed) {
            dispatchDraftEvents(first);
          }
        }
        break;
      }

      case "single": {
        if (isRadioControl(first)) {
          for (const control of controls) {
            const shouldCheck = control.value === entry.value;
            if (Boolean(control.checked) !== shouldCheck) {
              control.checked = shouldCheck;
              dispatchDraftEvents(control);
            }
          }
          break;
        }

        if ((first.value ?? "") !== entry.value) {
          first.value = entry.value;
          dispatchDraftEvents(first);
        }
        break;
      }
    }
  }
}

export function resolveDraftStorage(
  autoSave?: FormAutoSaveOptions,
): StorageWrapper | null {
  if (!autoSave) {
    return null;
  }

  if (!autoSave.storage || autoSave.storage === "session") {
    return storage.session;
  }

  if (autoSave.storage === "local") {
    return storage.local;
  }

  return autoSave.storage;
}

function isDraftControl(control: unknown): control is DraftControl {
  if (typeof control !== "object" || control === null) {
    return false;
  }

  const entry = control as DraftControlBase;
  const tagName = getControlTagName(entry);
  return (
    typeof entry.name === "string" &&
    (tagName === "input" || tagName === "select" || tagName === "textarea")
  );
}

function getControlTagName(control: DraftControlBase): string {
  return typeof control.tagName === "string"
    ? control.tagName.toLowerCase()
    : "";
}

function getControlType(control: DraftControlBase): string {
  return typeof control.type === "string" ? control.type.toLowerCase() : "";
}

function isFileInput(control: DraftControlBase): boolean {
  return (
    getControlTagName(control) === "input" && getControlType(control) === "file"
  );
}

function isCheckboxControl(control: DraftControlBase): boolean {
  return (
    getControlTagName(control) === "input" &&
    getControlType(control) === "checkbox"
  );
}

function isRadioControl(control: DraftControlBase): boolean {
  return (
    getControlTagName(control) === "input" &&
    getControlType(control) === "radio"
  );
}

function isMultiSelectControl(control: DraftControlBase): boolean {
  return getControlTagName(control) === "select" && control.multiple === true;
}

function createDraftEventsFor(control: DraftControlBase): Event[] {
  const type = getControlType(control);
  if (
    type === "checkbox" ||
    type === "radio" ||
    getControlTagName(control) === "select"
  ) {
    return [new Event("change", { bubbles: true })];
  }

  return [
    new Event("input", { bubbles: true }),
    new Event("change", { bubbles: true }),
  ];
}

function dispatchDraftEvents(control: DraftControlBase): void {
  if (!control.dispatchEvent) {
    return;
  }

  for (const event of createDraftEventsFor(control)) {
    control.dispatchEvent(event);
  }
}

function collectDraftControls(
  formEl: HTMLFormElement,
): Map<string, DraftControl[]> {
  const controls = new Map<string, DraftControl[]>();

  for (const entry of Array.from(formEl.elements ?? [])) {
    if (!isDraftControl(entry)) {
      continue;
    }

    if (!entry.name || isFileInput(entry)) {
      continue;
    }

    const group = controls.get(entry.name) ?? [];
    group.push(entry);
    controls.set(entry.name, group);
  }

  return controls;
}
