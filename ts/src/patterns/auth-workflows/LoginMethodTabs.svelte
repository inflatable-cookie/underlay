<script lang="ts">
  import { Tabs } from "@poodle/svelte";
  import type { TabItem } from "@poodle/svelte";

  type LoginMethod = "password" | "passkey" | "google";

  interface Props {
    methods: LoginMethod[];
    activeMethod: LoginMethod;
    onSelect: (method: LoginMethod) => void;
    onGoogleLogin?: () => Promise<void>;
  }

  let { methods, activeMethod, onSelect, onGoogleLogin }: Props = $props();

  const items = $derived<TabItem[]>(
    methods.map((method) => ({
      value: method,
      label: method === "password" ? "Password" : method === "passkey" ? "Passkeys" : "Google",
      disabled: method === "google" ? !onGoogleLogin : false,
    }))
  );
</script>

<div class="underlay-login-page__tabs">
  <Tabs
    value={activeMethod}
    items={items}
    variant="pill"
    ariaLabel="Authentication methods"
    on:valueChange={(event) => onSelect(event.detail.value as LoginMethod)}
  />
</div>

<style>
  .underlay-login-page__tabs {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-3, 0.75rem);
    width: 100%;
  }
</style>
