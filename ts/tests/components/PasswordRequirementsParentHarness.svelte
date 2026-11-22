<script lang="ts">
  import type { PasswordRequirementsPolicy } from "@inflatable-cookie/poodle-svelte";
  import PasswordRequirements from "../../src/patterns/auth-workflows/PasswordRequirements.svelte";

  interface Props {
    onFetch: (observedPassword: string) => Promise<PasswordRequirementsPolicy>;
    reloadKey?: string | number | null;
  }

  let { onFetch, reloadKey = null }: Props = $props();
  let password = $state("");
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
  fetchRequirements={() => onFetch(password)}
/>
