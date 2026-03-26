<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
  import type { Snippet } from "svelte";

  import type { TotpSetupPayload } from "./types";
  import { sanitizeSvgHtml } from "../../utils/html.js";

  import Card from "../Card.svelte";
  import FormActions from "../FormActions.svelte";

  import TotpInput from "./TotpInput.svelte";

  interface Props {
    issuer?: string;
    accountName?: string;
    secret: string;
    qrSvg?: string | null;
    backupCodes?: string[];
    code?: string;
    error?: string | null;
    confirmLabel?: string;
    loading?: boolean;
    onConfirm?: (payload: TotpSetupPayload) => void;
  }

  let {
    issuer = undefined,
    accountName = undefined,
    secret,
    qrSvg = null,
    backupCodes = [],
    code = $bindable(""),
    error = null,
    confirmLabel = "Confirm",
    loading = false,
    onConfirm,
  }: Props = $props();

  function handleConfirm() {
    onConfirm?.({ code: code.trim() });
  }

  const safeQrSvg = $derived(qrSvg ? sanitizeSvgHtml(qrSvg) : null);
</script>

<Card class="underlay-totp-setup">
  <header class="underlay-totp-setup__header">
    <h3 class="underlay-totp-setup__title">Two-factor authentication</h3>
    {#if issuer || accountName}
      <p class="underlay-totp-setup__subtitle">
        {#if issuer}{issuer}{/if}
        {#if issuer && accountName} — {/if}
        {#if accountName}{accountName}{/if}
      </p>
    {/if}
  </header>

  {#if safeQrSvg}
    <div class="underlay-totp-setup__qr" aria-hidden="true">
      {@html safeQrSvg}
    </div>
  {/if}

  <div class="underlay-totp-setup__secret">
    <div class="underlay-totp-setup__secret-label">Secret</div>
    <code class="underlay-totp-setup__secret-value">{secret}</code>
  </div>

  {#if backupCodes.length > 0}
    <div class="underlay-totp-setup__backup">
      <div class="underlay-totp-setup__backup-label">Backup codes</div>
      <ul class="underlay-totp-setup__backup-list">
        {#each backupCodes as bc (bc)}
          <li><code>{bc}</code></li>
        {/each}
      </ul>
    </div>
  {/if}

  <div class="underlay-totp-setup__confirm">
    {#if error}
      <p class="underlay-totp-setup__error" role="alert" aria-live="polite">{error}</p>
    {/if}

    <TotpInput bind:value={code} disabled={loading} />

    <FormActions align="end">
      <Button
        type="button"
        variant="primary"
        disabled={loading || !code.trim()}
        loading={loading}
        on:click={handleConfirm}
      >
        {confirmLabel}
      </Button>
    </FormActions>
  </div>
</Card>

<style>
  .underlay-totp-setup__header {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, 0.25rem);
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-totp-setup__title {
    margin: 0;
    font-size: var(--underlay-font-size-lg, 1.05rem);
  }

  .underlay-totp-setup__subtitle {
    margin: 0;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
    font-size: var(--underlay-font-size-sm, 0.9rem);
  }

  .underlay-totp-setup__qr {
    display: flex;
    justify-content: center;
    margin: var(--underlay-space-3, 0.75rem) 0;
  }

  .underlay-totp-setup__secret,
  .underlay-totp-setup__backup {
    margin: var(--underlay-space-3, 0.75rem) 0;
  }

  .underlay-totp-setup__secret-label,
  .underlay-totp-setup__backup-label {
    font-weight: 600;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
    margin-bottom: var(--underlay-space-1, 0.25rem);
  }

  .underlay-totp-setup__secret-value {
    display: inline-block;
    padding: 0.3em 0.45em;
    border-radius: var(--underlay-radius-sm, 0.35rem);
    border: 1px solid
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
  }

  .underlay-totp-setup__backup-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-totp-setup__error {
    margin: 0 0 var(--underlay-space-2, 0.5rem);
    color: var(--underlay-color-error, #ef4444);
  }

  .underlay-totp-setup__confirm {
    margin-top: var(--underlay-space-3, 0.75rem);
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
  }
</style>
