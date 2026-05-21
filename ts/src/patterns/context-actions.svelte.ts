import type {
  ContextActionDefinition,
  ContextActionRunState,
  ContextActionSubmitDetail
} from "../templates/contextual-action.types";

export interface ContextActionControllerOptions {
  actions?: ContextActionDefinition[];
}

export interface ContextActionController {
  readonly actions: ContextActionDefinition[];
  readonly open: boolean;
  readonly selectedAction: ContextActionDefinition | null;
  readonly values: Record<string, unknown>;
  readonly selectedProviderKey: string;
  readonly selectedModelId: string;
  readonly runState: ContextActionRunState;
  readonly errorMessage: string | null;
  setActions: (actions: ContextActionDefinition[]) => void;
  setOpen: (open: boolean) => void;
  selectAction: (action: ContextActionDefinition) => void;
  closeAction: () => void;
  setValue: (fieldId: string, value: unknown) => void;
  setSelectedModel: (providerKey: string, modelId: string) => void;
  setRunState: (state: ContextActionRunState) => void;
  setError: (message: string | null) => void;
  buildSubmitDetail: () => ContextActionSubmitDetail | null;
}

export function createContextActionController(
  options: ContextActionControllerOptions = {}
): ContextActionController {
  let actions = $state<ContextActionDefinition[]>(options.actions ?? []);
  let open = $state(false);
  let selectedAction = $state<ContextActionDefinition | null>(null);
  let values = $state<Record<string, unknown>>({});
  let selectedProviderKey = $state("");
  let selectedModelId = $state("");
  let runState = $state<ContextActionRunState>("idle");
  let errorMessage = $state<string | null>(null);

  function initialValues(action: ContextActionDefinition): Record<string, unknown> {
    return Object.fromEntries(
      (action.fields ?? [])
        .filter((field) => field.defaultValue !== undefined)
        .map((field) => [field.id, field.defaultValue])
    );
  }

  function initialModel(action: ContextActionDefinition): { providerKey: string; modelId: string } {
    if (action.defaultModelProviderKey && action.defaultModelId) {
      return {
        providerKey: action.defaultModelProviderKey,
        modelId: action.defaultModelId
      };
    }

    const fallback = action.modelOptions?.find((option) => !option.disabled);
    return {
      providerKey: fallback?.providerKey ?? "",
      modelId: fallback?.modelId ?? ""
    };
  }

  return {
    get actions() {
      return actions;
    },
    get open() {
      return open;
    },
    get selectedAction() {
      return selectedAction;
    },
    get values() {
      return values;
    },
    get selectedProviderKey() {
      return selectedProviderKey;
    },
    get selectedModelId() {
      return selectedModelId;
    },
    get runState() {
      return runState;
    },
    get errorMessage() {
      return errorMessage;
    },
    setActions(nextActions) {
      actions = nextActions;
      if (selectedAction && !nextActions.some((action) => action.id === selectedAction?.id)) {
        selectedAction = null;
      }
    },
    setOpen(nextOpen) {
      open = nextOpen;
    },
    selectAction(action) {
      selectedAction = action;
      values = initialValues(action);
      const initialModelSelection = initialModel(action);
      selectedProviderKey = initialModelSelection.providerKey;
      selectedModelId = initialModelSelection.modelId;
      runState = "idle";
      errorMessage = null;
    },
    closeAction() {
      selectedAction = null;
      values = {};
      selectedProviderKey = "";
      selectedModelId = "";
      runState = "idle";
      errorMessage = null;
    },
    setValue(fieldId, value) {
      values = { ...values, [fieldId]: value };
    },
    setSelectedModel(providerKey, modelId) {
      selectedProviderKey = providerKey;
      selectedModelId = modelId;
    },
    setRunState(state) {
      runState = state;
    },
    setError(message) {
      errorMessage = message;
      runState = message ? "failed" : runState;
    },
    buildSubmitDetail() {
      if (!selectedAction) return null;
      return {
        action: selectedAction,
        values,
        selectedProviderKey: selectedProviderKey || undefined,
        selectedModelId: selectedModelId || undefined
      };
    }
  };
}
