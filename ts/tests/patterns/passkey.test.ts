import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { arrayBufferToBase64url } from "../../src/utils/base64url";

const originalNavigator = globalThis.navigator;
const originalPublicKeyCredential = globalThis.PublicKeyCredential;

type PasskeyModule = typeof import("../../src/patterns/passkey.svelte");

async function loadPasskeyModule(): Promise<PasskeyModule> {
  vi.resetModules();
  (globalThis as any).$state = <T>(initial: T) => initial;
  return await import("../../src/patterns/passkey.svelte");
}

function stubPasskeyRuntime(
  overrides: Partial<{
    create: (options?: CredentialCreationOptions) => Promise<Credential | null>;
    get: (options?: CredentialRequestOptions) => Promise<Credential | null>;
    conditional: () => Promise<boolean>;
  }> = {},
): void {
  vi.stubGlobal("navigator", {
    credentials: {
      create: overrides.create ?? vi.fn(),
      get: overrides.get ?? vi.fn(),
    },
  });

  class PublicKeyCredentialStub {}
  (PublicKeyCredentialStub as typeof PublicKeyCredential).isConditionalMediationAvailable =
    overrides.conditional ?? (async () => false);
  vi.stubGlobal("PublicKeyCredential", PublicKeyCredentialStub);
}

describe("patterns/passkey", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("runs the passkey registration ceremony and serializes the result", async () => {
    const mod = await loadPasskeyModule();
    const finish = vi.fn(async () => undefined);
    const challenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);
    const userId = arrayBufferToBase64url(new Uint8Array([4, 5, 6]).buffer);

    stubPasskeyRuntime({
      create: vi.fn(async () => ({
        id: "cred-1",
        rawId: new Uint8Array([7, 8, 9]).buffer,
        type: "public-key",
        response: {
          clientDataJSON: new Uint8Array([10]).buffer,
          attestationObject: new Uint8Array([11]).buffer,
        },
        getClientExtensionResults: () => ({}),
      }) as unknown as PublicKeyCredential),
    });

    const registration = mod.usePasskeyRegistration({
      onStart: async () => ({
        challenge,
        user: {
          id: userId,
          name: "person@example.com",
          displayName: "Person",
        },
      }),
      onFinish: finish,
    });

    await registration.start();

    expect(finish).toHaveBeenCalledWith(
      expect.objectContaining({
        id: "cred-1",
        type: "public-key",
      }),
    );
    expect(registration.error).toBeNull();
  });

  it("maps authentication failures and tracks conditional support", async () => {
    const mod = await loadPasskeyModule();
    const errorHandler = vi.fn();
    const requestChallenge = arrayBufferToBase64url(new Uint8Array([1, 2, 3]).buffer);

    stubPasskeyRuntime({
      conditional: async () => true,
      get: vi.fn(async () => {
        throw new DOMException("The operation was aborted. https://www.w3.org/", "AbortError");
      }),
    });

    const authentication = mod.usePasskeyAuthentication({
      conditional: true,
      onStart: async () => ({ challenge: requestChallenge }),
      onFinish: vi.fn(async () => undefined),
      onError: errorHandler,
    });

    expect(await authentication.refreshConditionalSupport()).toBe(true);
    expect(authentication.conditionalSupported).toBe(true);

    await expect(authentication.start()).rejects.toMatchObject({
      code: "cancelled",
    });

    expect(authentication.error).toMatchObject({
      code: "cancelled",
      message: "The operation was aborted.",
    });
    expect(errorHandler).toHaveBeenCalled();
  });
});

afterEach(() => {
  vi.unstubAllGlobals();
  if (originalNavigator) {
    vi.stubGlobal("navigator", originalNavigator);
  }
  if (originalPublicKeyCredential) {
    vi.stubGlobal("PublicKeyCredential", originalPublicKeyCredential);
  }
});
