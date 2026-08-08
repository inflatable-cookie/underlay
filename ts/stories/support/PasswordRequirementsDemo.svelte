<script lang="ts">
  import { TextInput } from "@inflatable-cookie/poodle-svelte";
  import PasswordRequirements from "../../src/patterns/auth-workflows/PasswordRequirements.svelte";

  let password = $state("");

  async function fetchRequirements() {
    return {
      minLength: 12,
      requireMixedCase: true,
      requireDigit: true,
      requireSpecial: true,
      minStrengthScore: 3,
      description: "At least 12 characters, mixed case, one number, and one symbol."
    };
  }
</script>

<div class="password-requirements-demo">
  <TextInput
    type="password"
    value={password}
    placeholder="Type a password..."
    oninput={(value) => {
      password = value;
    }}
  />

  <PasswordRequirements {password} {fetchRequirements} />
</div>

<style>
  .password-requirements-demo {
    display: grid;
    gap: 1rem;
    min-width: 22rem;
  }
</style>
