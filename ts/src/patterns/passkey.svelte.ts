import {
  assertionToJson,
  credentialCreationToJson,
  mapWebAuthnError,
  supportsConditionalMediation,
  toPublicKeyCreationOptions,
  toPublicKeyRequestOptions,
  type AuthenticationCredentialJson,
  type PasskeyError,
  type RegistrationCredentialJson,
} from "../utils/webauthn";

export interface PasskeyRegistrationOptions {
  onStart: () => Promise<unknown>;
  onFinish: (credential: RegistrationCredentialJson) => Promise<void>;
  onError?: (error: PasskeyError) => void;
}

export interface PasskeyAuthenticationOptions {
  conditional?: boolean;
  onStart: () => Promise<unknown>;
  onFinish: (credential: AuthenticationCredentialJson) => Promise<void>;
  onError?: (error: PasskeyError) => void;
}

export interface PasskeyFlowState {
  readonly loading: boolean;
  readonly error: PasskeyError | null;
  clearError: () => void;
}

export interface PasskeyRegistrationResult extends PasskeyFlowState {
  start: () => Promise<void>;
}

export interface PasskeyAuthenticationResult extends PasskeyFlowState {
  readonly conditionalSupported: boolean;
  refreshConditionalSupport: () => Promise<boolean>;
  start: (options?: { mediation?: CredentialMediationRequirement }) => Promise<void>;
}

function getNavigatorCredentials():
  | {
      create: CredentialsContainer["create"];
      get: CredentialsContainer["get"];
    }
  | null {
  const navigatorLike = globalThis.navigator as Navigator | undefined;
  return navigatorLike?.credentials ?? null;
}

function resolveCredential(
  credential: Credential | null,
  fallbackMessage: string,
): PublicKeyCredential {
  if (
    !credential ||
    typeof credential !== "object" ||
    !("response" in credential) ||
    !("rawId" in credential)
  ) {
    throw new Error(fallbackMessage);
  }

  return credential as PublicKeyCredential;
}

function createPasskeyFlowState(): PasskeyFlowState & {
  setError: (error: PasskeyError | null) => void;
  setLoading: (next: boolean) => void;
} {
  let loading = $state(false);
  let error = $state<PasskeyError | null>(null);

  return {
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    clearError() {
      error = null;
    },
    setError(next) {
      error = next;
    },
    setLoading(next) {
      loading = next;
    },
  };
}

export function usePasskeyRegistration(
  options: PasskeyRegistrationOptions,
): PasskeyRegistrationResult {
  const state = createPasskeyFlowState();

  async function start(): Promise<void> {
    const credentials = getNavigatorCredentials();
    if (!credentials?.create) {
      const error = mapWebAuthnError(new Error("Passkeys are not supported in this browser."));
      state.setError(error);
      options.onError?.(error);
      throw error;
    }

    state.clearError();
    state.setLoading(true);

    try {
      const creationOptions = toPublicKeyCreationOptions(await options.onStart());
      const credential = resolveCredential(
        await credentials.create({ publicKey: creationOptions }),
        "Passkey registration did not return a credential.",
      );
      await options.onFinish(credentialCreationToJson(credential));
    } catch (cause) {
      const error = mapWebAuthnError(cause);
      state.setError(error);
      options.onError?.(error);
      throw error;
    } finally {
      state.setLoading(false);
    }
  }

  return {
    get loading() {
      return state.loading;
    },
    get error() {
      return state.error;
    },
    clearError: state.clearError,
    start,
  };
}

export function usePasskeyAuthentication(
  options: PasskeyAuthenticationOptions,
): PasskeyAuthenticationResult {
  const state = createPasskeyFlowState();
  let conditionalSupported = $state(false);

  async function refreshConditionalSupport(): Promise<boolean> {
    conditionalSupported = options.conditional
      ? await supportsConditionalMediation()
      : false;
    return conditionalSupported;
  }

  async function start(startOptions?: {
    mediation?: CredentialMediationRequirement;
  }): Promise<void> {
    const credentials = getNavigatorCredentials();
    if (!credentials?.get) {
      const error = mapWebAuthnError(new Error("Passkeys are not supported in this browser."));
      state.setError(error);
      options.onError?.(error);
      throw error;
    }

    state.clearError();
    state.setLoading(true);

    try {
      const conditionalAllowed =
        options.conditional && (conditionalSupported || (await refreshConditionalSupport()));
      const publicKey = toPublicKeyRequestOptions(await options.onStart());
      const mediation =
        startOptions?.mediation ?? (conditionalAllowed ? "conditional" : undefined);

      const credential = resolveCredential(
        await credentials.get({ publicKey, mediation } as CredentialRequestOptions),
        "Passkey authentication did not return a credential.",
      );
      await options.onFinish(assertionToJson(credential));
    } catch (cause) {
      const error = mapWebAuthnError(cause);
      state.setError(error);
      options.onError?.(error);
      throw error;
    } finally {
      state.setLoading(false);
    }
  }

  return {
    get loading() {
      return state.loading;
    },
    get error() {
      return state.error;
    },
    get conditionalSupported() {
      return conditionalSupported;
    },
    clearError: state.clearError,
    refreshConditionalSupport,
    start,
  };
}
