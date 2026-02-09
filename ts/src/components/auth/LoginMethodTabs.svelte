<script lang="ts">
  import TabsList from "../TabsList.svelte";
  import TabsTrigger from "../TabsTrigger.svelte";

  type LoginMethod = "password" | "passkey" | "google";

  interface Props {
    methods: LoginMethod[];
    onGoogleLogin?: () => Promise<void>;
  }

  let { methods, onGoogleLogin }: Props = $props();
</script>

<div class="underlay-login-page__tabs">
  <TabsList>
    {#if methods.includes("password")}
      <TabsTrigger value="password">Password</TabsTrigger>
    {/if}
    {#if methods.includes("passkey")}
      <TabsTrigger value="passkey">Passkeys</TabsTrigger>
    {/if}
    {#if methods.includes("google")}
      <TabsTrigger value="google" disabled={!onGoogleLogin}>Google</TabsTrigger>
    {/if}
  </TabsList>
</div>

<style>
  .underlay-login-page__tabs {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }
</style>
