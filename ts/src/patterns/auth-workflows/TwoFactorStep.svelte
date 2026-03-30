<script lang="ts">
  import { untrack } from "svelte";
  import { Button, Callout, Field, FormActions, Tabs, TextInput, TotpInput } from "@poodle/svelte-primitives";
  import type { TabItem } from "@poodle/svelte-primitives";

  type Method = "totp" | "backup" | "email";

  interface Props {
    hasTotpSetup?: boolean;
    email?: string;
    code?: string;
    loading?: boolean;
    error?: string | null;
    submitLabel?: string;
    backLabel?: string;
    onVerify?: (code: string) => void;
    onRequestEmailCode?: () => void;
    onResendEmailCode?: () => void;
    onBack?: () => void;
  }

  let {
    hasTotpSetup = false,
    email,
    code = $bindable(""),
    loading = false,
    error = null,
    submitLabel = "Verify",
    backLabel = "Use a different account",
    onVerify,
    onRequestEmailCode,
    onResendEmailCode,
    onBack,
  }: Props = $props();

  let activeMethod = $state<Method>(untrack(() => (hasTotpSetup ? "totp" : "email")));
  let emailCodeRequested = $state(untrack(() => !hasTotpSetup));
  let previousMethod = $state<Method>(untrack(() => activeMethod));

  const methodTabs = $derived<TabItem[]>([
    { value: "totp", label: "Authenticator" },
    { value: "backup", label: "Backup Code" },
    { value: "email", label: "Email" },
  ]);

  $effect(() => {
    if (activeMethod !== previousMethod) {
      code = "";
      previousMethod = activeMethod;
    }
  });

  async function handleRequestEmailCode() {
    onRequestEmailCode?.();
    emailCodeRequested = true;
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (code.trim()) {
      onVerify?.(code.trim());
    }
  }

  const totpHint = "Enter the 6-digit code from your authenticator app.";
  const backupHint = "Enter one of your backup codes. Each code can only be used once.";
  const emailHintPending = "We'll send a verification code to your email address.";
  const hasEmail = $derived(typeof email === "string" && email.length > 0);
</script>

{#if hasTotpSetup}
  <div class="underlay-two-factor-step__tabs">
    <Tabs
      value={activeMethod}
      items={methodTabs}
      variant="pill"
      size="sm"
      ariaLabel="Two-factor verification methods"
      on:valueChange={(event) => {
        activeMethod = event.detail.value as Method;
      }}
    />
  </div>

  <form onsubmit={handleSubmit} class="underlay-two-factor-step">
    <p class="underlay-two-factor-step__hint">
      {#if activeMethod === "totp"}
        {totpHint}
      {:else if activeMethod === "backup"}
        {backupHint}
      {:else if emailCodeRequested}
        {#if hasEmail}
          We've sent a verification code to <strong>{email}</strong>.
        {:else}
          We've sent a verification code to your email.
        {/if}
      {:else}
        {emailHintPending}
      {/if}
    </p>

    {#if activeMethod === "totp"}
      <TotpInput
        value={code}
        label="2FA code"
        disabled={loading}
        error={error ?? undefined}
        on:valueChange={(event) => { code = event.detail.value; }}
      />
    {:else if activeMethod === "backup"}
      <Field id="backup-code" label="Backup code" required let:describedBy>
        <TextInput
          id="backup-code"
          value={code}
          describedBy={describedBy}
          placeholder="XXXXX-XXXXX"
          disabled={loading}
          on:valueChange={(event) => { code = event.detail.value; }}
        />
      </Field>
      {#if error}
        <Callout tone="danger" message={error} announceMode="assertive" />
      {/if}
    {:else if emailCodeRequested}
      <TotpInput
        value={code}
        label="Email code"
        disabled={loading}
        error={error ?? undefined}
        on:valueChange={(event) => { code = event.detail.value; }}
      />
    {:else}
      <Callout tone="info" message={emailHintPending} />
    {/if}

    <FormActions align="between">
      {#if onBack}
        <Button type="button" variant="ghost" on:click={onBack} disabled={loading}>
          {backLabel}
        </Button>
      {/if}

      <div class="underlay-two-factor-step__actions">
        {#if activeMethod === "email" && !emailCodeRequested}
          <Button type="button" variant="primary" on:click={handleRequestEmailCode} loading={loading}>
            {loading ? "Sending..." : "Send code"}
          </Button>
        {:else}
          <Button type="submit" variant="primary" loading={loading} disabled={!code.trim()}>
            {loading ? "Verifying..." : submitLabel}
          </Button>
        {/if}
      </div>
    </FormActions>

    {#if activeMethod === "email" && emailCodeRequested && onResendEmailCode}
      <div class="underlay-two-factor-step__resend">
        <span class="underlay-two-factor-step__resend-text">Didn't receive the code?</span>
        <Button type="button" variant="ghost" size="sm" on:click={onResendEmailCode} disabled={loading}>
          Resend code
        </Button>
      </div>
    {/if}
  </form>
{:else}
  <form onsubmit={handleSubmit} class="underlay-two-factor-step">
    <p class="underlay-two-factor-step__hint">
      {#if hasEmail}
        We've sent a verification code to <strong>{email}</strong>.
      {:else}
        We've sent a verification code to your email.
      {/if}
    </p>

    <TotpInput
      value={code}
      label="Email code"
      disabled={loading}
      error={error ?? undefined}
      on:valueChange={(event) => { code = event.detail.value; }}
    />

    <FormActions align="between">
      {#if onBack}
        <Button type="button" variant="ghost" on:click={onBack} disabled={loading}>
          {backLabel}
        </Button>
      {/if}
      <Button type="submit" variant="primary" loading={loading} disabled={!code.trim()}>
        {loading ? "Verifying..." : submitLabel}
      </Button>
    </FormActions>

    {#if onResendEmailCode}
      <div class="underlay-two-factor-step__resend">
        <span class="underlay-two-factor-step__resend-text">Didn't receive the code?</span>
        <Button type="button" variant="ghost" size="sm" on:click={onResendEmailCode} disabled={loading}>
          Resend code
        </Button>
      </div>
    {/if}
  </form>
{/if}

<style>
  .underlay-two-factor-step {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-two-factor-step__tabs {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-2, 0.5rem);
  }

  .underlay-two-factor-step__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-two-factor-step__hint :global(strong) {
    color: var(--underlay-color-text, #e5e7eb);
    font-weight: 500;
  }

  .underlay-two-factor-step__actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  .underlay-two-factor-step__resend {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    justify-content: center;
    padding-top: var(--underlay-space-2, 0.5rem);
    border-top: 1px solid var(--underlay-color-border, rgba(255, 255, 255, 0.1));
  }

  .underlay-two-factor-step__resend-text {
    font-size: var(--underlay-font-size-sm, 0.8rem);
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
