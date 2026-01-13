<script lang="ts">
  import type { Snippet } from "svelte";

  import Card from "../Card.svelte";

  interface Props {
    title?: string;
    className?: string;
    showPassword?: boolean;
    showTwoFactor?: boolean;
    showPasskeys?: boolean;
    showOauth?: boolean;
    showSessions?: boolean;
    showRecovery?: boolean;
    password?: Snippet;
    twoFactor?: Snippet;
    passkeys?: Snippet;
    oauth?: Snippet;
    sessions?: Snippet;
    recovery?: Snippet;
  }

  let {
    title = "Security",
    className = "",
    showPassword = true,
    showTwoFactor = true,
    showPasskeys = true,
    showOauth = true,
    showSessions = true,
    showRecovery = true,
    password,
    twoFactor,
    passkeys,
    oauth,
    sessions,
    recovery,
  }: Props = $props();
</script>

<Card class={`underlay-security-settings ${className}`}>
  <header class="underlay-security-settings__header">
    <h2 class="underlay-security-settings__title">{title}</h2>
  </header>

  <div class="underlay-security-settings__sections">
    {#if showPassword && password}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Password</h3>
        {@render password()}
      </section>
    {/if}

    {#if showTwoFactor && twoFactor}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Two-factor</h3>
        {@render twoFactor()}
      </section>
    {/if}

    {#if showPasskeys && passkeys}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Passkeys</h3>
        {@render passkeys()}
      </section>
    {/if}

    {#if showOauth && oauth}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Connected accounts</h3>
        {@render oauth()}
      </section>
    {/if}

    {#if showSessions && sessions}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Sessions</h3>
        {@render sessions()}
      </section>
    {/if}

    {#if showRecovery && recovery}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Account recovery</h3>
        {@render recovery()}
      </section>
    {/if}
  </div>
</Card>

<style>
  .underlay-security-settings__header {
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-security-settings__title {
    margin: 0;
    font-size: var(--underlay-font-size-xl, 1.25rem);
  }

  .underlay-security-settings__sections {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-security-settings__section {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-security-settings__section-title {
    margin: 0;
    font-size: var(--underlay-font-size-md, 1rem);
  }
</style>
