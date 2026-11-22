<script lang="ts">
  import type { PasswordRequirementsPolicy } from "@inflatable-cookie/poodle-svelte";
  import PasswordRequirements from "../../src/patterns/auth-workflows/PasswordRequirements.svelte";

  interface Props {
    onFetch: () => Promise<PasswordRequirementsPolicy>;
    reloadKey?: string | number | null;
  }

  let { onFetch, reloadKey = null }: Props = $props();
  let password = $state("");

  // Deliberately pass a fresh inline arrow function that depends on reactive
  // `password` state (via a getter/wrapper) or parent re-renders to verify
  // PasswordRequirements does not refetch on keystrokes.
  const makeFetcher = (currentPassword: string) => {
    void currentPassword;
    return () => onFetch();
  };
</script>

<label>
  Password
  <input
    type="password"
    value={password}
    oninput={(event) => {
      password = (event.currentTarget as HTMLInputElement).value;
    }}
  />
</label>

<PasswordRequirements
  {password}
  {reloadKey}
  fetchRequirements={makeFetcher(password)}
/>
