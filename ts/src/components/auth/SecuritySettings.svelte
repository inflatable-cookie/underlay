<script lang="ts">
  import Card from "../Card.svelte";

  export let title: string = "Security";
  export let className: string = "";

  export let showPassword: boolean = true;
  export let showTwoFactor: boolean = true;
  export let showPasskeys: boolean = true;
  export let showOauth: boolean = true;
  export let showSessions: boolean = true;
  export let showRecovery: boolean = true;
</script>

<Card class={`underlay-security-settings ${className}`}>
  <header class="underlay-security-settings__header">
    <h2 class="underlay-security-settings__title">{title}</h2>
  </header>

  <div class="underlay-security-settings__sections">
    {#if showPassword && $$slots.password}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Password</h3>
        <slot name="password" />
      </section>
    {/if}

    {#if showTwoFactor && $$slots.twoFactor}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Two-factor</h3>
        <slot name="twoFactor" />
      </section>
    {/if}

    {#if showPasskeys && $$slots.passkeys}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Passkeys</h3>
        <slot name="passkeys" />
      </section>
    {/if}

    {#if showOauth && $$slots.oauth}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Connected accounts</h3>
        <slot name="oauth" />
      </section>
    {/if}

    {#if showSessions && $$slots.sessions}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Sessions</h3>
        <slot name="sessions" />
      </section>
    {/if}

    {#if showRecovery && $$slots.recovery}
      <section class="underlay-security-settings__section">
        <h3 class="underlay-security-settings__section-title">Account recovery</h3>
        <slot name="recovery" />
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
